use super::{
    attribute::Attribute,
    read_errors, reset_errors
};
use super::memory::{
    self,
    allocate, get_mut
};
use super::pointer::ElementPtr;
use crate::gstring::set_global_string;
use crate::parse::{parse_content, parse_element};
use crate::utils::into_v16;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn from_string(string: String) -> Result<Vec<Content>, Vec<String>> {
        let string_v16 = into_v16(&string);
        set_global_string(string_v16.clone());
        reset_errors();

        let mut curr_index = 0;
        let mut result = vec![];

        while curr_index < string_v16.len() {

            match parse_content(&string_v16, curr_index) {
                Some((content, last_index)) => {
                    let real_content = content.to_real();

                    match &real_content {
                        Content::Element(ptr) => {
                            ptr.set_parent_recursive();
                        }
                        _ => {}
                    }

                    result.push(real_content);
                    curr_index = last_index + 1;
                },
                None => {
                    return Err(vec![String::from("failed to parse an XML string...")]);
                }
            }

        }

        if result.len() > 0 {
            return Ok(result);
        }

        let errors = read_errors();

        // reset global states
        set_global_string(vec![]);
        reset_errors();

        if errors.len() > 0 {
            return Err(errors);
        }

        return Err(vec![String::from("No Contents have been found!")]);
    }

    pub fn to_string(&self) -> String {
        match self {
            Content::Element(pointer) => pointer.to_string(),
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

    pub fn new(tag_name: String, attributes: Vec<Attribute>, empty_element: bool, contents: Vec<Content>) -> ElementPtr {
        let mut id = None;
        let mut classes = vec![];
        let mut attributes_without_id_and_classes = Vec::with_capacity(attributes.len());

        for attribute in attributes.into_iter() {

            if attribute.name == "id".to_string() {
                id = Some(attribute.value.clone());
            }

            else if attribute.name == "class".to_string() {
                classes = attribute.value.split(" ").map(|c| c.to_string()).collect();
            }

            else {
                attributes_without_id_and_classes.push(attribute);
            }

        }

        let result = Element {
            pointer: ElementPtr::null(),
            parent: None,
            is_alive: true,
            tag_name, empty_element, contents,
            attributes: attributes_without_id_and_classes,
            id, classes
        };

        allocate(result)
    }

    pub fn from_string(string: String) -> Result<ElementPtr, Vec<String>> {

        let string_v16 = into_v16(&string);
        set_global_string(string_v16.clone());
        reset_errors();

        match parse_element(&string_v16, 0) {
            Some((element, _)) => {
                let result = element.to_real();
                result.set_parent_recursive();
                return Ok(result);
            },
            None => {
                return Err(vec![String::from("Failed to parse an element!")]);
            }
        }

        let errors = read_errors();

        // reset global states
        set_global_string(vec![]);
        reset_errors();

        if errors.len() > 0 {
            return Err(errors);
        }

        return Err(vec![String::from("No Elements have been found!")]);
    }

    pub fn add_contents(&mut self, contents: Vec<Content>) {

        for content in contents.into_iter() {
            match content {
                Content::Element(ptr) => {
                    self.add_element_ptr(ptr);
                },
                _ => {
                    self.contents.push(content);
                }
            }
        }

    }

    // it does nothing if `element_ptr` is not a child of `self`
    pub fn delete_child_element(&mut self, element_ptr: ElementPtr) {
        let deletion_indexes = self.contents.iter().enumerate().filter(
            |(_, content)| content == &&Content::Element(element_ptr)
        ).map(
            |(ind, _)| ind
        ).collect::<Vec<usize>>();

        let deletion_index = if deletion_indexes.len() == 0 {
            return;
        } else {
            deletion_indexes[0]
        };

        self.contents.remove(deletion_index);
    }

    pub fn add_element_ptr(&mut self, element_ptr: ElementPtr) {
        element_ptr.set_parent(self.pointer);
        self.contents.push(Content::Element(element_ptr));
    }

    pub fn add_char_data(&mut self, char_data: String) {
        self.contents.push(Content::new_char_data(char_data));
    }

    pub fn get_attribute(&self, attribute: String) -> Option<String> {

        if attribute == "id".to_string() {
            return self.id.clone();
        }

        else if attribute == "class".to_string() {

            return if self.classes.len() > 0 {
                Some(self.classes.join(" "))
            } else {
                None
            };
        }

        for att in self.attributes.iter() {

            if att.name == attribute {
                return Some(att.value.to_string());
            }

        }

        None
    }

    // TODO: add_class, remove_class, toggle_class

    pub fn set_attribute(&mut self, attribute: String, value: String) {

        if attribute == "id".to_string() {
            self.id = Some(value);
            return;
        }

        else if attribute == "class".to_string() {
            self.classes = value.split(" ").map(|c| c.to_string()).collect();
            return;
        }

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

    pub fn get_parent(&self) -> Option<ElementPtr> {

        match self.parent {
            Some(pointer) => Some(pointer),
            None => None
        }

    }

    pub fn set_parent(&mut self, parent: ElementPtr) {
        self.parent = Some(parent);
    }

    pub fn get_children(&self) -> Vec<ElementPtr> {
        let mut result = Vec::with_capacity(self.contents.len());

        for content in self.contents.iter() {

            match content {
                Content::Element(pointer) => {
                    result.push(*pointer);
                },
                _ => {}
            }

        }

        result
    }

    // it has to be called when initializing the root node
    pub fn set_parent_recursive(&mut self) {

        for child in self.get_children() {
            child.set_parent(self.pointer);
            child.set_parent_recursive();
        }

    }

    pub fn to_string(&self) -> String {
        let inside_tag = format!(
            "{}{}{}{}{}",
            self.tag_name,
            if self.attributes.len() == 0 {
                ""
            } else {
                " "
            },
            self.attributes.iter().map(
                |att| format!("{}=\"{}\"", att.name, att.value)
            ).collect::<Vec<String>>().join(" "),
            match &self.id {
                Some(id) => format!(" id=\"{}\"", id),
                _ => String::new()
            },
            if self.classes.len() == 0 {
                String::new()
            } else {
                format!(" class=\"{}\"", self.classes.join(" "))
            }
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
        let contents = self.get_inner_string();

        format!(
            "{}{}{}",
            opening_tag,
            contents,
            closing_tag
        )
    }

    #[inline]
    pub fn get_inner_string(&self) -> String {
        self.contents.iter().map(|c| c.to_string()).collect::<Vec<String>>().concat()
    }

}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        self.pointer.ptr != super::pointer::NULL
        && self.pointer.ptr == other.pointer.ptr
    }
}