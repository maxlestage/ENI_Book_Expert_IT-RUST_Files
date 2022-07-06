struct JeuMesures {
    nom: String,
    mesures: Vec<i64>,
    annee : i64
}

impl Drop for JeuMesures {
    fn drop(&mut self) {
        println!("Drop de l'instance.");
    }
}

use std::ops::Deref;
impl Deref for JeuMesures {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.mesures
    }
}

impl Default for JeuMesures {
    fn default() -> JeuMesures {
        JeuMesures {
            nom : "Nom par défaut".to_string(),
            mesures: vec![],
            annee : 2022,
        }
    }
}

impl From<i64> for JeuMesures {
    fn from(aa: i64) -> Self {

        JeuMesures { annee: aa,
                     nom : "Nom par défaut".to_string(),
                     mesures: vec![]}
        }
}

fn main() {
    
    let jeu1 = JeuMesures { mesures : vec![1, 2, 3], nom : "Températures mesurées".to_string(), annee : 2022 };
    println!("{}", jeu1.nom);
    println!("Nombre de mesures : {}", jeu1.mesures.len());
    println!("Année : {}", jeu1.annee);

    // Non autorisé par le compilateur Rust.
    //jeu1.drop();

    // Déréférencement avec Deref.
    println!(" --- Usage du trait Deref --- ");
    println!("{:?}", *jeu1);
    let ii : i64 = (*jeu1)[0];
    println!("Valeur du premier élément du vecteur après déréférencement : {} ", ii);

    // Usage du trait Default.
    println!(" --- Usage du trait Default --- ");
    let jeu2 : JeuMesures = Default::default();
    println!("{}", jeu2.nom);
    println!("Nombre de mesures : {}", jeu2.mesures.len());
    println!("Année : {}", jeu2.annee);

    // Usage du trait From.
    println!(" --- Usage du trait From --- ");
    let jeu3 = JeuMesures::from(2021);
    println!("{}", jeu3.annee);

}