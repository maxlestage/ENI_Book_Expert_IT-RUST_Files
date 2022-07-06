use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    
    let (tx, rx) = channel();

    // Thread qui envoie.
    thread::spawn(move|| {
            
            let  vecteur = vec![ 
                String::from("Coucou"),
                String::from(", "),
                String::from("ça "),
                String::from("va "),
                String::from("?")
            ];    
            
            let id = thread::current().id();
            println!("A - Je suis le thread qui envoie : {:?}.", id);

            let intervalle = Duration::from_secs(2);

            for mot in vecteur{
                tx.send(mot).unwrap();

                thread::sleep(intervalle); // Deux secondes avant le prochain mot envoyé.
            }            
        });

    // Thread qui reçoit, par ailleurs thread principal.
    let id = thread::current().id();
    println!("B - Je suis le thread qui reçoit : {:?}.", id);
    
    for mot_recu in rx {
        println!("Mot reçu par B envoyé par A : {}", mot_recu);
    }
} 
