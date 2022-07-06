use std::sync::mpsc::channel;
use std::thread;

fn main() {
    
    let (tx, rx) = channel();

    // Thread qui envoie.
    thread::spawn(move|| {
            
            let id = thread::current().id();
            println!("A - Je suis le thread qui envoie : {:?}.", id);

            let valeur = String::from("Coucou, ça va ?");
           
            tx.send(valeur).unwrap();

            // println!("{}.", valeur); // Test erreur de compilation.
        });

    // Thread qui reçoit, par ailleurs thread principal.
    let id = thread::current().id();
    println!("B - Je suis le thread qui reçoit : {:?}.", id);
    let message = rx.recv().unwrap();
    println!("Message reçu par B envoyé par A : {}", message);

} 
