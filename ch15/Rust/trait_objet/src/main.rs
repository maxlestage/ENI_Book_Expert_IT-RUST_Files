use std::fmt::Display;

// Test de la surcharge statique.
fn afficher_valeur<T:Display>(valeur : T){

    println!("Affichage de la valeur : {} ", valeur);
 }

 // Test de la surcharge dynamique.
struct S1 {
     valeur: i64
}
 
struct S2 {
    valeur: f64
}

struct S3 {
    valeur:  String
}

trait Affichage {
     fn afficher_valeur(&self);
}
 
impl Affichage for S1 {

     fn afficher_valeur(&self) {
         println!("Entier {:?}", self.valeur);
     }
 }
 
impl Affichage for S2 {

    fn afficher_valeur(&self) {
        println!("Flottant {:?}", self.valeur);
    }
}

impl Affichage for S3 {

    fn afficher_valeur(&self) {
        println!("String {:?}", self.valeur);
    }
}

fn fonction_surcharge_statique<T:Display>(valeur : T){
    println!("Affichage de la valeur de manière statique : {} ", valeur);     
 }

fn fonction_surcharge_dynamique(objet_implementant_trait: &(dyn Affichage + 'static)) {

    objet_implementant_trait.suivi_appel_surcharge_dynamique();
    objet_implementant_trait.afficher_valeur();
}

impl dyn Affichage + 'static {
    fn suivi_appel_surcharge_dynamique(&self) {
        println!("Ici on utilise un objet-trait.");
    }
}

fn main(){

    // Test de la surcharge statique.
    //afficher_valeur(42 as u64);
    //afficher_valeur(3.14159);
    //afficher_valeur("Bonjour tout le monde.".to_string());

    // Test de la surcharge dynamique avec un objet-trait.
    let s1 = S1{valeur : 42};
    let s2 = S2{valeur : 3.14159};
    let s3 = S3{valeur : "Bonjour tout le monde.".to_string()};
    fonction_surcharge_statique(s1.valeur);
    fonction_surcharge_statique(s2.valeur);
    fonction_surcharge_statique(s3.valeur);
    
    let s1_d = S1{valeur : 42};
    let s2_d = S2{valeur : 3.14159};
    let s3_d= S3{valeur : "Bonjour tout le monde.".to_string()};
    fonction_surcharge_dynamique(&s1_d);
    fonction_surcharge_dynamique(&s2_d);
    fonction_surcharge_dynamique(&s3_d);
}

