fn obtenir_type<T>(_: &T) {
    println!("Type : {}", unsafe {  std::any::type_name::<T>() });
}

// Quelques instanciations de vecteurs.
fn instanciations_vecteurs() {

        // Vecteur d'entiers.
        println!(" --- Vecteur d'entiers. --- ");
        let mut vecteur_entiers = vec![1, 2, 3, 4, 5];
        obtenir_type(&vecteur_entiers);
        println!("Longueur : {}", vecteur_entiers.len());
        println!("Capacité : {}", vecteur_entiers.capacity());
    
        // Vecteur d'entiers vide.
        println!(" --- Vecteur d'entiers vide. --- ");
        let mut vecteur_entiers__vide : Vec<i64> = vec![];
        obtenir_type(&vecteur_entiers__vide);
        println!("Longueur : {}", vecteur_entiers__vide.len());
        println!("Capacité : {}", vecteur_entiers__vide.capacity());
        
        // Vecteur de mots - type implicite.
        println!(" --- Vecteur de mots - type implicite. --- ");
        let mut vecteurs_de_mots_str = vec!["Paris", "Lyon", "Marseille"];
        obtenir_type(&vecteurs_de_mots_str);
        println!("Longueur : {}", vecteurs_de_mots_str.len());
        println!("Capacité : {}", vecteurs_de_mots_str.capacity());
    
        // Vecteur de String.
        println!(" --- Vecteur de String. --- ");
        let mut vecteurs_de_mots_String : Vec<String> = vec!["Paris".to_string(), "Lyon".to_string(), "Marseille".to_string()];
        obtenir_type(&vecteurs_de_mots_String);
        println!("Longueur : {}", vecteurs_de_mots_String.len());
        println!("Capacité : {}", vecteurs_de_mots_String.capacity());

        // Ajout 1 : "Lille".to_string().
        println!(" --- Ajout 1 : Lille. --- ");
        vecteurs_de_mots_String.push("Lille".to_string());
        println!("Longueur : {}", vecteurs_de_mots_String.len());
        println!("Capacité : {}", vecteurs_de_mots_String.capacity());

        // Ajout 2 : "Bordeaux".to_string().
        println!(" --- Ajout 2 : Bordeaux. --- ");
        vecteurs_de_mots_String.push("Bordeaux".to_string());
        println!("Longueur : {}", vecteurs_de_mots_String.len());
        println!("Capacité : {}", vecteurs_de_mots_String.capacity());

        // Ajout 3 : "Dijon".to_string().
        println!(" --- Ajout 3 : Dijon. --- ");
        vecteurs_de_mots_String.push("Dijon".to_string());
        println!("Longueur : {}", vecteurs_de_mots_String.len());
        println!("Capacité : {}", vecteurs_de_mots_String.capacity());

        // Ajout 4 : "Nantes".to_string().
        println!(" --- Ajout 4 : Nantes. --- ");
        vecteurs_de_mots_String.push("Nantes".to_string());
        println!("Longueur : {}", vecteurs_de_mots_String.len());
        println!("Capacité : {}", vecteurs_de_mots_String.capacity());

        // Accès à une tranche.
        println!(" --- Accès à une tranche. --- ");
        println!("Le vecteur : {:?}", vecteurs_de_mots_String);
        let tranche = &vecteurs_de_mots_String[2..4];
        println!("La tranche : {:?}", tranche);

        //let tranche_panique = &vecteurs_de_mots_String[2..12]; // Panique !

        println!("tranche[0] : {}", tranche[0]);
        println!("tranche[1] : {}", tranche[1]);
        //println!("tranche[0] : {}", tranche[2]); // Panique !

        let nouveau_vecteur = tranche.to_vec();
        obtenir_type(&nouveau_vecteur);
}

fn acces_vecteurs(){

    let mut vecteur_entiers : Vec<i64> = vec![1, 2, 3, 4, 5];
    obtenir_type(&vecteur_entiers);
    println!("Longueur : {}", vecteur_entiers.len());
    println!("Capacité : {}", vecteur_entiers.capacity());

    // Accès par référence.
    let element1_ref = &vecteur_entiers[0];
    println!("Le premier élément du vecteur : {}", element1_ref);

    for ii in 0..vecteur_entiers.len(){
        let element_courant = &vecteur_entiers[ii];
        println!("L'élément courant d'index {} : {}", ii, element_courant);
    }

    // Copie des valeurs.
    let element1_Copy = vecteur_entiers[0]; // Trait Copy
    println!("Copie du premier élément du vecteur (trait Copy) : {}", element1_Copy);

    let element1_Clone = vecteur_entiers[0].clone(); // Trait Clone
    println!("Copie du premier élément du vecteur (trait Clone) : {}", element1_Clone);
}

