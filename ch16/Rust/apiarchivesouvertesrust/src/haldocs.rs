#[allow(non_snake_case)]

use crate::haldoc::HALDoc;

use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {

    numFound: i64,
    start: i64,
    docs: Vec<HALDoc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HALDocs {

    response: Response,
}

impl HALDocs {

    pub fn haldocs(&self) -> &Vec<HALDoc> {
        &self.response.docs
    }

    pub fn numFound(&self) -> i64 {
        self.response.numFound
    }

    pub fn start(&self) -> i64 {
        self.response.start
    }
}