use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HALDoc {
    
    docid: i64,
    label_s: Option<String>,
    uri_s: Option<String>
}

impl HALDoc {

    pub fn new(
        docid: i64,
        label_s: Option<String>,
        uri_s: Option<String>
    ) -> HALDoc {
        HALDoc {
            docid,
            label_s,
            uri_s
        }
    }

    pub fn docid(&self) -> i64 {
        self.docid
    }

    pub fn label_s(&self) -> Option<String> {
        self.label_s.clone()
    }

    pub fn uri_s(&self) -> Option<String> {
        self.uri_s.clone()
    }
}

impl fmt::Display for HALDoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}