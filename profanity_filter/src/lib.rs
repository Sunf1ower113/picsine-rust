pub struct Message {
    content: String,
    user: String
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Self {
            content: ms,
            user: u
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            return Option::None
        }
        Option::Some(self.content.as_str())
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    if ms.send_ms() == None {
        return (false, "ERROR: illegal");
    }
    return (true, ms.content.as_str())
}