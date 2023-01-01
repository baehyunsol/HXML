use crate::{dom, into_dom, ElementPtr};
use crate::utils::{into_v16, from_v16};

pub fn get_xxx_end_index(cases: Vec<(&str, Option<usize>)>, func: fn(&[u16], usize) -> Option<usize>){

    let mut cases = cases.into_iter().map(
        |(case, answer)|
        (0..4).map(
            |n|
            (
                vec![vec![' ' as u16; n], into_v16(case)].concat(),
                n,
                match answer {
                    None => None,
                    Some(ans) => Some(ans + n)
                }
            )
        ).collect()
    ).collect::<Vec<Vec<(Vec<u16>, usize, Option<usize>)>>>().concat();

    // xxx cannot be empty
    cases.push((vec![], 0, None));
    cases.push((vec![], 1, None));
    cases.push((vec![], 2, None));

    let mut errors = vec![];

    for (case, start, answer) in cases.iter() {
        let result = func(case, *start);

        if &result != answer {
            errors.push(
                format!(
                    "case: `{}`\nstart: {}\nanswer: {:?}\nactual result: {:?}",
                    from_v16(case), start, answer, result
                )
            );
        }

    }

    if errors.len() > 0 {
        panic!("{} out of {} tests have failed!\n\n{}{}", errors.len(), cases.len(), errors.join("\n\n"), "\n\n------------------");
    }

}

fn check_tree(node: ElementPtr) {

    for child in node.get_children().into_iter() {
        assert_eq!(child.get_parent().unwrap(), node);
        check_tree(child);
    }

}

// It converts between String <-> dom multiple times, then sees if the dom remains unchanged.
// If the result string changes or it fails to parse, there must be an error with its implementation.
// `tags` includes some tags that are in the given xml. It tests if the dom selector succefully finds the tag.
pub fn parse_valid_xml(xml: String, tags: Vec<String>, ids: Vec<String>) {
    into_dom(xml).unwrap();

    for tag in tags.iter() {
        assert_eq!(dom::get_elements_by_tag_name(None, tag.clone())[0], dom::get_element_by_tag_name(None, tag.clone()).unwrap());
    }

    for id in ids.iter() {
        assert!(dom::get_element_by_id(None, id.clone()).is_some());
    }

    let another_string = dom::to_string();
    into_dom(another_string.clone()).unwrap();

    for tag in tags.iter() {
        assert_eq!(dom::get_elements_by_tag_name(None, tag.clone())[0], dom::get_element_by_tag_name(None, tag.clone()).unwrap());
    }

    for id in ids.iter() {
        assert!(dom::get_element_by_id(None, id.clone()).is_some());
    }

    check_tree(dom::get_root());

    assert_eq!(another_string, dom::to_string());
}

#[test]
fn xml_dom_test() {

    let testcases: Vec<(&str, Vec<&str>, Vec<&str>)> = vec![
        ("<html></html>", vec!["html"], vec![]),
        ("<!DOCTYPE html><html><head></head><body></body></html>", vec!["head", "body"], vec![]),
        ("<!DOCTYPE html><html><head></head><body><p id=\"references\">&#123;&nbsp;</p></body></html>", vec!["head", "body"], vec!["references"]),
        ("<body><img src=\"a.jpg\"/><div id=\"div1\"><p>Paragraph 1</p><p>Paragraph 2</p></div><div><p>Paragraph 3</p><p>Paragraph 4</p></div></body>", vec!["div", "p"], vec!["div1"]),
        ("<body><img src=\"a.jpg\"/><div><p>Paragraph 3</p><p>Paragraph 4</p></div></body>", vec!["div", "p"], vec![]),
        ("<body><div class=\"a b c\" id=\"div1\"></div><div class=\"a c d\" id=\"div2\"></div><div id=\"div3\" class=\"b d f\"></div><div id=\"div4\" class=\"b d a\"></div></body>", vec!["div", "body"], vec!["div1", "div2", "div3", "div4"]),
    ];

    // I'm too lazy to write `.to_string()` multiple times... haha
    let testcases: Vec<(String, Vec<String>, Vec<String>)> = testcases.into_iter().map(
        |(xml, tags, ids)|
        (
            xml.to_string(),
            tags.into_iter().map(|tag| tag.to_string()).collect(),
            ids.into_iter().map(|id| id.to_string()).collect(),
        )
    ).collect();

    for (xml, tags, ids) in testcases.clone().into_iter() {
        parse_valid_xml(xml, tags, ids);
    }

    let mut before_deletion = testcases[3].0.clone();
    let mut after_deletion = testcases[4].0.clone();

    into_dom(before_deletion).unwrap();
    let div_to_delete = dom::get_element_by_id(None, "div1".to_string()).unwrap();
    dom::delete(div_to_delete);
    before_deletion = dom::to_string();

    into_dom(after_deletion).unwrap();
    after_deletion = dom::to_string();

    assert_eq!(before_deletion, after_deletion);

    let class_test = testcases[5].0.clone();

    into_dom(class_test).unwrap();
    let div1 = dom::get_element_by_id(None, "div1".to_string()).unwrap();
    let div2 = dom::get_element_by_id(None, "div2".to_string()).unwrap();
    let div3 = dom::get_element_by_id(None, "div3".to_string()).unwrap();
    let div4 = dom::get_element_by_id(None, "div4".to_string()).unwrap();

    let class_a = dom::get_elements_by_class_name(None, "a".to_string());
    let class_b = dom::get_elements_by_class_name(None, "b".to_string());
    let class_c = dom::get_elements_by_class_name(None, "c".to_string());
    let class_d = dom::get_elements_by_class_name(None, "d".to_string());

    assert!(class_a.contains(&div1) && class_a.contains(&div2) && !class_a.contains(&div3) && class_a.contains(&div4));
    assert!(class_b.contains(&div1) && !class_b.contains(&div2) && class_b.contains(&div3) && class_b.contains(&div4));
    assert!(class_c.contains(&div1) && class_c.contains(&div2) && !class_c.contains(&div3) && !class_c.contains(&div4));
    assert!(!class_d.contains(&div1) && class_d.contains(&div2) && class_d.contains(&div3) && class_d.contains(&div4));
    assert_eq!(dom::get_elements_by_class_name(None, "f".to_string()), vec![div3]);
}