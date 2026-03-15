use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Resource {
    id: i64,
    name: String
}


impl Resource {
    pub fn new(id: i64, name: &String) -> Self {
        Self {
            id,
            name: name.clone()
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Display  for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource ( id: {}, name {})", self.id, self.name) 
    }
}