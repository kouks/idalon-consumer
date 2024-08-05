#[derive(Debug)]
pub struct IdalonError {
    msg: String,
}

impl IdalonError {
    pub fn new(msg: &str) -> IdalonError {
        IdalonError {
            msg: String::from(msg),
        }
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}

pub fn fetch(e: reqwest::Error) -> IdalonError {
    IdalonError {
        msg: format!("Failed to fetch URL: {:?}", e),
    }
}

pub fn parse(e: reqwest::Error) -> IdalonError {
    IdalonError {
        msg: format!("Failed to parse JSON from URL: {:?}", e),
    }
}
