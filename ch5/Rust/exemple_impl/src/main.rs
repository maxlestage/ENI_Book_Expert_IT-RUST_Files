struct Rectangle{
    largeur: f64,
    longueur: f64
}

impl Rectangle{

    pub fn new(la : f64, lo : f64) -> Rectangle {
        Rectangle{largeur : la, longueur : lo}
    }

    pub fn perimetre(&self) -> f64 {
        2.0 * (self.largeur + self.longueur)
    }

    pub fn aire(&self) -> f64 {
        self.largeur * self.longueur
    }

    pub fn obtenir_largeur_int(&self) -> i64{
        self.largeur.floor() as i64
    }

    pub fn obtenir_longueur_int(&self) -> i64{
        self.longueur.ceil() as i64
    }
}

fn main() {
    let lar = 5.5_f64;
    let lon = 6.4_f64;
    let rect = Rectangle::new(lar, lon);

    println!("Périmètre : {}", rect.perimetre());
    println!("Aire : {}", rect.aire());
    println!("Valeur entière largeur : {}", rect.obtenir_largeur_int());
    println!("Valeur entière longueur : {}", rect.obtenir_longueur_int());
}
