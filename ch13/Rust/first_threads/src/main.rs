use std::thread;
use std::time::Duration;

fn main() {
   
    let thread_secondaire = thread::spawn(move || {

        for instant in 1..21 {
            println!("Instant {} depuis le thread secondaire", instant);
            thread::sleep((Duration::from_millis(10)));
        }
    });

    for instant in 1..11 {
        println!("Instant {} depuis le thread principal", instant);
        thread::sleep((Duration::from_millis(10)));
    }

    thread_secondaire.join().unwrap();
}
