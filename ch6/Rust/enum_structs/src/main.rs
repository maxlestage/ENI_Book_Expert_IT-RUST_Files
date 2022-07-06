#![allow(unused)]

#[derive(Debug)]
enum Exemple{

    A,
    B(Option<String>),
    C {
        champ_nomme_1 : String,
        champ_nomme_2 : i64
    }
}

fn main() {
    let a : Exemple = Exemple::A;
    let b : Exemple = Exemple::B(Some("test B".to_string()));
    let c : Exemple = Exemple::C{ champ_nomme_1 : "test B".to_string(), champ_nomme_2 : 23};

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
