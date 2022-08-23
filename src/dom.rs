use crate::node::{
    memory::ELEMENTS,
    element::Element
};
use std::collections::HashSet;

pub fn get_all_elements() -> Vec<&'static Element> {
    unsafe {
        ELEMENTS.iter().filter(|e| e.is_alive).collect()
    }
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_element_by_id(elements: Option<Vec<&mut Element>>, id: String) -> Option<&mut Element> {

    match elements {
        None => {
            unsafe {
                for element in ELEMENTS.iter_mut() {

                    if !element.is_alive || element.id.is_none() {
                        continue;
                    }

                    if element.id.as_ref().unwrap() == &id {
                        return Some(element);
                    }

                }
            }
        }
        Some(mut elements) => {

            for element in elements.into_iter() {

                if element.id.is_some() && element.id.as_ref().unwrap() == &id {
                    return Some(element);
                }

            }

        }
    }

    None
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_ids(elements: Option<Vec<&mut Element>>) -> Vec<String> {

    match elements {
        None => unsafe {
            ELEMENTS.iter().filter_map(|e|
                if !e.is_alive || e.id.is_none() {
                    None
                } else {
                    e.id.clone()
                }
            ).collect()
        }
        Some(elements) => elements.iter().filter_map(|e| e.id.clone()).collect()
    }

}

/// if `elements` is None, it searches the entire DOM.
pub fn get_elements_by_tag_name(elements: Option<Vec<&mut Element>>, tag_name: String) -> Vec<&mut Element> {

    match elements {
        None => unsafe {
            ELEMENTS.iter_mut().filter(|e| e.is_alive && e.tag_name == tag_name).collect()
        },
        Some(elements) => elements.into_iter().filter(|e| e.tag_name == tag_name).collect()
    }

}

/// if `elements` is None, it searches the entire DOM.
pub fn get_elements_by_class_name(elements: Option<Vec<&mut Element>>, class_name: String) -> Vec<&mut Element> {

    match elements {
        None => unsafe {
            ELEMENTS.iter_mut().filter(|e| e.is_alive && e.classes.contains(&class_name)).collect()
        },
        Some(elements) => elements.into_iter().filter(|e| e.classes.contains(&class_name)).collect()
    }

}

pub fn get_root() -> &'static Element {

    unsafe {
        let mut curr_element = &ELEMENTS[0];

        for element in ELEMENTS.iter() {

            if element.is_alive {
                curr_element = element;
                break;
            }

        }

        while let Some(parent) = curr_element.get_parent() {
            curr_element = parent;
        }

        curr_element
    }

}

/// It checks whether
/// - all the tags are closed properly
/// - all the ids are unique
/// - all the elements have unique attributes
pub fn some_checks() -> Result<(), String> {
    let ids = get_ids(None);
    let mut id_set = HashSet::with_capacity(ids.len());

    for id in ids.iter() {

        if id_set.contains(id) {
            return Err(format!("ID#{} appears multiple times!", id));
        }

        id_set.insert(id.clone());
    }

    let root = get_root();

    if !root.has_unique_attributes() {
        return Err(String::from("Some elements don't have unique attributes!"));
    }

    Ok(())
}