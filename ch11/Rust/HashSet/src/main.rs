use std::collections::HashSet;

fn fn_btree_set(){

    println!(" --- BTreeSet. ---");

    use std::collections::BTreeSet;

    let mut alphabet_1 = BTreeSet::new();

    alphabet_1.insert("A".to_string());
    alphabet_1.insert("B".to_string());
    alphabet_1.insert("C".to_string());
    alphabet_1.insert("D".to_string());
    alphabet_1.insert("E".to_string());
    alphabet_1.insert("F".to_string());
    alphabet_1.insert("G".to_string());
    alphabet_1.insert("H".to_string());
    alphabet_1.insert("I".to_string());
    alphabet_1.insert("J".to_string());
    alphabet_1.insert("K".to_string());
    alphabet_1.insert("L".to_string());
    println!("alphabet_1 : {:?}", alphabet_1);
    println!("alphabet_1 - longueur : {}", alphabet_1.len());

    // Est-ce que B est inclus ?
    if !alphabet_1.contains("B") {
        println!("B n'est pas présent dans l'ensemble.");
    } else {
        println!("B est présent dans l'ensemble.");
    }

    // On retire un élément.
    alphabet_1.remove("L");
    println!("alphabet_1 : {:?}", alphabet_1);
    println!("alphabet_1 - longueur : {}", alphabet_1.len());

    // On itère sur les valeurs de l'ensemble.
    for lettre in &alphabet_1 {
        println!("{}", lettre);
    }
}

fn main() {

    let mut alphabet_1 = HashSet::new();

    alphabet_1.insert("A".to_string());
    alphabet_1.insert("B".to_string());
    alphabet_1.insert("C".to_string());
    alphabet_1.insert("D".to_string());
    alphabet_1.insert("E".to_string());
    alphabet_1.insert("F".to_string());
    alphabet_1.insert("G".to_string());
    alphabet_1.insert("H".to_string());
    alphabet_1.insert("I".to_string());
    alphabet_1.insert("J".to_string());
    alphabet_1.insert("K".to_string());
    alphabet_1.insert("L".to_string());
    println!("alphabet_1 : {:?}", alphabet_1);
    println!("alphabet_1 - longueur : {}", alphabet_1.len());

    // Est-ce que B est inclus ?
    if !alphabet_1.contains("B") {
        println!("B n'est pas présent dans l'ensemble.");
    } else {
        println!("B est présent dans l'ensemble.");
    }

    // On retire un élément.
    alphabet_1.remove("L");
    println!("alphabet_1 : {:?}", alphabet_1);
    println!("alphabet_1 - longueur : {}", alphabet_1.len());

    // On itère sur les valeurs de l'ensemble.
    for lettre in &alphabet_1 {
        println!("{}", lettre);
    }

    // 2e alphabet.
    let mut alphabet_2 = HashSet::new();
    alphabet_2.insert("H".to_string());
    alphabet_2.insert("I".to_string());
    alphabet_2.insert("J".to_string());
    alphabet_2.insert("K".to_string());
    alphabet_2.insert("L".to_string());
    alphabet_2.insert("M".to_string());
    alphabet_2.insert("N".to_string());
    alphabet_2.insert("O".to_string());
    alphabet_2.insert("P".to_string());
    println!("alphabet_2 : {:?}", alphabet_2);
    println!("alphabet_2 - longueur : {}", alphabet_2.len());

    // Test insertion élément déjà présent.
    alphabet_2.insert("P".to_string());
    println!("alphabet_2 : {:?}", alphabet_2);
    println!("alphabet_2 - longueur : {}", alphabet_2.len());

    // Opérations logiques sur les ensembles.
    println!(" --- Opérations logiques --- ");

    // Intersection.
    let intersection  = &alphabet_1 & &alphabet_2;
    println!("intersection : {:?}", intersection);

    // Union "Soit dans l'un, soit dans l'autre, soit les deux".
    let iterateur_union = alphabet_1.union(&alphabet_2);
    println!("union : {:?}", iterateur_union);

    // Différence : présent dans alphabet_1, mais pas dans alphabet_2.
    let iterateur_difference = alphabet_1.difference(&alphabet_2);
    println!("difference : {:?}", iterateur_difference);

    // Différence symétrique : présent dans alphabet_1 ou dans alphabet_2, mais pas dans les deux.
    let iterateur_symmetric_difference = alphabet_1.symmetric_difference(&alphabet_2);
    println!("difference symétrique : {:?}", iterateur_symmetric_difference);

    // Disjoint ? Oui ou non.
    println!("Disjoint ? {}", alphabet_1.is_disjoint(&alphabet_2));

    // Sous-ensemble ? Oui ou non.
    println!("Sous-ensemble ? {}", alphabet_1.is_subset(&alphabet_2));

    // Sur-ensemble ? Oui ou non.
    println!("Sur-ensemble ? {}", alphabet_1.is_superset(&alphabet_2));

    fn_btree_set();
}
