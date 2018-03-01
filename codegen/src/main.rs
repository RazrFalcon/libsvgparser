// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// We don't use cargo build script, since this data will be changed rarely.
// There is no point in regenerating it each time, only if you want to save a few KiB.

#[macro_use]
extern crate derive_error;
extern crate phf_codegen;
extern crate itertools;

use itertools::Itertools;

use std::fs;
use std::io::{self, Read, Write};
use std::str;

#[derive(Debug, Error)]
enum Error {
    Io(io::Error),
    Utf8(str::Utf8Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(e) = gen() {
        println!("{:?}", e);
        std::process::exit(1);
    }
}

fn gen() -> Result<()> {
    gen_file(
        "spec/elements.txt",
        "ElementId",
        "ELEMENTS",
        "List of all SVG elements.",
        "../src/element_id.rs"
    )?;

    gen_file(
        "spec/attributes.txt",
        "AttributeId",
        "ATTRIBUTES",
        "List of all SVG attributes.",
        "../src/attribute_id.rs"
    )?;

    gen_file(
        "spec/values.txt",
        "ValueId",
        "VALUES",
        "List of values for presentation attributes.",
        "../src/value_id.rs"
    )?;

    gen_colors()?;

    Ok(())
}

fn gen_file(
    spec_path: &str,
    enum_name: &str,
    map_name: &str,
    doc: &str,
    out_path: &str,
) -> Result<()> {
    let mut spec = String::new();
    fs::File::open(spec_path)?.read_to_string(&mut spec)?;

    let names: Vec<&str> = spec.split('\n').filter(|s| !s.is_empty()).collect();

    let joned_names = names.iter().map(|n| to_enum_name(n)).join(",\n    ");

    let joned_names2 = names.iter()
                            .map(|n| format!("{}::{} => \"{}\"", enum_name, to_enum_name(n), n))
                            .join(",\n            ");


    let mut map = phf_codegen::Map::new();
    for name in &names {
        map.entry(*name, &format!("{}::{}", enum_name, to_enum_name(name)));
    }

    let mut map_data = Vec::new();
    map.build(&mut map_data)?;
    let map_data = str::from_utf8(&map_data)?;


    let f = &mut fs::File::create(out_path)?;

    write_header(f)?;

    writeln!(f, "use std::fmt;\n")?;

    writeln!(f, "/// {}", doc)?;
    writeln!(f, "#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Hash)]")?;
    writeln!(f, "#[allow(missing_docs)]")?;
    writeln!(f, "pub enum {} {{", enum_name)?;
    writeln!(f, "    {}", joned_names)?;
    writeln!(f, "}}\n")?;

    writeln!(f, "static {}: ::phf::Map<&'static str, {}> = {};\n", map_name, enum_name, map_data)?;

    writeln!(f, "impl {} {{", enum_name)?;
    writeln!(f, "    /// Converts name into id.")?;
    writeln!(f, "    pub fn from_name(text: &str) -> Option<{}> {{", enum_name)?;
    writeln!(f, "        {}.get(text).cloned()", map_name)?;
    writeln!(f, "    }}")?;
    writeln!(f, "")?;
    writeln!(f, "    /// Converts id into name.")?;
    writeln!(f, "    pub fn name(&self) -> &str {{")?;
    writeln!(f, "        match *self {{")?;
    writeln!(f, "            {}", joned_names2)?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl fmt::Debug for {} {{", enum_name)?;
    writeln!(f, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(f, "        write!(f, \"{{}}\", self.name())")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}\n")?;

    writeln!(f, "impl fmt::Display for {} {{", enum_name)?;
    writeln!(f, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(f, "        write!(f, \"{{}}\", self.name())")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn gen_colors() -> Result<()> {
    let map_name = "COLORS";
    let struct_name = "Color";

    let mut spec = String::new();
    fs::File::open("spec/colors.txt")?.read_to_string(&mut spec)?;

    let mut map = phf_codegen::Map::new();
    for (line1, line2) in spec.split('\n').filter(|s| !s.is_empty()).tuples() {
        let rgb: Vec<&str> = line2.split(',').collect();
        map.entry(line1, &format!("{}{{ red: {}, green: {}, blue: {} }}",
                                  struct_name, rgb[0], rgb[1], rgb[2]));
    }

    let mut map_data = Vec::new();
    map.build(&mut map_data)?;
    let map_data = str::from_utf8(&map_data)?;


    let f = &mut fs::File::create("../src/colors.rs")?;

    write_header(f)?;

    writeln!(f, "use {};\n", struct_name)?;

    writeln!(f, "static {}: ::phf::Map<&'static str, {}> = {};\n", map_name, struct_name, map_data)?;

    writeln!(f, "pub fn rgb_color_from_name(text: &str) -> Option<{}> {{", struct_name)?;
    writeln!(f, "    {}.get(text).cloned()", map_name)?;
    writeln!(f, "}}")?;

    Ok(())
}

// some-string -> SomeString
// some_string -> SomeString
// some:string -> SomeString
// 100 -> N100
fn to_enum_name(name: &str) -> String {
    let mut change_case = false;
    let mut s = String::with_capacity(name.len());
    for (idx, c) in name.chars().enumerate() {
        if idx == 0 {
            if c.is_digit(10) {
                s.push('N');
                s.push(c);
            } else {
                s.push(c.to_uppercase().next().unwrap());
            }

            continue;
        }

        if c == '-' || c == '_' || c == ':' {
            change_case = true;
            continue;
        }

        if change_case {
            s.push(c.to_uppercase().next().unwrap());
            change_case = false;
        } else {
            s.push(c);
        }
    }

    s
}

fn write_header(f: &mut fs::File) -> Result<()> {
    writeln!(f,
       "// Copyright 2018 Evgeniy Reizner\n\
        //\n\
        // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n\
        // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license\n\
        // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your\n\
        // option. This file may not be copied, modified, or distributed\n\
        // except according to those terms.\n\
        \n\
        // This file is autogenerated. Do not edit it!\n\
    ")?;

    Ok(())
}
