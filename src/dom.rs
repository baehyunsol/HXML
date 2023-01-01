use crate::node::{
    element::Element,
    memory::{ELEMENTS, self},
    pointer::ElementPtr,
    prolog::Prolog
};
use std::collections::HashSet;

pub static mut prolog: Option<Prolog> = None;

pub fn get_all_elements() -> Vec<ElementPtr> {
    unsafe {
        ELEMENTS.iter().filter(|e| e.is_alive).map(|e| e.pointer).collect()
    }
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_element_by_id(elements: Option<Vec<ElementPtr>>, id: String) -> Option<ElementPtr> {

    match elements {
        None => {
            unsafe {
                for element in ELEMENTS.iter_mut() {

                    if !element.is_alive || element.id.is_none() {
                        continue;
                    }

                    if element.id.as_ref().unwrap() == &id {
                        return Some(element.pointer);
                    }

                }
            }
        }
        Some(elements) => {

            for element in elements.into_iter() {
                let el = memory::get(element.ptr);

                if el.id.is_some() && el.id.as_ref().unwrap() == &id {
                    return Some(el.pointer);
                }

            }

        }
    }

    None
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_ids(elements: Option<Vec<ElementPtr>>) -> Vec<String> {

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
        Some(elements) => elements.into_iter().filter_map(|e| memory::get(e.ptr).id.clone()).collect()
    }

}

pub fn delete(element: ElementPtr) {

    match element.get_parent() {
        Some(p) => {
            p.delete_child_element(element);
        }
        _ => {}
    }

    memory::delete(element);
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_elements_by_tag_name(elements: Option<Vec<ElementPtr>>, tag_name: String) -> Vec<ElementPtr> {

    match elements {
        None => unsafe {
            ELEMENTS.iter().filter(|e| e.is_alive && e.tag_name == tag_name).map(|e| e.pointer).collect()
        },
        Some(elements) => elements.into_iter().filter(|e| memory::get(e.ptr).tag_name == tag_name).collect()
    }

}

/// if `elements` is None, it searches the entire DOM.
/// It returns the first element with the given tag_name, if exists.
pub fn get_element_by_tag_name(elements: Option<Vec<ElementPtr>>, tag_name: String) -> Option<ElementPtr> {

    match elements {
        None => unsafe {

            for element in ELEMENTS.iter() {

                if element.is_alive && element.tag_name == tag_name {
                    return Some(element.pointer);
                }

            }

        },
        Some(elements) => {

            for element in elements.iter() {

                if memory::get(element.ptr).is_alive && memory::get(element.ptr).tag_name == tag_name {
                    return Some(*element);
                }
            
            }

        }
    }

    None
}

/// if `elements` is None, it searches the entire DOM.
pub fn get_elements_by_class_name(elements: Option<Vec<ElementPtr>>, class_name: String) -> Vec<ElementPtr> {

    match elements {
        None => unsafe {
            ELEMENTS.iter_mut().filter(|e| e.is_alive && e.classes.contains(&class_name)).map(|e| e.pointer).collect()
        },
        Some(elements) => elements.into_iter().filter(|e| memory::get(e.ptr).classes.contains(&class_name)).collect()
    }

}

pub fn get_root() -> ElementPtr {

    unsafe {
        let mut curr_element = &ELEMENTS[0];

        for element in ELEMENTS.iter() {

            if element.is_alive {
                curr_element = element;
                break;
            }

        }

        let mut curr_element = curr_element.pointer;

        while let Some(parent) = curr_element.get_parent() {
            curr_element = parent;
        }

        curr_element
    }

}

pub fn to_string() -> String {

    let prolog_text = unsafe {
        match &prolog {
            Some(p) => p.to_string(),
            None => String::new()
        }
    };
    let ee = get_root();
    let element_text = ee.to_string();
    //let element_text = get_root().to_string();

    format!(
        "{}{}",
        prolog_text,
        element_text
    )
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