fn acces_avances_vecteurs(){

    let mut vecteur_entiers : Vec<i64> = vec![];
    
    let mut first = vecteur_entiers.first();
    println!("Premier élément : {:?}", first);

    let mut last = vecteur_entiers.last();
    println!("Dernier élément : {:?}", last);

    let mut get_2 = vecteur_entiers.get(2);
    println!("L'élément d'index 2 : {:?}", get_2);

    vecteur_entiers.push(10);
    vecteur_entiers.push(20);
    vecteur_entiers.push(40);

    first = vecteur_entiers.first();
    println!("Premier élément : {:?}", first);

    last = vecteur_entiers.last();
    println!("Dernier élément : {:?}", last);

    get_2 = vecteur_entiers.get(2);
    println!("L'élément d'index 2 : {:?}", get_2);

    let tranche_entiers = &vecteur_entiers[1..2];
    println!("La tranche : {:?}", tranche_entiers);

    first = tranche_entiers.first();
    println!("Premier élément (tranche) : {:?}", first);

    last = tranche_entiers.last();
    println!("Dernier élément (tranche) : {:?}", last);

    get_2 = tranche_entiers.get(2);
    println!("L'élément d'index 2 (tranche) : {:?}", get_2);

    let mut tranche_mut = [1, 2, 3];
    println!("La tranche (mutable) : {:?}", tranche_mut);

    let first_mut = tranche_mut.first_mut();
    println!("Premier élément (mutable) : {:?}", first_mut);

    let last_mut = tranche_mut.last_mut();
    println!("Dernier élément (mutable)) : {:?}", last_mut);

    let get_1_mut = tranche_mut.get_mut(1);
    println!("L'élément d'index 1 (mutable) : {:?}", get_1_mut);
    
    let valeur = get_1_mut.unwrap();
    *valeur = 4;
    println!("La tranche (mutable) après modification : {:?}", tranche_mut);
}

fn taille__vecteurs(){

    let mut vecteur : Vec<i64> = Vec::with_capacity(10);
    println!("Le vecteur : {:?}", vecteur);
    println!("Sa capacité : {}", vecteur.capacity());
    println!("Sa longueur : {}", vecteur.len());

    vecteur.reserve(11);
    println!("Réserve : {:?}", vecteur.capacity());

    vecteur.reserve_exact(21);
    println!("Réserve : {:?}", vecteur.capacity());

    vecteur.push(42);
    println!("Longueur avant shrink_to_fit : {:?}", vecteur.len());
    println!("Capacité avant shrink_to_fit: {:?}", vecteur.capacity());

    vecteur.shrink_to_fit();
    println!("Longueur après shrink_to_fit : {:?}", vecteur.len());
    println!("Capacité après shrink_to_fit : {:?}", vecteur.capacity());   
}

fn ajouts_retraits_vecteurs(){

    let mut vecteur_entiers : Vec<i64> = vec![10, 20, 30, 40, 50];
    println!("Le vecteur : {:?}", vecteur_entiers);
    
    vecteur_entiers.push(60);
    println!("Le vecteur : {:?}", vecteur_entiers);

    vecteur_entiers.pop();
    println!("Le vecteur : {:?}", vecteur_entiers);

    vecteur_entiers.insert(3, 70);
    println!("Le vecteur : {:?}", vecteur_entiers);

    vecteur_entiers.remove(4);
    println!("Le vecteur : {:?}", vecteur_entiers);

    // vecteur_entiers.insert(10, 80); // Panique.

    // vecteur_entiers.remove(20); // Panique.
}

fn ajouts_retraits_vecteurs_2(){

    let mut vecteur_entiers : Vec<i64> = vec![10, 20, 30, 40, 50];
    println!("Le vecteur : {:?}", vecteur_entiers);

    vecteur_entiers.resize(6, 15);
    println!("Le vecteur après resize : {:?}", vecteur_entiers);

    vecteur_entiers.truncate(4);
    println!("Le vecteur après truncate : {:?}", vecteur_entiers);

    let mut vecteur2 : Vec<i64> = vec![2, 2, 2, 2];
    vecteur_entiers.extend(vecteur2);
    println!("Le vecteur après extend : {:?}", vecteur_entiers);

    let mut vecteur3 : Vec<i64> = vec![3, 3, 3, 3];
    vecteur_entiers.append(&mut vecteur3);
    println!("Le vecteur après append : {:?}", vecteur_entiers);

    vecteur_entiers.dedup();
    println!("Le vecteur après dedup : {:?}", vecteur_entiers);

    vecteur_entiers.retain(|&x| x % 2 == 0);
    println!("Le vecteur après retain (valeurs paires) : {:?}", vecteur_entiers);
}

