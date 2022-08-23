#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {

    pub fn new(name: String, value: String) -> Self {

        Attribute {
            name, value
        }

    }

}