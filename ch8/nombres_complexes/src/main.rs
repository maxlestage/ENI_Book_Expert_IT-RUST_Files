// Nombre complexe.
struct Complexe{
    re: f64,
    im: f64
}

// Trait Display pour Complexe.
use std::fmt;
impl fmt::Display for Complexe {
    fn fmt(&self, exemple: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(exemple, "z = {} + {} * i", self.re, self.im)
    }
}

// Opérateur addition pourComplexe.
use std::ops::Add;
impl Add for Complexe {
    type Output = Complexe;
    fn add(self, z2 : Self) -> Self {
        Complexe {re : self.re + z2.re, im : self.im + z2.im }
    }
}

// Opérateur soustraction pour Complexe.
use std::ops::Sub;
impl Sub for Complexe {
    type Output = Complexe;
    fn sub(self, z2 : Self) -> Self {
        Complexe {re : self.re - z2.re, im : self.im - z2.im }
    }
}

// Opérateur de comparaison pour Complexe.
use std::cmp::PartialEq;
impl PartialEq for Complexe {
    fn eq(&self, z2 : &Self) -> bool {
        self.re == z2.re && self.im == z2.im
    }
}

fn main() {

    let z1 = Complexe{re : 5.0, im: 4.0};
    println!("{}", z1);
    let z2 = Complexe{re : 5.0, im: 3.0};
    
    let z_addition = z1 + z2;
    println!("{}", z_addition);

    let z3 = Complexe{re : -3.0, im: 4.0};
    let z4 = Complexe{re : 1.0, im: 2.0};
    let z_soustraction = z3 - z4;
    println!("{}", z_soustraction);

    let z5 = Complexe{re : -3.0, im: 2.0};
    let z6 = Complexe{re : -3.0, im: 2.0};
    println!("z5 == z6 : {}", z5 == z6);
}
