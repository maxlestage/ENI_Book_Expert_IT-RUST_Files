#[warn(non_snake_case)]

fn main() {
    //possession_chaine_tas()
    //possession_entier_pile()
    //possession_fonction()
    possession_retour_fonction()
}

fn possession_chaine_tas() {
    let chaine = String::from("ENI");
    let chaine2 = chaine;
    println!("{}", chaine2);
    //println!("{}", chaine); // provoque une erreur car la valeur a été prêtée.

    // Copie dans le tas : deux emplacements mémoire différents
    let chaine_clone = chaine2.clone();
    println!("{:p}", &chaine2);
    println!("{:p}", &chaine_clone);
}

fn possession_entier_pile() {
    let annee_hector : i64 = 2011;
    let annee_hector_2 : i64 = annee_hector;

    println!("{}", annee_hector);
    println!("{:p}", &annee_hector);
    println!("{}", annee_hector_2);
    println!("{:p}", &annee_hector_2);
    
}

fn possession_fonction(){
    
    let chaine = String::from("ENI");
    let annee_arthur : i64 = 2007;

    println!("{}", chaine);
    println!("{}", annee_arthur);

    passage_fonction( chaine, annee_arthur);

    let valeur = annee_arthur;
    println!("{}", valeur);
    //println!("{}", chaine); // Déclenche une erreur de compilation, la valeur a été déplacée.
}

fn passage_fonction(texte: String, entier: i64){
    println!("{}", texte);
    println!("{}", entier);
}

fn possession_retour_fonction() {
    let chaine = String::from("ENI");
    let chaine_retour = possession_retour_fonction_appel(chaine);
    println!("{}", chaine_retour);
}

fn possession_retour_fonction_appel(chaine_param : String) -> String {
    chaine_param 
}