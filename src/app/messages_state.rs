
#[derive(Debug, Default)]
pub struct MessagesState {
    messages: Vec<String>,
}

impl MessagesState {
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn push<StrT: Into<String>>(&mut self, message: StrT) {
        self.messages.push(message.into());
    }

    pub fn messages(&self) -> &Vec<String> {
        &self.messages
    }
}