fn concat_split_vecteurs(){

    let mut vecteur_1 : Vec<i64> = vec![10, 20];
    let mut vecteur_2 : Vec<i64> = vec![30, 40];
    let mut vecteur_3 : Vec<i64> = vec![50, 60];

    let mut vecteur_entiers : Vec<i64> = [vecteur_1, vecteur_2, vecteur_3].concat();
    println!("Le vecteur après concat : {:?}", vecteur_entiers);

    let mut vecteur_11 : Vec<i64> = vec![10, 20];
    let mut vecteur_22 : Vec<i64> = vec![30, 40];
    let mut vecteur_33 : Vec<i64> = vec![50, 60];
    let mut vecteur_entiers_2 : Vec<i64> = [vecteur_11, vecteur_22, vecteur_33].join(&100);
    println!("Le vecteur après join : {:?}", vecteur_entiers_2);

    // On découpe à l'index 4.
    let iter_split_at = vecteur_entiers_2.split_at(4);
    println!("Tranche gauche : {:?}", iter_split_at.0);
    println!("Tranche droite : {:?}", iter_split_at.1);
}

fn tris_vecteurs(){

    let mut vecteur_entiers : Vec<i64> = vec![20, 10, 40, 30, 50, 60, 25, 10];
    
    vecteur_entiers.reverse();
    println!("Le vecteur après reverse : {:?}", vecteur_entiers);

    vecteur_entiers.sort();
    println!("Le vecteur après sort : {:?}", vecteur_entiers);

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Individu {
        prenom: String,
        age: i64
    }

    impl Individu {
        pub fn creer(prenom: String, age: i64) -> Self {
            Individu {
                prenom,
                age
            }
        }
    }

    let mut individus = vec![
        Individu::creer("Arthur".to_string(), 14),
        Individu::creer("Hector".to_string(), 10),
        Individu::creer("Clément".to_string(), 24),
        Individu::creer("Marie".to_string(), 22),
        Individu::creer("Léna".to_string(), 21),
        Individu::creer("Coline".to_string(), 18),        
    ];

    // Tri par âge.
    individus.sort_by(|xx, yy| xx.age.cmp(&yy.age));
    println!("Le vecteur après tri par âge : {:?}", individus);

    // Tri par longueur du prénom.
    individus.sort_by(|xx, yy| xx.prenom.len().cmp(&yy.prenom.len()));
    println!("Le vecteur après tri par longueur du prénom : {:?}", individus);
}

fn recherches_vecteurs(){

    let mut vecteur_entiers : Vec<i64> = vec![20, 10, 40, 30, 50, 60, 25, 10];

    let resultat = vecteur_entiers.binary_search(&40);
    println!("Résultat de la recherche de 40 : {:?}", resultat);

    let resultat_non = vecteur_entiers.binary_search(&408);
    println!("Résultat de la recherche de 408 : {:?}", resultat_non);

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Individu {
        prenom: String,
        age: i64
    }

    impl Individu {
        pub fn creer(prenom: String, age: i64) -> Self {
            Individu {
                prenom,
                age
            }
        }
    }

    let mut individus = vec![
        Individu::creer("Arthur".to_string(), 14),
        Individu::creer("Hector".to_string(), 10),
        Individu::creer("Clément".to_string(), 24),
        Individu::creer("Marie".to_string(), 22),
        Individu::creer("Léna".to_string(), 21),
        Individu::creer("Coline".to_string(), 18),        
    ];

    let resultat_individu = individus.binary_search_by(|xx| xx.age.cmp(&22));
    println!("Résultat de la recherche de l'âge égal à 22 : {:?}", resultat_individu);

    let resultat_individu_non = individus.binary_search_by(|xx| xx.age.cmp(&34));
    println!("Résultat de la recherche de l'âge égal à 34 : {:?}", resultat_individu_non);
}

fn main() {

    // instanciations_vecteurs();
    
    // acces_vecteurs();
    
    // acces_avances_vecteurs();
    
    // taille__vecteurs();
    
    // ajouts_retraits_vecteurs();

    // ajouts_retraits_vecteurs_2();

    // concat_split_vecteurs();

    // tris_vecteurs();

    recherches_vecteurs();
}
