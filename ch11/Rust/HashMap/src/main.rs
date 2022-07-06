use std::collections::HashMap;

fn fn_btreemap() {
    
    use std::collections::BTreeMap;

    println!(" --- BTreeMap. --- ");

    let mut abecedaire_pays_tree = BTreeMap::new();

    // On ajoute des paires clé-valeur dans la collection.
    abecedaire_pays_tree.insert(
        "A".to_string(),
        "Arménie".to_string(),
    );
    abecedaire_pays_tree.insert(
        "B".to_string(),
        "Brésil".to_string(),
    );
    abecedaire_pays_tree.insert(
        "C".to_string(),
        "Cameroun".to_string(),
    );
    abecedaire_pays_tree.insert(
        "D".to_string(),
        "Danemark".to_string(),
    );
    println!("abecedaire_pays : {:?}", abecedaire_pays_tree);

    if !abecedaire_pays_tree.contains_key("C") {
        println!("Nous n'avons pas encore d'entrée d'abcédaire pour la lettre 'C'");
    }
    else {
        println!("Clé-valeur 'C'-{}", abecedaire_pays_tree["C"]);
    }

    // Finalement on retire la clé 'B'.
    abecedaire_pays_tree.remove("B");
    println!("abecedaire_pays : {:?}", abecedaire_pays_tree);

    // On fait tourner un itérateur sur la collection.
    for (clé, valeur) in &abecedaire_pays_tree {
        println!("Clé : {} - Valeur : {}", clé, valeur);
    }
}

fn main() {

    let mut abecedaire_pays = HashMap::new();

    // On ajoute des paires clé-valeur dans la collection.
    abecedaire_pays.insert(
        "A".to_string(),
        "Arménie".to_string(),
    );
    abecedaire_pays.insert(
        "B".to_string(),
        "Brésil".to_string(),
    );
    abecedaire_pays.insert(
        "C".to_string(),
        "Cameroun".to_string(),
    );
    abecedaire_pays.insert(
        "D".to_string(),
        "Danemark".to_string(),
    );
    println!("abecedaire_pays : {:?}", abecedaire_pays);

    if !abecedaire_pays.contains_key("E") {
        println!("Nous n'avons pas encore d'entrée d'abcédaire pour la lettre 'E'");
    }
    else {
        println!("Clé-valeur 'E'-{}", abecedaire_pays["E"]);
    }

    // Finalement on retire la clé 'D'.
    abecedaire_pays.remove("D");
    println!("abecedaire_pays : {:?}", abecedaire_pays);


    // On fait tourner un itérateur sur la collection.
    for (clé, valeur) in &abecedaire_pays {
        println!("Clé : {} - Valeur : {}", clé, valeur);
    }

    fn_btreemap();
}
