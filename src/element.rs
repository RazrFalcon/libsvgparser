// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is autogenerated. Do not edit it!

use std::fmt;

/// List of all SVG elements.
#[derive(Copy,Clone,Eq,PartialEq,PartialOrd,Ord,Hash)]
#[allow(missing_docs)]
pub enum ElementId {
    A,
    AltGlyph,
    AltGlyphDef,
    AltGlyphItem,
    Animate,
    AnimateColor,
    AnimateMotion,
    AnimateTransform,
    Circle,
    ClipPath,
    ColorProfile,
    Cursor,
    Defs,
    Desc,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    Font,
    FontFace,
    FontFaceFormat,
    FontFaceName,
    FontFaceSrc,
    FontFaceUri,
    ForeignObject,
    G,
    Glyph,
    GlyphRef,
    Hkern,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Metadata,
    MissingGlyph,
    Mpath,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Script,
    Set,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tref,
    Tspan,
    Use,
    View,
    Vkern,
}

static ELEMENTS: ::phf::Map<&'static str, ElementId> = ::phf::Map {
    key: 8958141709656110593,
    disps: ::phf::Slice::Static(&[
        (0, 1),
        (10, 8),
        (0, 58),
        (0, 30),
        (13, 14),
        (0, 41),
        (1, 34),
        (0, 29),
        (0, 27),
        (2, 37),
        (2, 28),
        (0, 5),
        (20, 46),
        (0, 0),
        (0, 3),
        (0, 0),
    ]),
    entries: ::phf::Slice::Static(&[
        ("g", ElementId::G),
        ("missing-glyph", ElementId::MissingGlyph),
        ("font-face", ElementId::FontFace),
        ("font-face-uri", ElementId::FontFaceUri),
        ("switch", ElementId::Switch),
        ("font-face-name", ElementId::FontFaceName),
        ("feMerge", ElementId::FeMerge),
        ("glyph", ElementId::Glyph),
        ("circle", ElementId::Circle),
        ("feFuncB", ElementId::FeFuncB),
        ("animateColor", ElementId::AnimateColor),
        ("textPath", ElementId::TextPath),
        ("animateTransform", ElementId::AnimateTransform),
        ("altGlyph", ElementId::AltGlyph),
        ("feColorMatrix", ElementId::FeColorMatrix),
        ("font-face-format", ElementId::FontFaceFormat),
        ("tspan", ElementId::Tspan),
        ("feDisplacementMap", ElementId::FeDisplacementMap),
        ("polyline", ElementId::Polyline),
        ("feComponentTransfer", ElementId::FeComponentTransfer),
        ("font", ElementId::Font),
        ("polygon", ElementId::Polygon),
        ("linearGradient", ElementId::LinearGradient),
        ("feFuncR", ElementId::FeFuncR),
        ("title", ElementId::Title),
        ("mask", ElementId::Mask),
        ("line", ElementId::Line),
        ("rect", ElementId::Rect),
        ("feMorphology", ElementId::FeMorphology),
        ("hkern", ElementId::Hkern),
        ("fePointLight", ElementId::FePointLight),
        ("feFuncA", ElementId::FeFuncA),
        ("feSpotLight", ElementId::FeSpotLight),
        ("metadata", ElementId::Metadata),
        ("feBlend", ElementId::FeBlend),
        ("feDistantLight", ElementId::FeDistantLight),
        ("altGlyphDef", ElementId::AltGlyphDef),
        ("filter", ElementId::Filter),
        ("mpath", ElementId::Mpath),
        ("glyphRef", ElementId::GlyphRef),
        ("feTile", ElementId::FeTile),
        ("feConvolveMatrix", ElementId::FeConvolveMatrix),
        ("altGlyphItem", ElementId::AltGlyphItem),
        ("foreignObject", ElementId::ForeignObject),
        ("feSpecularLighting", ElementId::FeSpecularLighting),
        ("animate", ElementId::Animate),
        ("symbol", ElementId::Symbol),
        ("svg", ElementId::Svg),
        ("feComposite", ElementId::FeComposite),
        ("defs", ElementId::Defs),
        ("image", ElementId::Image),
        ("clipPath", ElementId::ClipPath),
        ("desc", ElementId::Desc),
        ("ellipse", ElementId::Ellipse),
        ("feMergeNode", ElementId::FeMergeNode),
        ("feFuncG", ElementId::FeFuncG),
        ("view", ElementId::View),
        ("text", ElementId::Text),
        ("feTurbulence", ElementId::FeTurbulence),
        ("feOffset", ElementId::FeOffset),
        ("feDiffuseLighting", ElementId::FeDiffuseLighting),
        ("feGaussianBlur", ElementId::FeGaussianBlur),
        ("vkern", ElementId::Vkern),
        ("set", ElementId::Set),
        ("script", ElementId::Script),
        ("pattern", ElementId::Pattern),
        ("a", ElementId::A),
        ("cursor", ElementId::Cursor),
        ("tref", ElementId::Tref),
        ("feFlood", ElementId::FeFlood),
        ("feImage", ElementId::FeImage),
        ("path", ElementId::Path),
        ("radialGradient", ElementId::RadialGradient),
        ("use", ElementId::Use),
        ("marker", ElementId::Marker),
        ("stop", ElementId::Stop),
        ("color-profile", ElementId::ColorProfile),
        ("animateMotion", ElementId::AnimateMotion),
        ("font-face-src", ElementId::FontFaceSrc),
        ("style", ElementId::Style),
    ]),
};

