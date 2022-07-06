#![allow(non_snake_case)]

fn main() {
    exempleTableau();
    exempleVecteur();
    exempleSlice();
    exempleString();
}

fn exempleTableau(){

    println!("----------");

    let mut tableauEntiers : [u32; 8] = [2, 1, 3, 5, 6, 4, 7, 8];
    println!("4e case du tableau d'entiers : {}", tableauEntiers[3]); 
    tableauEntiers.sort();
    for ii in 0..tableauEntiers.len() {
        println!("Chaque case après tri : {}", tableauEntiers[ii]);
    }

    println!("----------");

    let tableauFlottants = [3.4, 2.0, 5.1];
    println!("Taille du tableau de flottants : {}", tableauFlottants.len());
    println!("3e case du tableau de flottants : {}", tableauFlottants[2]);
}

fn exempleVecteur(){

    let mut vecteur1 = vec![1, 2, 3, 4];
    vecteur1.push(5);
    println!("Longueur : {}", vecteur1.len());
    vecteur1.pop();
    println!("Longueur : {}", vecteur1.len());

    let vecteur1_iter = vecteur1.iter();
    for valeur in vecteur1_iter {
        println!("Valeur : {}", valeur);
    }

    println!("----------");

    let mut vecteur2 = Vec::new();
    vecteur2.push("aaa");
    vecteur2.push("bbb");
    vecteur2.push("bbb");

    // collect()
    let mut vecteur3 : Vec<i32> = (10..15).collect();
    let vecteur3_iter = vecteur3.iter();
    for valeur in vecteur3_iter {
        println!("Valeur : {}", valeur);
    }
    vecteur3.pop();
    println!("Longueur : {}", vecteur3.len());
    println!("Capacité : {}", vecteur3.capacity());
}

fn exempleSlice(){

    let tableau = [1, 2, 3, 4, 5];
    let tranche = &tableau[0..2];

    println!("Longueur tranche : {:?}", tranche.len());
    println!("Tranche : {:?}", tranche);
    println!("Tableau : {:?}", tableau);

    println!("----------");

    let mut vecteur = Vec::new();
    vecteur.push("A");
    vecteur.push("B");
    vecteur.push("C");

    let trancheVecteur = &vecteur[0..2];
    println!("Longueur trancheVecteur : {:?}", trancheVecteur.len());
    println!("Tranche : {:?}", trancheVecteur);
    println!("Vecteur : {:?}", vecteur);
}

fn exempleString(){

    let mut chaine1 = String::new();
    chaine1.push('a');
    chaine1.push('b');
    chaine1.push('c');
    println!("{}", chaine1);

    let chaine2 = String::from("def");
    println!("{}", chaine2);

    let chaine3 = "ghi".to_string();
    println!("{}", chaine3);

    let trancheChaine3 = &chaine3[1..];
    println!("{}", trancheChaine3);

    println!("Inclut 'hi' ? : {}", chaine3.contains("hi"));

    let chaine4 = chaine3.replace("i", "L");
    println!("Après remplacement : {}", chaine4);

    for lettre in chaine4.split("h"){
        println!("Lettre : {}", lettre);
    }
}