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
    memory::{self, allocate}
};
use parse::{get_prolog_end_index, parse_element, parse_prolog};
use utils::into_v16;
use gstring::set_global_string;

/// It's global.
/// You can't handle multiple doms at once.
/// It frees all the elements created so far.
pub fn into_dom(document: String) -> Result<(), Vec<String>> {

    memory::init();

    unsafe {
        dom::prolog = None;
    }

    let document = into_v16(&document);
    set_global_string(document.clone());
    reset_errors();

    let mut curr_index = 0;

    match get_prolog_end_index(&document, curr_index) {
        Some(end_index) => {
            unsafe {
                dom::prolog = Some(parse_prolog(&document, curr_index).0);
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
    use crate::node::memory::*;

    #[test]
    fn file_test() {

        let mut f = File::open("test.html").unwrap();
        let mut s = String::new();

        f.read_to_string(&mut s).unwrap();

        crate::into_dom(s).unwrap();
        crate::dom::some_checks().unwrap();

        let mut images = crate::dom::get_elements_by_tag_name(None, "img".to_string());

        if images.len() > 0 {

            for img in images.iter() {

                match img.get_attribute("src".to_string()) {
                    Some(src) => {
                        img.set_attribute("onclick".to_string(), format!("open_modal_img('{}');", src));
                    },
                    _ => {}
                }

            }

            let body = crate::dom::get_elements_by_tag_name(None, "body".to_string())[0];
            let modal_box = "<div id=\"modal-box\"><div id=\"close-button\">Click the image to close.</div><img id=\"modal-img\" onclick=\"close_modal_img();\"/></div><script>/*<![CDATA[*/var modal_box = document.getElementById(\"modal-box\");var modal_img = document.getElementById(\"modal-img\");
function open_modal_img(src) {
    modal_img.src = src;
    modal_box.style.display = \"block\";
}
function close_modal_img() {
    modal_box.style.display = \"none\";
}/*]]>*/</script>".to_string();

            let xx = crate::node::element::Content::from_string(modal_box).unwrap();

            body.add_contents(xx);
            //println!("!lib.rs <<{}>> {}\n{}\n\n", body.ptr, crate::node::memory::get(body.ptr).contents.len(), body.to_string());
        }

        let mut f = File::create("test copy.html").unwrap();
        f.write_all(crate::dom::to_string().as_bytes()).unwrap();
    }

}