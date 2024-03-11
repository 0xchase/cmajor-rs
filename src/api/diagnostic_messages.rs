pub struct DiagnosticMessageList {
    messages: Vec<DiagnosticMessage>,
}

impl DiagnosticMessageList {
    pub fn add(&mut self, message: DiagnosticMessage) {
        self.messages.push(message);
    }

    pub fn add_from_json_string(&mut self, json: &str) -> bool {
        todo!()
    }
}

pub struct DiagnosticMessage {}

impl DiagnosticMessage {
    pub fn create_error(message: &str) -> Self {
        todo!()
    }
}