impl ElementId {
    /// Converts name into id.
    pub fn from_name(text: &str) -> Option<ElementId> {
        ELEMENTS.get(text).cloned()
    }

    /// Converts id into name.
    pub fn name(&self) -> &str {
        match *self {
            ElementId::A => "a",
            ElementId::AltGlyph => "altGlyph",
            ElementId::AltGlyphDef => "altGlyphDef",
            ElementId::AltGlyphItem => "altGlyphItem",
            ElementId::Animate => "animate",
            ElementId::AnimateColor => "animateColor",
            ElementId::AnimateMotion => "animateMotion",
            ElementId::AnimateTransform => "animateTransform",
            ElementId::Circle => "circle",
            ElementId::ClipPath => "clipPath",
            ElementId::ColorProfile => "color-profile",
            ElementId::Cursor => "cursor",
            ElementId::Defs => "defs",
            ElementId::Desc => "desc",
            ElementId::Ellipse => "ellipse",
            ElementId::FeBlend => "feBlend",
            ElementId::FeColorMatrix => "feColorMatrix",
            ElementId::FeComponentTransfer => "feComponentTransfer",
            ElementId::FeComposite => "feComposite",
            ElementId::FeConvolveMatrix => "feConvolveMatrix",
            ElementId::FeDiffuseLighting => "feDiffuseLighting",
            ElementId::FeDisplacementMap => "feDisplacementMap",
            ElementId::FeDistantLight => "feDistantLight",
            ElementId::FeFlood => "feFlood",
            ElementId::FeFuncA => "feFuncA",
            ElementId::FeFuncB => "feFuncB",
            ElementId::FeFuncG => "feFuncG",
            ElementId::FeFuncR => "feFuncR",
            ElementId::FeGaussianBlur => "feGaussianBlur",
            ElementId::FeImage => "feImage",
            ElementId::FeMerge => "feMerge",
            ElementId::FeMergeNode => "feMergeNode",
            ElementId::FeMorphology => "feMorphology",
            ElementId::FeOffset => "feOffset",
            ElementId::FePointLight => "fePointLight",
            ElementId::FeSpecularLighting => "feSpecularLighting",
            ElementId::FeSpotLight => "feSpotLight",
            ElementId::FeTile => "feTile",
            ElementId::FeTurbulence => "feTurbulence",
            ElementId::Filter => "filter",
            ElementId::Font => "font",
            ElementId::FontFace => "font-face",
            ElementId::FontFaceFormat => "font-face-format",
            ElementId::FontFaceName => "font-face-name",
            ElementId::FontFaceSrc => "font-face-src",
            ElementId::FontFaceUri => "font-face-uri",
            ElementId::ForeignObject => "foreignObject",
            ElementId::G => "g",
            ElementId::Glyph => "glyph",
            ElementId::GlyphRef => "glyphRef",
            ElementId::Hkern => "hkern",
            ElementId::Image => "image",
            ElementId::Line => "line",
            ElementId::LinearGradient => "linearGradient",
            ElementId::Marker => "marker",
            ElementId::Mask => "mask",
            ElementId::Metadata => "metadata",
            ElementId::MissingGlyph => "missing-glyph",
            ElementId::Mpath => "mpath",
            ElementId::Path => "path",
            ElementId::Pattern => "pattern",
            ElementId::Polygon => "polygon",
            ElementId::Polyline => "polyline",
            ElementId::RadialGradient => "radialGradient",
            ElementId::Rect => "rect",
            ElementId::Script => "script",
            ElementId::Set => "set",
            ElementId::Stop => "stop",
            ElementId::Style => "style",
            ElementId::Svg => "svg",
            ElementId::Switch => "switch",
            ElementId::Symbol => "symbol",
            ElementId::Text => "text",
            ElementId::TextPath => "textPath",
            ElementId::Title => "title",
            ElementId::Tref => "tref",
            ElementId::Tspan => "tspan",
            ElementId::Use => "use",
            ElementId::View => "view",
            ElementId::Vkern => "vkern",
        }
    }
}

impl fmt::Debug for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
