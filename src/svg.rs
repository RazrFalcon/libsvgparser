// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing SVG structure.

use std::fmt;
use std::str;

use super::{Stream, Error};

/// ElementEnd token.
#[derive(Debug,PartialEq,Clone)]
pub enum ElementEnd<'a> {
    /// Indicates `>`
    Open,
    /// Indicates `</name>`
    Close(&'a [u8]),
    /// Indicates `/>`
    Empty,
}

/// SVG token.
#[derive(PartialEq)]
pub enum Token<'a> {
    /// Tuple contains tag name of the element.
    ElementStart(&'a [u8]),
    /// Tuple contains the type of enclosing tag.
    ElementEnd(ElementEnd<'a>),
    /// Tuple contains attribute name and value.
    Attribute(&'a [u8], Stream<'a>),
    /// Tuple contains a text object.
    Text(Stream<'a>),
    /// Tuple contains CDATA object without `<![CDATA[` and `]]>`.
    Cdata(Stream<'a>),
    /// Tuple contains whitespace object. It will contain only ` \n\t\r`.
    Whitespace(&'a [u8]),
    /// Tuple contains comment object without `<!--` and `-->`.
    Comment(&'a [u8]),
    /// Tuple contains a title of empty DOCTYPE.
    DtdEmpty(&'a [u8]),
    /// Tuple contains a title of DOCTYPE.
    DtdStart(&'a [u8]),
    /// Tuple contains name and value of ENTITY.
    Entity(&'a [u8], Stream<'a>),
    /// Tuple indicates DOCTYPE end.
    DtdEnd,
    /// Tuple contains declaration object without `<?` and `?>`.
    Declaration(&'a [u8]),
    /// The end of the stream.
    EndOfStream,
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::ElementStart(s) =>
                write!(f, "ElementStart({})", u8_to_str!(s)),
            Token::ElementEnd(ref e) => {
                let c = match *e {
                    ElementEnd::Open => ">",
                    ElementEnd::Close(_) => "</",
                    ElementEnd::Empty => "/>",
                };
                write!(f, "ElementEnd({})", c)
            }
            Token::Attribute(k, ref v) =>
                write!(f, "Attribute({}, {:?})", u8_to_str!(k), v),
            Token::Text(ref s) =>
                write!(f, "Text({:?})", s),
            Token::Cdata(ref s) =>
                write!(f, "CDATA({:?})", s),
            Token::Whitespace(s) =>
                write!(f, "Whitespace({})", u8_to_str!(s)),
            Token::Comment(s) =>
                write!(f, "Comment({})", u8_to_str!(s)),
            Token::DtdEmpty(s) =>
                write!(f, "DtdEmpty({})", u8_to_str!(s)),
            Token::DtdStart(s) =>
                write!(f, "DtdStart({})", u8_to_str!(s)),
            Token::Entity(k, ref v) =>
                write!(f, "ENTITY({}, {:?})", u8_to_str!(k), v),
            Token::DtdEnd =>
                write!(f, "DtdEnd"),
            Token::Declaration(s) =>
                write!(f, "Declaration({})", u8_to_str!(s)),
            Token::EndOfStream =>
                write!(f, "EndOfStream"),
        }
    }
}

enum State {
    AtStart,
    Unknown,
    Dtd,
    Attributes,
    Finished,
}

/// Tokenizer for SVG structure.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    state: State,
    depth: u32,
}

impl<'a> Tokenizer<'a> {
    /// Constructs a new `Tokenizer`.
    pub fn new(text: &[u8]) -> Tokenizer {
        Tokenizer {
            stream: Stream::new(text),
            state: State::AtStart,
            depth: 0,
        }
    }

    /// Extracts next SVG node from the stream.
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - Only ENTITY objects are extracted from DOCTYPE. Library will print a warning to stdout.
    /// - The parser doesn't check file encoding, assuming that it's UTF-8.
    pub fn parse_next(&mut self) -> Result<Token<'a>, Error> {
        match self.state {
            State::Unknown => {
                if self.stream.at_end() {
                    self.state = State::Finished;
                    return Ok(Token::EndOfStream);
                }

                if self.stream.starts_with(b"<?") {
                    self.parse_declaration()
                } else if self.stream.starts_with(b"<!--") {
                    self.parse_comment()
                } else if self.stream.starts_with(b"<![") {
                    self.parse_cdata()
                } else if self.stream.starts_with(b"<!DOCTYPE") {
                    self.parse_dtd()
                } else if self.stream.starts_with(b"</") {
                    try!(self.stream.advance(2)); // </
                    let text = try!(self.stream.read_to(b'>'));
                    try!(self.stream.advance(1)); // >

                    if self.depth == 0 {
                        // Error will occur on the next closing tag after invalid,
                        // because we only checking depth and not a closing tag names.
                        return Err(Error::UnexpectedClosingTag(self.stream.gen_error_pos()));
                    }

                    self.depth -= 1;

                    Ok(Token::ElementEnd(ElementEnd::Close(text)))
                } else if self.stream.is_char_eq_raw(b'<') {
                    self.depth += 1;
                    self.parse_element()
                } else if self.depth > 0 {
                    let start = self.stream.pos();

                    while !self.stream.at_end() {
                        if !self.stream.is_space_raw() {
                            break;
                        }
                        try!(self.stream.advance(1));
                    }

                    if try!(self.stream.is_char_eq(b'<')) {
                        let text = self.stream.slice_region_raw(start, self.stream.pos());
                        Ok(Token::Whitespace(text))
                    } else {
                        let b = self.stream.pos() - start;
                        try!(self.stream.back(b));
                        let end = self.stream.pos() + try!(self.stream.len_to(b'<'));
                        let substream = Stream::sub_stream(&self.stream, self.stream.pos(), end);
                        self.stream.advance_raw(substream.left());

                        Ok(Token::Text(substream))
                    }
                } else if try!(self.stream.is_space()) {
                    // ignore spaces outside the root element
                    assert!(self.depth == 0);
                    self.stream.skip_spaces();
                    self.parse_next()
                } else {
                    Err(Error::InvalidSvgToken(self.stream.gen_error_pos()))
                }
            }
            State::Dtd => {
                self.parse_entity()
            }
            State::Attributes => {
                self.parse_attribute()
            }
            State::AtStart => {
                if self.stream.at_end() {
                    self.state = State::Finished;
                    return Ok(Token::EndOfStream);
                }

                // skip byte order
                if try!(self.stream.is_char_eq(0xEF)) {
                    try!(self.stream.advance(3)); // EF BB BF
                }

                self.state = State::Unknown;
                self.parse_next()
            }
            State::Finished => {
                Ok(Token::EndOfStream)
            }
        }
    }

    fn parse_declaration(&mut self) -> Result<Token<'a>, Error> {
        let l = try!(self.stream.len_to(b'>'));
        try!(self.stream.advance(6)); // '<?xml '
        let s = self.stream.read_raw(l - 7);
        try!(self.stream.advance(2)); // '?>'

        Ok(Token::Declaration(s))
    }

    fn parse_comment(&mut self) -> Result<Token<'a>, Error> {
        try!(self.stream.advance(4)); // skip <!--
        let comment_start_pos = self.stream.pos();

        // read all until -->
        loop {
            try!(self.stream.jump_to(b'>'));
            if try!(self.stream.char_at(-1)) == b'-' && try!(self.stream.char_at(-2)) == b'-' {
                break;
            }
            try!(self.stream.advance(1));
        }

        // save data between <!-- and -->
        let comment_end_pos = self.stream.pos() - 2;
        let s = self.stream.slice_region_raw(comment_start_pos, comment_end_pos);
        try!(self.stream.advance(1));

        Ok(Token::Comment(s))
    }

    fn parse_cdata(&mut self) -> Result<Token<'a>, Error> {
        try!(self.stream.advance(9)); // skip <![CDATA[
        let start_pos = self.stream.pos();

        loop {
            try!(self.stream.jump_to(b']'));
            if self.stream.starts_with(b"]]>") {
                break;
            }
            try!(self.stream.advance(1));
        }

        // go back to CDATA start to properly init substream.
        let end = self.stream.pos();
        self.stream.set_pos_raw(start_pos);

        let substream = Stream::sub_stream(&self.stream, self.stream.pos(), end);

        // go to end of CDATA again
        self.stream.set_pos_raw(end);
        try!(self.stream.advance(3));

        Ok(Token::Cdata(substream))
    }

    fn parse_dtd(&mut self) -> Result<Token<'a>, Error> {
        // if first occurred char is '[' - than DTD has content
        // if first occurred char is '>' - than DTD is empty

        try!(self.stream.advance(10)); // '<!DOCTYPE '
        let start = self.stream.pos();

        let l = self.stream.slice_tail().into_iter().position(|x| *x == b'[' || *x == b'>');
        match l {
            Some(l) => try!(self.stream.advance(l)),
            None => return Err(self.stream.gen_end_of_stream_error()),
        }

        if try!(self.stream.is_char_eq(b'>')) {
            // empty DOCTYPE
            let text = self.stream.slice_region_raw(start, self.stream.pos());
            try!(self.stream.advance(1));
            Ok(Token::DtdEmpty(text))
        } else {
            // [
            self.state = State::Dtd;

            // skip space at the end
            let text = self.stream.slice_region_raw(start, self.stream.pos() - 1);
            try!(self.stream.advance(1)); // [
            self.stream.skip_spaces();

            Ok(Token::DtdStart(text))
        }
    }

    fn parse_entity(&mut self) -> Result<Token<'a>, Error> {
        if self.stream.starts_with(b"<!ENTITY") {
            try!(self.stream.advance(9)); // '<!ENTITY '

            let key = try!(self.stream.read_to(b' '));

            self.stream.skip_spaces();
            try!(self.stream.consume_char(b'"'));

            let value_len = try!(self.stream.len_to(b'"'));

            let substream = Stream::sub_stream(&self.stream, self.stream.pos(),
                                               self.stream.pos() + value_len);

            self.stream.advance_raw(value_len);

            try!(self.stream.consume_char(b'"'));
            self.stream.skip_spaces();
            try!(self.stream.consume_char(b'>'));
            self.stream.skip_spaces();

            Ok(Token::Entity(key, substream))
        } else if self.stream.starts_with(b"]>") {
            try!(self.stream.advance(2)); // ]>
            self.state = State::Unknown;

            Ok(Token::DtdEnd)
        } else {
            // skip unsupported elements

            let l = try!(self.stream.len_to(b'>')) + 1;
            println!("Warning: Unsupported DOCTYPE object: '{}'.",
                u8_to_str!(self.stream.slice_next_raw(l)));
            self.stream.advance_raw(l);

            self.stream.skip_spaces();
            self.parse_next()
        }
    }

    fn parse_element(&mut self) -> Result<Token<'a>, Error> {
        try!(self.stream.advance(1)); // <

        let start_pos = self.stream.pos();

        while    !self.stream.at_end()
              && !self.stream.is_space_raw()
              && !self.stream.is_char_eq_raw(b'/')
              && !self.stream.is_char_eq_raw(b'>') {
            try!(self.stream.advance(1));
        }

        // check that element has tag name
        if start_pos == self.stream.pos() {
            return Err(Error::ElementWithoutTagName(self.stream.gen_error_pos()));
        }

        let tag_name = self.stream.slice_region_raw(start_pos, self.stream.pos());
        self.stream.skip_spaces();
        self.state = State::Attributes;

        Ok(Token::ElementStart(tag_name))
    }

    fn parse_attribute(&mut self) -> Result<Token<'a>, Error> {
        if try!(self.stream.is_char_eq(b'/')) {
            self.depth -= 1;
            try!(self.stream.advance(2));
            self.state = State::Unknown;
            return Ok(Token::ElementEnd(ElementEnd::Empty));
        }

        if try!(self.stream.is_char_eq(b'>')) {
            self.stream.advance_raw(1);
            self.state = State::Unknown;

            return Ok(Token::ElementEnd(ElementEnd::Open));
        }

        let key = try!(self.stream.read_to_trimmed(b'='));

        try!(self.stream.advance(1)); // =
        self.stream.skip_spaces();

        if !(try!(self.stream.is_char_eq(b'"')) || try!(self.stream.is_char_eq(b'\''))) {
            return Err(Error::InvalidChar {
                current: self.stream.curr_char_raw() as char,
                expected: '"',
                pos: self.stream.gen_error_pos(),
            });
        }

        let quote = try!(self.stream.curr_char());
        try!(self.stream.advance(1)); // quote

        let end = self.stream.pos() + try!(self.stream.len_to(quote));
        let substream = Stream::sub_stream(&self.stream, self.stream.pos(), end);

        self.stream.advance_raw(substream.left());
        try!(self.stream.advance(1)); // quote

        self.stream.skip_spaces();

        Ok(Token::Attribute(key, substream))
    }
}
