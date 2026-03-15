use crate::domain::Resource;

pub struct Message {
    id: i64,
    msg: String,
}

impl Message {
    pub fn new(id: i64, msg: String) -> Self {
        Self {
            id,
            msg
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl From<Resource> for Message {
    fn from(value: Resource) -> Self {
        Self {
            id: value.id(),
            msg: format!("CREATED {} {}", value.id(), value.name())
        }
    }
}