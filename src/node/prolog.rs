#[derive(Clone)]
pub struct Prolog {
    xml_decl: Option<XMLDecl>,
    doctype_decl: Option<DocTypeDecl>
}

impl Prolog {

    pub fn new(xml_decl: Option<XMLDecl>, doctype_decl: Option<DocTypeDecl>) -> Self {
        Prolog { xml_decl, doctype_decl }
    }

    pub fn get_doctype_name(&self) -> Option<String> {

        match &self.doctype_decl {
            Some(d) => Some(d.name.clone()),
            _ => None
        }

    }

    pub fn get_xml_version(&self) -> Option<String> {

        match &self.xml_decl {
            Some(d) => Some(d.version_num.clone()),
            _ => None
        }

    }

    pub fn to_string(&self) -> String {
        let xml_decl_string = match &self.xml_decl {
            Some(x) => x.to_string(),
            _ => String::new()
        };
        let doctype_decl_string = match &self.doctype_decl {
            Some(d) => d.to_string(),
            _ => String::new()
        };

        format!(
            "{}{}",
            xml_decl_string,
            doctype_decl_string,
        )
    }

}

#[derive(Clone)]
pub struct XMLDecl {
    pub version_num: String
}

impl XMLDecl {

    pub fn to_string(&self) -> String {
        format!("<?xml version='{}'>", self.version_num)
    }

}

#[derive(Clone)]
pub struct DocTypeDecl {
    pub name: String
}

impl DocTypeDecl {

    pub fn new(name: String) -> DocTypeDecl {
        DocTypeDecl { name }
    }

    pub fn to_string(&self) -> String {
        format!("<!DOCTYPE {}>", self.name)
    }

}