use std::collections::LinkedList;

fn main() {
    
    let mut liste_1 : LinkedList<i64> = LinkedList::new();
    println!("liste_1 : {:?}", liste_1);

    let mut liste_2 = LinkedList::from([10, 20, 30, 40, 50]);
    println!("liste_2 : {:?}", liste_2);

    liste_2.push_back(5);
    println!("liste_2 : {:?}", liste_2);

    liste_1.push_front(55);
    println!("liste_1 : {:?}", liste_1);

    liste_2.append(&mut liste_1);
    println!("liste_2 : {:?}", liste_2);

    let mut iterateur = liste_2.iter();
    iterateur.next();
    iterateur.next();
    println!("iterateur : {:?}", iterateur);
}
