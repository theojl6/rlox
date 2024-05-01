#[derive(Clone, Debug)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}
