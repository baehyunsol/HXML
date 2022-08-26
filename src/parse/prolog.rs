use super::{get_processing_instruction_end_index, get_comment_end_index, get_name_end_index, get_eq_end_index};
use crate::node::prolog::{DocTypeDecl, Prolog, XMLDecl};
use crate::predicate::{
    is_alpha_cap,
    is_alpha_low,
    is_numeric,
    is_whitespace,
};
use crate::utils::{from_v16, into_v16};

// https://www.w3.org/TR/xml/#NT-prolog
// xml_decl? miscellaneous* (doctype_decl miscellaneous*)?
pub fn get_prolog_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index >= document.len() {
        return None;
    }

    let initial_index = index;

    match get_xml_decl_end_index(document, index) {
        Some(xml_decl_end_index) => {
            index = xml_decl_end_index + 1;
        },
        None => {}
    }

    while let Some(misc_end_index) = get_miscellaneous_end_index(document, index) {
        index = misc_end_index + 1;
    }

    match get_doctype_decl_end_index(document, index) {
        Some(doctype_decl_end_index) => {
            index = doctype_decl_end_index + 1;
        },
        None => {

            if initial_index == index {
                return None;
            }
        
            else {
                return Some(index - 1);
            }

        }
    }

    while let Some(misc_end_index) = get_miscellaneous_end_index(document, index) {
        index = misc_end_index + 1;
    }

    if initial_index == index {
        None
    }

    else {
        Some(index - 1)
    }

}

// https://www.w3.org/TR/xml/#NT-Misc
// comment | processing_instruction | whitespace
pub fn get_miscellaneous_end_index(document: &[u16], index: usize) -> Option<usize> {

    if index >= document.len() {
        return None;
    }

    match get_comment_end_index(document, index) {
        None => match get_processing_instruction_end_index(document, index) {
            None => if is_whitespace(&document[index]) {
                Some(index)
            } else {
                None
            },
            Some(end_index) => Some(end_index)
        },
        Some(end_index) => Some(end_index)
    }

}

// https://www.w3.org/TR/xml/#NT-XMLDecl
// '<?xml' version_info encoding_decl? sd_decl? whitespace? '?>'
pub fn get_xml_decl_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 5 >= document.len() || &document[index..index + 5] != &into_v16("<?xml") {
        return None;
    }

    index += 5;

    match get_version_info_end_index(document, index) {
        Some(version_info_end_index) => {
            index = version_info_end_index;
        }
        None => { return None; }
    }

    match get_encoding_decl_end_index(document, index + 1) {
        Some(encoding_decl_end_index) => {
            index = encoding_decl_end_index;
        }
        None => {}
    }

    match get_sd_decl_end_index(document, index + 1) {
        Some(sd_decl_end_index) => {
            index = sd_decl_end_index;
        }
        None => {}
    }

    if index + 1 < document.len() && is_whitespace(&document[index + 1]) {
        index += 1;
    }

    if index + 2 < document.len() && document[index + 1] == '?' as u16 && document[index + 2] == '>' as u16 {
        Some(index + 2)
    }

    else {
        None
    }

}

// https://www.w3.org/TR/xml/#NT-VersionInfo
// whitespace 'version' eq ("'" version_num "'" | '"' version_num '"')
pub fn get_version_info_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index >= document.len()
        || !is_whitespace(&document[index])
        || &document[index + 1..index + 8] != into_v16("version")
    {
        return None;
    }

    index += 8;

    match get_eq_end_index(document, index + 1) {
        Some(eq_end_index) => {

            if eq_end_index + 1 < document.len() &&
            (document[eq_end_index + 1] == '\'' as u16 || document[eq_end_index + 1] == '"' as u16) {
                let quote = document[eq_end_index + 1];

                match get_version_num_end_index(document, eq_end_index + 2) {
                    Some(version_num_end_index) if version_num_end_index + 1 < document.len() &&
                    document[version_num_end_index + 1] == quote => Some(version_num_end_index + 1),
                    _ => None
                }

            }

            else {
                None
            }

        },
        None => None
    }

}

// https://www.w3.org/TR/xml/#NT-VersionNum
// '1.' [0-9]+
pub fn get_version_num_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 2 > document.len()
        || document[index] != '1' as u16
        || document[index + 1] != '.' as u16
        || !is_numeric(&document[index + 2])
    {
        return None;
    }

    index += 2;

    while index < document.len() && is_numeric(&document[index]) {
        index += 1;
    }

    Some(index)
}

// https://www.w3.org/TR/xml/#NT-SDDecl
// whitespace 'standalone' eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
pub fn get_sd_decl_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 11 > document.len()
        || !is_whitespace(&document[index])
        || &document[(index + 1)..(index + 11)] != &into_v16("standalone")
    {
        return None;
    }

    index += 11;

    index = match get_eq_end_index(document, index) {
        None => { return None; }
        Some(i) => i
    };

    if &document[(index + 1)..(index + 5)] == &into_v16("\"no\"")
        || &document[(index + 1)..(index + 5)] == &into_v16("'no'")
    {
        Some(index + 5)
    }

    else if &document[(index + 1)..(index + 6)] == &into_v16("\"yes\"")
        || &document[(index + 1)..(index + 6)] == &into_v16("'yes'")
    {
        Some(index + 6)
    }

    else {
        None
    }

}

