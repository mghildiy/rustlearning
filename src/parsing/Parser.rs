
pub struct Message {
    message: String
}

impl Message {
    pub fn from(m: String) -> Self {
        Message {message: m}
    }

    pub fn parse(&self) {
        let mess = &self.message;
    }
}
