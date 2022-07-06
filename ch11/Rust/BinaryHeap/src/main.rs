use std::collections::BinaryHeap;

fn fn_tas_binaire_min(){

    println!(" --- Tas binaire-min. ---");
    
    use std::cmp::Reverse;

    let mut tas_binaire_min = BinaryHeap::from([Reverse(55), 
                                                Reverse(23), 
                                                Reverse(12), 
                                                Reverse(34), 
                                                Reverse(1), 
                                                Reverse(2), 
                                                Reverse(4), 
                                                Reverse(3), 
                                                Reverse(2), 
                                                Reverse(8), 
                                                Reverse(65), 
                                                Reverse(23), 
                                                Reverse(24), 
                                                Reverse(12)]);

    println!("Représentation de tas_binaire_min : {:?}", tas_binaire_min);

    // On ajoute quelques valeurs.
    tas_binaire_min.push(Reverse(24));
    tas_binaire_min.push(Reverse(5));
    println!("Représentation de tas_binaire_min : {:?}", tas_binaire_min);

    // Puis on en retire une.println!
    tas_binaire_min.pop();
    println!("Représentation de tas_binaire_min : {:?}", tas_binaire_min);

    // peek().
    let minimum = tas_binaire_min.peek();
    println!("Valeur minimum du tas : {:?}", minimum);
}

fn main() {

    let mut tas_binaire_max = BinaryHeap::new();
    tas_binaire_max.push(1);
    tas_binaire_max.push(5);
    tas_binaire_max.push(2);
    tas_binaire_max.push(23);
    tas_binaire_max.push(52);
    tas_binaire_max.push(71);
    tas_binaire_max.push(2);
    tas_binaire_max.push(3);
    tas_binaire_max.push(99);
    
    println!("Représentation de tas_binaire_max : {:?}", tas_binaire_max);

    for element in &tas_binaire_max {
        println!("Affichage de l'élément courant : {}", element);
    }

    let maximum = tas_binaire_max.peek();
    println!("Valeur maximum du tas : {:?}", maximum);

    // On retire des valeurs.
    tas_binaire_max.pop();
    tas_binaire_max.pop();
    println!("Représentation de tas_binaire_max : {:?}", tas_binaire_max);

    // On vide complètement le tas binaire-max.
    tas_binaire_max.clear();
    println!("Représentation de tas_binaire_max : {:?}", tas_binaire_max);

    // Tas binaire-min.
    fn_tas_binaire_min();
}
