fn main() {
    // Solution 1 : Transfert de propriété de valeur
    let chaine = String::from("ENI");
    let (chaine_retour, taille) = obtenir_taille(chaine);

    println!("{}", chaine_retour);
    println!("{}", taille);

    println!("{}", " ------ ");

    // solution 2 : prêt de valeur
    let chaine2 = String::from("ENI");
    let taille2 = obtenir_taille_2(&chaine2);

    println!("{}", chaine2);
    println!("{}", taille2);
}

fn obtenir_taille(ch: String) -> (String, usize) {
   let taille = ch.len(); 
   (ch, taille)
}

fn obtenir_taille_2(ch: &String) -> usize {
    ch.push_str(" !");
    ch.len()
}