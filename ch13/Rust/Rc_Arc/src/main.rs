use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn Rc_usage() {
    let ma_valeur  : Rc<String> = Rc::new("Paris".to_string());

    println!("longueur de la chaîne : {}", ma_valeur.len());

    // Erreur volontaire de compilation : Rc immutable.
    //ma_valeur.push_str(" est la capitale de la France");

    let reference_1 : Rc<String> = ma_valeur.clone();
    println!("Compteur de références : {}", Rc::strong_count(&ma_valeur));

    let reference_2 : Rc<String> = ma_valeur.clone();
    println!("Compteur de références : {}", Rc::strong_count(&ma_valeur));

    let reference_3 : Rc<String> = ma_valeur.clone();
    println!("Compteur de références : {}", Rc::strong_count(&ma_valeur));

    {
        let reference_4 : Rc<String> = ma_valeur.clone();
        println!("Compteur de références : {}", Rc::strong_count(&ma_valeur));
    }
    println!("Compteur de références : {}", Rc::strong_count(&ma_valeur));

 
    drop(ma_valeur);

    println!("Référence 1 : {}", reference_1);
    println!("Référence 1 : {}", reference_2);
    println!("Référence 1 : {}", reference_3);
}

fn Arc_usage() {

    // Vecteur d'entiers.
    let mut vecteur : Vec<i64> = vec![];
    for ii in 0..1000 {
        vecteur.push(ii);
    }
     
    // Trois références sur le vecteur d'entiers.
    let vecteur_reference_1 : Arc<Vec<i64>> = Arc::new(vecteur);
    let vecteur_reference_2 : Arc<Vec<i64>> = vecteur_reference_1.clone();
    let vecteur_reference_3 : Arc<Vec<i64>> = vecteur_reference_1.clone();
     
    // Vecteur qui contient les trois références.
    let mut vecteur_references : Vec<Arc<Vec<i64>>> = vec![];
    vecteur_references.push(vecteur_reference_1);
    vecteur_references.push(vecteur_reference_2);
    vecteur_references.push(vecteur_reference_3);

     // Lancement d'un thread par référence.
    let mut threads = vec![];
    let mut index : i64 = 1;
    for reference in vecteur_references {
        threads.push(thread::spawn(move || {
             
            let id = thread::current().id();
             println!("Longueur du vecteur d'entiers 'vecteur' via la référence {} du thread {:?}: {}", index, id, reference.len());
             

        }));
        index = index + 1;
    }
     
     // On attend la fin de tous les threads.
     for thread in threads {
        let ret = thread.join();
     }
    
}

fn main() {
    
    // Rc_usage();
    Arc_usage();
}
