use crate::{dom, into_dom};
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

    assert_eq!(another_string, dom::to_string());
}

#[test]
fn xml_dom_test() {

    let testcases: Vec<(&str, Vec<&str>, Vec<&str>)> = vec![
        ("<html></html>", vec!["html"], vec![]),
        ("<!DOCTYPE html><html><head></head><body></body></html>", vec!["head", "body"], vec![]),
        ("<!DOCTYPE html><html><head></head><body><p id=\"references\">&#123;&nbsp;</p></body></html>", vec!["head", "body"], vec!["references"]),
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

    for (xml, tags, ids) in testcases.into_iter() {
        parse_valid_xml(xml, tags, ids);
    }

}