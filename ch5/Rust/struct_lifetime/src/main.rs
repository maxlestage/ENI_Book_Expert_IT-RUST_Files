//struct Exemple {
//    ii: &i64
//}

struct Exemple<'a> {
    ii: &'a i64
}

struct Exemple2{
    ii: i64
}

struct Exemple3{
    ii: &'static i64
}

fn main() {
    let valeur: i64 = 5;
    let exemple = Exemple{ii: &valeur};
    println!("Affichage de ii - exemple 1 : {}", exemple.ii);

    static valeur_2 : Exemple2 = Exemple2 { ii : 3};
    let ref_static_valeur2: &'static Exemple2 = &valeur_2;
    println!("Affichage de ii - exemple 2 : {}", ref_static_valeur2.ii);

    static valeur_3 : i64 = 2;
    let exemple_3 = Exemple3{ii: &valeur_3};
    println!("Affichage de ii - exemple 3 : {}", exemple_3.ii);
}
