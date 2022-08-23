use super::{
    attribute::Attribute,
    read_errors, reset_errors
};
use super::memory::{
    self, ElementPtr,
    allocate, get_mut
};
use crate::gstring::set_global_string;
use crate::parse::parse_element;
use crate::utils::into_v16;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Content {
    Element(ElementPtr),
    CharData(String),
    CDSect(String),
    Comment(String),
    Reference(String),
}

impl Content {

    pub fn new_element(element: Element) -> Self {
        Content::Element(memory::allocate(element))
    }

    pub fn new_char_data(char_data: String) -> Self {
        Content::CharData(char_data)
    }

    pub fn new_cd_sect(cd_sect: String) -> Self {
        Content::CDSect(cd_sect)
    }

    pub fn new_comment(comment: String) -> Self {
        Content::Comment(comment)
    }

    pub fn new_reference(reference: String) -> Self {
        Content::Reference(reference)
    }

    pub fn to_string(&self) -> String {

        match self {
            Content::Element(pointer) => memory::get(*pointer).to_string(),
            Content::CharData(char_data) => char_data.clone(),
            Content::CDSect(cd_sect) => format!("<![CDATA[{}]]>", cd_sect),
            Content::Comment(comment) => format!("<!--{}-->", comment),
            Content::Reference(reference) => format!("&{};", reference)
        }

    }

}

#[derive(Debug, Clone)]
pub struct Element {
    pub pointer: ElementPtr,
    parent: Option<ElementPtr>,
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    empty_element: bool,
    pub contents: Vec<Content>,
    pub is_alive: bool,

    /// for HTML
    pub id: Option<String>,

    /// for HTML
    pub classes: Vec<String>
}

impl Element {

    pub fn new(tag_name: String, attributes: Vec<Attribute>, empty_element: bool, contents: Vec<Content>) -> Self {

        let mut id = None;
        let mut classes = vec![];

        for attribute in attributes.iter() {

            if attribute.name == String::from("id") {
                id = Some(attribute.value.clone());
                break;
            }

            else if attribute.name == String::from("class") {
                classes = attribute.value.split(" ").map(|c| c.to_string()).collect();
                break;
            }

        }

        Element {
            pointer: 0,
            parent: None,
            is_alive: true,
            tag_name, attributes, empty_element, contents,
            id, classes
        }
    }

    pub fn from_string(string: String) -> Result<ElementPtr, Vec<String>> {

        let string_v16 = into_v16(&string);
        set_global_string(string_v16.clone());
        reset_errors();
    
        match parse_element(&string_v16, 0) {
            Some((element, _)) => {
                let mut result = element.to_real();
                let result_ptr = allocate(result);
                get_mut(result_ptr).set_parent_recursive();
                return Ok(result_ptr);
            },
            None => {}
        }
    
        let errors = read_errors();
    
        // reset global states
        set_global_string(vec![]);
        reset_errors();
    
        if errors.len() > 0 {
            return Err(errors);
        }
    
        return Err(vec![String::from("No Elements have found!")]);
    
    }

    pub fn add_element(&mut self, mut element: Element) {
        element.parent = Some(self.pointer);
        self.contents.push(Content::new_element(element));
    }

    pub fn add_element_ptr(&mut self, element_ptr: ElementPtr) {
        get_mut(element_ptr).parent = Some(self.pointer);
        self.contents.push(Content::Element(element_ptr));
    }

    pub fn add_char_data(&mut self, char_data: String) {
        self.contents.push(Content::new_char_data(char_data));
    }

    pub fn get_attribute(&self, attribute: String) -> Option<String> {

        for att in self.attributes.iter() {

            if att.name == attribute {
                return Some(att.value.to_string());
            }

        }

        None
    }

    pub fn set_attribute(&mut self, attribute: String, value: String) {

        for att in self.attributes.iter_mut() {

            if att.name == attribute {
                att.value = value;
                return;
            }

        }

        self.attributes.push(Attribute::new(attribute, value));
    }

    pub fn has_unique_attributes(&self) -> bool {
        let mut attribute_names = HashSet::with_capacity(self.attributes.len());

        for attribute in self.attributes.iter() {

            if attribute_names.contains(&attribute.name) {
                return false;
            }

            attribute_names.insert(attribute.name.clone());
        }

        for child in self.get_children() {

            if !child.has_unique_attributes() {
                return false;
            }

        }

        true
    }

    pub fn get_children(&self) -> Vec<&Element> {
        let mut result = Vec::with_capacity(self.contents.len());

        for content in self.contents.iter() {

            match content {
                Content::Element(pointer) => {
                    result.push(memory::get(*pointer));
                },
                _ => {}
            }

        }

        result
    }

    pub fn get_children_mut(&mut self) -> Vec<&mut Element> {
        let mut result = Vec::with_capacity(self.contents.len());

        for content in self.contents.iter() {

            match content {
                Content::Element(pointer) => {
                    result.push(memory::get_mut(*pointer));
                },
                _ => {}
            }

        }

        result
    }

    pub fn get_parent(&self) -> Option<&Element> {

        match self.parent {
            Some(pointer) => Some(memory::get(pointer)),
            None => None
        }

    }

    pub fn get_parent_mut(&mut self) -> Option<&mut Element> {

        match self.parent {
            Some(pointer) => Some(memory::get_mut(pointer)),
            None => None
        }

    }

    // root element는 이거 꼭 호출해서 init해줘야 함.
    pub fn set_parent_recursive(&mut self) {
        let self_pointer = self.pointer;

        for child in self.get_children_mut() {
            child.parent = Some(self_pointer);
            child.set_parent_recursive();
        }

    }

    pub fn to_string(&self) -> String {
        let inside_tag = format!(
            "{}{}{}",
            self.tag_name,
            if self.attributes.len() == 0 {
                ""
            } else {
                " "
            },
            self.attributes.iter().map(
                |att| format!("{}=\"{}\"", att.name, att.value)
            ).collect::<Vec<String>>().join(" ")
        );
        let opening_tag = format!(
            "<{}{}",
            inside_tag,
            if self.empty_element {
                "/>"
            } else {
                ">"
            }
        );
        let closing_tag = if self.empty_element {
            String::new()
        } else {
            format!("</{}>", self.tag_name)
        };

        format!(
            "{}{}{}",
            opening_tag,
            self.contents.iter().map(|c| c.to_string()).collect::<Vec<String>>().concat(),
            closing_tag
        )
    }

}