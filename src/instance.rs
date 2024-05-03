use crate::class::Class;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Instance {
    klass: Class,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        Instance { klass }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", format!("{} instance", self.klass.name.clone()))
    }
}
