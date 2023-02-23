use super::memory;
use super::element::Content;
use super::attribute::Attribute;

#[derive(Copy, Clone, Debug)]
pub struct ElementPtr {
    pub ptr: usize
}

pub const NULL: usize = 0x100_000;

impl ElementPtr {

    pub fn null() -> Self {
        Self::new(NULL)
    }

    pub fn new(ptr: usize) -> Self {
        ElementPtr { ptr }
    }

    #[inline]
    pub fn to_string(&self) -> String {
        memory::get(self.ptr).to_string()
    }

    #[inline]
    pub fn get_inner_string(&self) -> String {
        memory::get(self.ptr).get_inner_string()
    }

    #[inline]
    pub fn add_element_ptr(&self, element_ptr: ElementPtr) {
        memory::get_mut(self.ptr).add_element_ptr(element_ptr);
    }

    #[inline]
    pub fn add_contents(&self, contents: Vec<Content>) {
        memory::get_mut(self.ptr).add_contents(contents);
    }

    #[inline]
    pub fn get_attribute(&self, attribute: String) -> Option<String> {
        memory::get_mut(self.ptr).get_attribute(attribute)
    }

    #[inline]
    pub fn get_attributes(&self) -> &Vec<Attribute> {
        &memory::get(self.ptr).attributes
    }

    #[inline]
    pub fn set_attribute(&self, attribute: String, value: String) {
        memory::get_mut(self.ptr).set_attribute(attribute, value);
    }

    #[inline]
    pub fn get_parent(&self) -> Option<ElementPtr> {
        memory::get(self.ptr).get_parent()
    }

    #[inline]
    pub fn delete_child_element(&self, child: ElementPtr) {
        memory::get_mut(self.ptr).delete_child_element(child);
    }

    #[inline]
    pub fn get_children(&self) -> Vec<ElementPtr> {
        memory::get(self.ptr).get_children()
    }

    #[inline]
    pub fn set_parent(&self, parent: ElementPtr) {
        memory::get_mut(self.ptr).set_parent(parent);
    }

    #[inline]
    pub fn set_parent_recursive(&self) {
        memory::get_mut(self.ptr).set_parent_recursive();
    }

    #[inline]
    pub fn has_unique_attributes(&self) -> bool {
        memory::get(self.ptr).has_unique_attributes()
    }

}

impl PartialEq for ElementPtr {
    fn eq(&self, other: &ElementPtr) -> bool {
        self.ptr != super::pointer::NULL
        && self.ptr == other.ptr
    }
}