use reqwest::blocking::Response;
use crate::haldocs::HALDocs;
use crate::halerror::HALError;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

pub struct HALClient {
    
    base_url:  String
}

impl HALClient {
    
    pub fn new() -> HALClient {

        println!("{}", "HALClient::new()");
        HALClient {
            base_url: "http://api.archives-ouvertes.fr/search/".to_string()
        }
    }

    pub fn basic_search(&self, q: &str) -> Result<HALDocs, HALError> {

        let url: String = self.base_url.clone() + "?q='" + &q.to_string() + "'&wt=json";
        println!("{}", url);

        let client = reqwest::blocking::Client::new();
        let response: Result<Response, ReqwestError> = client.get(url).send();
        let content = match response {
            Ok(response) => response.text(),
            Err(reason) => panic!("{}", reason),
        };
        //println!("{:?}", content);

        let text = match content {
            Ok(text) => text,
            Err(reason) => panic!("{}", reason),
        };
        //println!("{:?}", text);

        let haldocs: Result<HALDocs, SerdeJsonError> = serde_json::from_str(text.as_str());
        //println!("{:?}", haldocs);

        match haldocs {
            Ok(haldocs) => Ok(haldocs),
            Err(reason) => Err(HALError::GenericError(reason.to_string()))
        }
    }
}
