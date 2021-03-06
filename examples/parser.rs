extern crate svgparser;
extern crate stderrlog;

use std::{env, fs, str};
use std::io::Read;

use svgparser::{
    svg,
    style,
    xmlparser,
    AttributeId,
    AttributeValue,
    ElementId,
    StreamError,
};

use xmlparser::{
    FromSpan,
    StrSpan,
    TextUnescape,
    XmlSpace,
};

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    // Get a file path from the args.
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: parser img.svg");
        return;
    }

    // Read a file into the buffer.
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // Parse an SVG.
    if let Err(e) = parse(&text) {
        println!("{}", e.to_string());
    }
}

// Helper macro for pretty-printing.
macro_rules! print_indent {
    ($msg:expr, $depth:expr) => ({
        write_indent($depth);
        println!($msg);
    });

    ($fmt:expr, $depth:expr, $($arg:tt)*) => ({
        write_indent($depth);
        println!($fmt, $($arg)*);
    });
}

fn write_indent(depth: usize) {
    for _ in 0..(depth * 4) {
        print!(" ");
    }
}

fn parse(text: &str) -> Result<(), xmlparser::Error> {
    // Control XML nodes depth.
    let mut depth = 0;

    let mut curr_tag_name = None;

    // Begin parsing.

    // Loop through tokens.
    for token in svg::Tokenizer::from_str(text) {
        match token? {
            svg::Token::ElementStart(tag_name) => {
                print_indent!("Element start: {:?}", depth, tag_name);
                curr_tag_name = Some(tag_name.local);
            }
            svg::Token::Attribute(name, value) => {
                match name.local {
                    svg::Name::Xml(name) => {
                        print_indent!("Non-SVG attribute: {} = '{}'",
                                      depth + 1, name, value);
                    }
                    svg::Name::Svg(aid) => {
                        if let Some(svg::Name::Svg(eid)) = curr_tag_name {
                            parse_svg_attribute(eid, name.prefix, aid, value, depth + 1).unwrap();
                        }
                    }
                }
            }
            svg::Token::ElementEnd(end) => {
                match end {
                    svg::ElementEnd::Open => {
                        depth += 1;
                    }
                    svg::ElementEnd::Close(_) => {
                        depth -= 1;
                    }
                    svg::ElementEnd::Empty => {}
                }

                print_indent!("Element end: {:?}", depth, end);
            }
            svg::Token::Text(text) => {
                // 'text' contain text node content as is.
                // Basically everything between > and <.
                //
                // Token::Whitespace will not be emitted inside Token::Text.
                //
                // Use 'TextUnescape' to convert text entity references,
                // remove unneeded spaces and other.

                print_indent!("Text node: '{}'", depth,
                              TextUnescape::unescape(text.to_str(), XmlSpace::Default));
            }
            svg::Token::Cdata(cdata) => {
                // CDATA usually used inside the 'style' element and contain CSS,
                // but svgparser doesn't include CSS parser, so you have to use anyone you like.

                print_indent!("CDATA node: '{}'", depth + 1, cdata.to_str());
            }
            svg::Token::Whitespaces(_) => {
                // We usually don't care about whitespaces.
            }
            svg::Token::Comment(comment) => {
                println!("Comment node: '{}'", comment);
            }
            svg::Token::EntityDeclaration(name, value) => {
                // svgparser supports only 'ENTITY'.
                // Any other DTD node will be ignored.

                println!("Entity declaration: '{}' = '{}'", name, value.to_str());
            }
            svg::Token::Declaration(version, encoding, standalone) => {
                println!("Declaration node: version={} encoding={:?} standalone={:?}",
                         version, encoding, standalone);
            }
            svg::Token::ProcessingInstruction(target, content) => {
                println!("Processing Instruction node: target={}, content={:?}",
                         target, content);
            }
        }
    }

    Ok(())
}

fn parse_svg_attribute(
    eid: ElementId,
    prefix: &str,
    aid: AttributeId,
    value: StrSpan,
    depth: usize,
) -> Result<(), StreamError> {
    // SVG attributes parsing should be done 'manually'.
    // svgparser doesn't parse attributes by default because it can be
    // very expensive (in a case of paths).
    // So you can decide for yourself what to do with attributes.

    // We need ElementId for attribute parsing.
    // See 'from_span' documentation for details.
    let av = AttributeValue::from_span(eid, prefix, aid, value)?;
    match av {
        AttributeValue::Path(tokenizer) => {
            print_indent!("Path:", depth);

            // By the SVG spec, any invalid data occurred in the path should
            // stop parsing of this path, but not the whole document.
            for segment in tokenizer {
                print_indent!("{:?}", depth + 1, segment)
            }
        }
        AttributeValue::Points(tokenizer) => {
            print_indent!("Points:", depth);

            // By the SVG spec, any invalid data occurred in the `points` should
            // stop parsing of this attribute, but not the whole document.
            for point in tokenizer {
                print_indent!("{:?}", depth + 1, point)
            }
        }
        AttributeValue::Style(tokenizer) => {
            print_indent!("Style:", depth);

            for token in tokenizer {
                match token? {
                    style::Token::XmlAttribute(name, value) => {
                        print_indent!("Non-SVG attribute: {} = '{}'", depth + 1, name, value);
                    }
                    style::Token::SvgAttribute(aid, value) => {
                        parse_svg_attribute(eid, "", aid, value, depth + 1)?;
                    }
                    style::Token::EntityRef(name) => {
                        print_indent!("Entity reference: {}", depth + 1, name)
                    }
                }
            }
        }
        AttributeValue::Transform(tokenizer) => {
            print_indent!("Transform:", depth);

            for ts in tokenizer {
                print_indent!("{:?}", depth + 1, ts?)
            }
        }
        _ => {
            print_indent!("SVG attribute: {:?} = {:?}", depth, aid, av);
        }
    }

    // Note that 'class' attribute should be parsed manually if needed.

    Ok(())
}
