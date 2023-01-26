mod predicate;
mod parse;
mod utils;
mod gstring;
mod node;
pub mod dom;

#[cfg(test)]
mod testbench;

pub use node::element::{Content, Element};
pub use node::pointer::ElementPtr;
pub use node::prolog::Prolog;

use node::{
    reset_errors,
    read_errors,
    memory
};
use parse::{get_prolog_end_index, parse_element, parse_prolog};
use utils::into_v16;
use gstring::set_global_string;
use std::collections::HashMap;

/// It's global.
/// You can't handle multiple doms at once.
/// It frees all the elements created before.
pub fn into_dom(document: String) -> Result<(), Vec<String>> {
    memory::init();

    unsafe {
        dom::PROLOG = None;
        dom::TAGS_BY_NAME = Some(HashMap::new());
        dom::TAGS_BY_ID = Some(HashMap::new());
        dom::TAGS_BY_CLASS = Some(HashMap::new());
    }

    let document = into_v16(&document);
    set_global_string(document.clone());
    reset_errors();

    let mut curr_index = 0;

    match get_prolog_end_index(&document, curr_index) {
        Some(end_index) => {
            unsafe {
                dom::PROLOG = Some(parse_prolog(&document, curr_index).0);
            }
            curr_index = end_index + 1;
        },
        None => {}
    }

    match parse_element(&document, curr_index) {
        Some((element, _)) => {
            let result = element.to_real();
            result.set_parent_recursive();
            return Ok(());
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

    return Err(vec![String::from("Nothing has been parsed!")]);
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn file_test() {

        let mut f = File::open("test.html").unwrap();
        let mut s = String::new();

        f.read_to_string(&mut s).unwrap();

        crate::into_dom(s).unwrap();
        crate::dom::some_checks().unwrap();
    }

}