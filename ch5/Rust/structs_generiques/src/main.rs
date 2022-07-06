pub struct VecteurEntiers{
    vec : Vec<i64>
}

impl VecteurEntiers{
     
    pub fn creer() -> VecteurEntiers{
        VecteurEntiers { vec: Vec::new() }
    }
    
    pub fn ajouter(&mut self, elt: i64)
    {
        self.vec.push(elt);
    }

    pub fn obtenir_longeur(&mut self) -> usize
    {
        self.vec.len()
    }

    pub fn obtenir_element(&mut self, indice: usize) -> i64
    {
        if indice < self.vec.len()  {
            self.vec[indice]
        }
        else {
             -1   
        }
    }
}

pub struct VecteurGenerique<T>{
    vec : Vec<T>
}

impl<T> VecteurGenerique<T>{
     
    pub fn creer() -> VecteurGenerique<T>{
        VecteurGenerique { vec: Vec::new() }
    }
    
    pub fn ajouter(&mut self, elt: T)
    {
        self.vec.push(elt);
    }

    pub fn obtenir_longeur(&mut self) -> usize
    {
        self.vec.len()
    }
}

fn main() {
    
    println!("--- Exemple vecteur d'entiers ---");

    let mut mon_vecteur = VecteurEntiers::creer();
    println!("Taille courante du vecteur : {}", mon_vecteur.obtenir_longeur());
    mon_vecteur.ajouter(1);
    mon_vecteur.ajouter(2);
    mon_vecteur.ajouter(3);
    println!("Taille courante du vecteur : {}", mon_vecteur.obtenir_longeur());
    println!("Quel est l'élément d'indice 2 : {}", mon_vecteur.obtenir_element(2));
    println!("On demande un indice au-delà de la taille du vecteur : {}", mon_vecteur.obtenir_element(42));

    println!("--- Exemple vecteur générique ---");
    let mut mon_vecteur_generique = VecteurGenerique::<f64>::creer();
    mon_vecteur_generique.ajouter(5.0);
    mon_vecteur_generique.ajouter(4.9);
    mon_vecteur_generique.ajouter(4.8);
    mon_vecteur_generique.ajouter(4.7);
    println!("Taille courante du vecteur : {}", mon_vecteur_generique.obtenir_longeur());

    println!("--- Exemple vecteur générique 2 ---");
    let mut mon_vecteur_generique_2 = VecteurGenerique::creer();
    mon_vecteur_generique_2.ajouter("Hector");
    mon_vecteur_generique_2.ajouter("Arthur"); 
    mon_vecteur_generique_2.ajouter("Sophie");  
    println!("Taille courante du vecteur : {}", mon_vecteur_generique_2.obtenir_longeur());
}
