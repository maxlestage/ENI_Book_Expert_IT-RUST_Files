extern crate rayon;
use rayon::prelude::*;
use std::thread;
use std::time::Duration;

fn fonction_1() {

    for instant in 1..21 {
        println!("Instant {} depuis le thread secondaire", instant);
        thread::sleep((Duration::from_millis(10)));
    }
}

fn fonction_2() {

    for instant in 1..11 {
        println!("Instant {} depuis le thread principal", instant);
        thread::sleep((Duration::from_millis(10)));
    }
}

fn first_prime_number_after(ii : u64){

    use primes::{Sieve, PrimeSet};
    let mut pset = Sieve::new();
    
    let (ix, n) = pset.find(ii);
    println!("Le nombre premier {} est le premier après {} et est le {}e nombre premier depuis 2.", n, ii, ix);
}


fn par_iter_exemple(){

    let vecteur = vec!(4, 5, 25, 132, 172, 994, 1839, 9876);

    vecteur.into_par_iter().for_each(|x|  first_prime_number_after(x));
}

fn main() {
    // let (variable_1, variable_2) = rayon::join(fonction_1, fonction_2);

    par_iter_exemple();
}