// https://www.w3.org/TR/xml/#NT-EncodingDecl
// whitespace 'encoding' eq ('"' encoding_name '"' | "'" encoding_name "'" )
pub fn get_encoding_decl_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 9 >= document.len()
        || !is_whitespace(&document[index])
        || &document[(index + 1)..(index + 9)] != &into_v16("encoding")
    {
        return None;
    }

    index += 9;

    index = match get_eq_end_index(document, index) {
        None => { return None; }
        Some(i) => i
    };

    if index + 1 >= document.len()
        || document[index + 1] != '\'' as u16
        || document[index + 1] != '"' as u16
    {
        None
    }

    else {
        let quot = document[index + 1];

        let name_end_index = match get_encoding_name_end_index(document, index + 2) {
            None => {
                return None;
            }
            Some(i) => i
        };

        if name_end_index + 1 < document.len() && document[name_end_index + 1] == quot {
            Some(name_end_index + 1)
        }

        else {
            None
        }

    }

}

// https://www.w3.org/TR/xml/#NT-EncName
// [A-Za-z] ([A-Za-z0-9._] | '-')*
pub fn get_encoding_name_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index >= document.len()
        || !is_alpha_low(&document[index])
        && !is_alpha_cap(&document[index])
    {
        return None;
    }

    index += 1;

    while index < document.len() && (
        !is_alpha_low(&document[index])
        || !is_alpha_cap(&document[index])
        || document[index] == '.' as u16
        || document[index] == '_' as u16
        || document[index] == '-' as u16
    ) {
        index += 1;
    }

    Some(index)
}

// https://www.w3.org/TR/xml/#NT-doctypedecl
// '<!DOCTYPE' whitespace name (whitespace ExternalID)? whitespace? ('[' internal_subset ']' whitespace?)? '>'
pub fn get_doctype_decl_end_index(document: &[u16], mut index: usize) -> Option<usize> {

    if index + 11 >= document.len()
        || &document[index..index + 9] != into_v16("<!DOCTYPE")
        || !is_whitespace(&document[index + 9])
    {
        return None;
    }

    index += 10;

    match get_name_end_index(document, index + 1) {
        Some(name_end_index) => if name_end_index + 1 >= document.len() {
            None
        } else if document[name_end_index + 1] == '>' as u16 {
            Some(name_end_index + 1)
        } else if document[name_end_index + 1] == '[' as u16 {

            let internal_subset_end_index = match get_internal_subset_end_index(document, name_end_index + 2) {
                None => { return None; }
                Some(i) => i
            };

            todo!()
        } else if is_whitespace(&document[name_end_index + 1]) {
            todo!()
        } else {
            None
        },
        None => None
    }

}

// https://www.w3.org/TR/xml/#NT-intSubset
// (markupdecl | DeclSep)*
pub fn get_internal_subset_end_index(document: &[u16], mut index: usize) -> Option<usize> {
    todo!()
}

// it assumes that get_prolog_end_index(document, index) returns Some(..) for this args
pub fn parse_prolog(document: &[u16], mut index: usize) -> (Prolog, usize) {  // (Prolog, end_index)

    let begin_index = index;
    let mut xml_decl = None;
    let mut doctype_decl = None;

    loop {

        match get_miscellaneous_end_index(document, index) {
            Some(miscellaneous_end_index) => {
                index = miscellaneous_end_index + 1;
                continue;
            }
            _ => {}
        }

        match get_xml_decl_end_index(document, index) {
            Some(xml_decl_end_index) => {
                xml_decl = Some(parse_xml_decl(document, index + 5));  // 5 for `<?xml`
                index = xml_decl_end_index + 1;
                continue;
            }
            _ => {}
        }

        match get_doctype_decl_end_index(document, index) {
            Some(doctype_decl_end_index) => {
                doctype_decl = Some(parse_doctype_decl(document, index + 9));  // 9 for `<!DOCTYPE`
                index = doctype_decl_end_index + 1;
                continue;
            }
            _ => {}
        }

        break;
    }

    (
        Prolog::new(xml_decl, doctype_decl),
        if begin_index == index { index } else { index - 1 }
    )
}

pub fn parse_xml_decl(document: &[u16], index: usize) -> XMLDecl {
    todo!()
}

pub fn parse_doctype_decl(document: &[u16], index: usize) -> DocTypeDecl {
    #[cfg(test)]
    assert!(is_whitespace(&document[index]));

    let name_end_index = get_name_end_index(document, index + 1).unwrap();
    let name = document[(index + 1)..(name_end_index + 1)].to_vec();

    DocTypeDecl::new(from_v16(&name))
}

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::testbench::*;

    #[test]
    fn prolog_test() {
        get_xxx_end_index(
            vec![
                ("not a prolog", None),
                ("  ", Some(1)),
                ("  end", Some(1)),
                ("<!DOCTYPE html>", Some(14)),
                (" <!DOCTYPE html>", Some(15)),
                ("<!DOCTYPE html> ", Some(15)),
                (" <!DOCTYPE html> ", Some(16)),
            ],
            get_prolog_end_index
        );
    }

    #[test]
    fn doctype_test() {
        get_xxx_end_index(
            vec![
                ("not a doctype", None),
                ("<!DOCTYPE html>", Some(14)),
            ],
            get_doctype_decl_end_index
        );
    }

}