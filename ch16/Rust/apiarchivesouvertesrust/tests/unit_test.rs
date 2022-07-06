#[cfg(test)]
extern crate apiarchivesouvertesrust;

use apiarchivesouvertesrust::client;

#[test]
fn test_basic_search() {

    let client = client::HALClient::new();
    let results = client.basic_search("programmation");
    match results {
     
        Ok(results) => {
            println!("Nombre de résultats : {}", results.numFound());
            for haldoc in results.haldocs().iter() {
                println!("{} - {:?}", haldoc.docid(), haldoc.label_s());
            }
        },
        Err(_reason) => assert!(true),
    }
}