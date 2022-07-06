#[derive(PartialEq, Clone)]
struct Exemple {
    xx: i32,
    yy: String,
}

use std::fmt;
impl fmt::Display for Exemple {
    fn fmt(&self, exemple: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(exemple, "Instance : {} - {}", self.xx, self.yy)
    }
}

fn main() {
    let a = Exemple{xx: 5, yy: "aa".to_string()};
    let b = Exemple{xx: 5, yy: "bb".to_string()};

    if a == b {
        println!("Les deux instances sont égales.");
    }
    else {
        println!("Les deux instances ne sont sont égales.");
    }

    let c = b.clone();
    println!("Instance après clone : {}", c);
}
