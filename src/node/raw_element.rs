use super::element::{Element, Content};
use super::memory::allocate;
use super::attribute::Attribute;
use crate::gstring::GString;

pub enum RawContent {
    Element(RawElement),
    CharData(GString),
    CDSect(GString),
    Comment(GString),
    Reference(GString),
}

impl RawContent {

    pub fn to_real(&self) -> Content {
        match self {
            RawContent::Element(e) => Content::Element(allocate(e.to_real())),
            RawContent::CharData(c) => Content::CharData(c.to_string()),
            RawContent::CDSect(c) => Content::CDSect(c.to_string()),
            RawContent::Comment(c) => Content::Comment(c.to_string()),
            RawContent::Reference(r) => Content::Reference(r.to_string()),
        }
    }

}

pub struct RawElement {
    tag_name: GString,
    attributes: Vec<(GString, GString)>,  // Vec<(name, value)>
    empty_element: bool,
    contents: Vec<Box<RawContent>>
}

impl RawElement {

    pub fn new(tag_name: GString, attributes: Vec<(GString, GString)>, empty_element: bool, contents: Vec<Box<RawContent>>) -> Self {
        RawElement {
            tag_name, attributes, empty_element, contents
        }
    }

    pub fn to_real(&self) -> Element {
        Element::new(
            self.tag_name.to_string(),
            self.attributes.iter().map(
                |(name, value)|
                Attribute::new(name.to_string(), value.to_string())
            ).collect(),
            self.empty_element,
            self.contents.iter().map(|content| content.to_real()).collect()
        )
    }

}