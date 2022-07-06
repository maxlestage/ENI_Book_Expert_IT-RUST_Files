use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {

    let compteur = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for _ in 0..10 {
        let compteur_reference = Arc::clone(&compteur);
        let thread_courant = thread::spawn(move || {
            let mut num = compteur_reference.lock().unwrap();
            let id = thread::current().id();
            *num += 1;
            println!("Compteur à {} incrémenté par {:?}.", num, id);
        });
        threads.push(thread_courant);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Situation courante du compteur : {}.", *compteur.lock().unwrap());
}