mod predicate;
mod parse;
mod utils;
mod gstring;
mod node;
pub mod dom;

#[cfg(test)]
mod testbench;

pub use node::element::Element;

use node::{
    reset_errors,
    read_errors,
    memory::{self, allocate}
};
use parse::{get_prolog_end_index, parse_element};
use utils::into_v16;
use gstring::set_global_string;

/// It's global.
/// You can't handle multiple doms at once.
/// It frees all the elements created so far.
pub fn into_dom(document: String) -> Result<(), Vec<String>> {

    memory::init();

    let document = into_v16(&document);
    set_global_string(document.clone());
    reset_errors();

    let mut curr_index = 0;

    match get_prolog_end_index(&document, curr_index) {
        Some(ind) => { curr_index = ind + 1; },
        None => {}
    }

    match parse_element(&document, curr_index) {
        Some((element, _)) => {
            let result = element.to_real();
            let result_ptr = allocate(result);
            memory::get_mut(result_ptr).set_parent_recursive();
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

            for img in images.iter_mut() {

                match img.get_attribute("src".to_string()) {
                    Some(src) => {
                        img.set_attribute("onclick".to_string(), format!("open_modal_img('{}');", src));
                    },
                    _ => {}
                }

            }

            let mut body = &mut crate::dom::get_elements_by_tag_name(None, "body".to_string())[0];
            let modal_box = crate::Element::from_string("<div id=\"modal-box\"><div id=\"close-button\">Click the image to close.</div><img id=\"modal-img\" onclick=\"close_modal_img();\"/></div>".to_string()).unwrap();
            let script = crate::Element::from_string("<script>/*<![CDATA[*/var modal_box = document.getElementById(\"modal-box\");var modal_img = document.getElementById(\"modal-img\");
    function open_modal_img(src) {
        modal_img.src = src;
        modal_box.style.display = \"block\";
    }
    function close_modal_img() {
        modal_box.style.display = \"none\";
    }/*]]>*/</script>".to_string()).unwrap();
            body.add_element_ptr(modal_box);
            body.add_element_ptr(script);
        }

        let r = crate::dom::get_root();

        let mut f = File::create("test copy.html").unwrap();
        f.write_all(r.to_string().as_bytes()).unwrap();
        //panic!("{}", r.to_string());
    }

}