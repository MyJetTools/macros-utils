pub struct AttributeFields {
    pub content: Vec<u8>,
}

impl AttributeFields {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}
