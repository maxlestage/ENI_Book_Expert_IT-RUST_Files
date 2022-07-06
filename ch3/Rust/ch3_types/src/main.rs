#![allow(non_snake_case)]

fn main() {

    tuple_exemple();
    struct_exemple();
}

fn tuple_exemple()
{
    let tuple_exemple = (1u64, 2u32, 3u8, 1.2f32, 3.4f64, -4i64, 'R', false);
    
    println!("{}", tuple_exemple.1);
    println!("{}", tuple_exemple.5);
    //println!("{}", tuple_exemple.10);
}

struct Cycliste 
{    
   en_cours: bool,
   nom: String,
   pays: String,
   dernierClassement: u64
}

fn struct_exemple()
{
    let mut c = Cycliste { en_cours: false, nom: String::from("Jean Forestier"), pays: String::from("France"), dernierClassement: 4};
    println!("{}", c.en_cours);
    println!("{}", c.nom);
    println!("{}", c.pays);
    println!("{}", c.dernierClassement);

    c.dernierClassement = 1;
    println!("{}", c.dernierClassement);

    c = Cycliste { dernierClassement: 2, .. c };
    println!("{}", c.dernierClassement);

    // Tuple Struct
    struct StructTuple(i32, f64, bool);
    let exemple = StructTuple(0, 3.14, false);
    println!("{}", exemple.1);

    // Définition d'un type avec "Tuple Struct"
    struct Metres(f64);
    let exempleLongueur = Metres(3.5);
    let Metres(valeur) = exempleLongueur;
    println!("{}", valeur);
}