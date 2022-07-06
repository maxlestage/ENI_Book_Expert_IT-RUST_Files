use std::collections::VecDeque;



fn main() {

    let collection_1: VecDeque<i64> = VecDeque::new();
    println!("collection_1 - longueur : {}", collection_1.len());

    let collection_2: VecDeque<i64> = VecDeque::with_capacity(5);
    println!("collection_2 - longueur : {}", collection_2.len());

    let mut vecteur : Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
    println!("vecteur - capacité : {}", vecteur.capacity());

    let mut collection_3 = VecDeque::from(vecteur);
    println!("collection_3 - longueur : {}", collection_3.len());
    println!("collection_3 - capacité : {}", collection_3.capacity());

    collection_3.push_back(8);
    println!("collection_3  : {:?}", collection_3);

    collection_3.pop_back();
    println!("collection_3  : {:?}", collection_3);

    collection_3.push_front(8);
    println!("collection_3  : {:?}", collection_3);

    collection_3.pop_front();
    println!("collection_3  : {:?}", collection_3);
    
}
