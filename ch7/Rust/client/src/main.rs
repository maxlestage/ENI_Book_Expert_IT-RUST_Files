#[warn(unused_variables)]

extern crate animal;
use crate::animal::mod_animal::Animal;

pub struct Test;

mod tortue;
use crate::tortue::*;

pub fn afficher_trait(animal: &impl Animal) {
    animal.afficher();
}

pub fn afficher_trait_2<T: Animal>(animal: &T) {
    animal.afficher();
}

pub fn getbiggestname(animal1: &impl Animal, animal2: &impl Animal) -> String {
    
    if animal1.obtenir_nom().len() > animal2.obtenir_nom().len() {
        return animal1.obtenir_nom();
    }
    else if animal1.obtenir_nom().len() < animal2.obtenir_nom().len(){ 
        return animal2.obtenir_nom();
    }
    "Même longueur".to_string()
}

/*
fn obtenir_animal(b : bool) -> impl Animal {

    let nom : String = "Mimi l'animal".to_string();
    if b == true {  
        animal::mod_animal::Chien::creer(nom)
    }
    else {
        animal::mod_animal::Chat::creer(nom)
    }
}
*/

fn main() {

    // Un chien.
    let nom_chien : String = "Toto le cabot".to_string();
    let toto = animal::mod_animal::Chien::creer(nom_chien);
    println!("{}", toto.obtenir_nom());
    toto.afficher();

    // Un chat.
    let nom_chat : String = "Kiki le chaton".to_string();
    let kiki = animal::mod_animal::Chat::creer(nom_chat);
    println!("{}", kiki.obtenir_nom());
    kiki.dormir();
    kiki.afficher();

    println!("--- Trait en paramètre ---");
    afficher_trait(&toto);
    afficher_trait(&kiki);

    let _test = Test;
    //afficher_trait(&_test);

    println!("Le plus long nom des deux : {}", getbiggestname(&kiki, &toto));

    // Une tortue (implémentation de deux traits).
    println!("--- Deux traits, Tortue et Display ---");
    let nom_tortue : String = "Tutu la tortue".to_string();
    let tutu = Tortue::creer(nom_tortue);
    println!("{}", tutu);
    affichage_format(&tutu);

    // Provoque une erreur (n'implémente pas Display).
    //affichage_format(&toto);
    
}

// Course d'un jour.
trait CourseJour {

}

// Course par étapes.
struct CourseÉtapes<J : CourseJour> {
    etapes : Vec<J>
 }

 // Course par étapes 2.
/*
struct CourseÉtapes2 {
    etapes : Vec<CourseJour>
}
*/

// Course par étapes 3.
struct CourseÉtapes3<J : CourseJour > {
    etapes : Vec<Box<J>>
}
