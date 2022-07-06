// Version avec trait // use std::cmp::Ordering;

// Version avec trait //#[derive(Eq, Debug)]
#[derive(Debug)]
struct Individu {

    prenom : String,
    age : i32
}

impl Individu {

    pub fn creer(prenom : String, age : i32) -> Individu {
        Individu { prenom : prenom, age : age }
    }
}

/* // Version avec trait //
impl Ord for Individu {
    fn cmp(&self, autre: &Self) -> Ordering {
        self.age.cmp(&autre.age)
    }
}

impl PartialOrd for Individu {
    fn partial_cmp(&self, autre: &Self) -> Option<Ordering> {
        Some(self.cmp(autre))
    }
}

impl PartialEq for Individu {
    fn eq(&self, autre: &Self) -> bool {
        self.age == autre.age
    }
}

*/ 

fn fn_closure_move_keyword(){

    let vecteur = vec![5, 10, 15, 20];
    let closure = move || println!("Closure propriétaire par valeur de : {:?}", vecteur);
    closure();

    let vecteur_cible = vec![10, 20, 30, 40, 50];
    let comparaison_vecteur_cible = move |vec| vec == vecteur_cible;

    let vecteur_1 = vec![10, 20, 30, 45, 50];
    println!("Comparaison vecteur_1 : {}", comparaison_vecteur_cible(vecteur_1));

    let vecteur_2 = vec![10, 20, 30, 40, 50];
    println!("Comparaison vecteur_2 : {}", comparaison_vecteur_cible(vecteur_2));
}

fn fn_closure_autre_exemple(){

    let valeur_numerique = 4;
 
    let chaine_de_caractères = "Benoît".to_string();

    let affichage = || print!(" Chaîne de caractères : {}", chaine_de_caractères); 
    affichage();

    let affichage_num = || print!(" - valeur numérique : {}", valeur_numerique); 
    affichage_num();
}

fn tri(individus : &mut Vec<Individu>){
    individus.sort_by_key(| individu | individu.age);
}

fn fn_closure_relation_ordre(){

    let hector = Individu::creer("Hector".to_string(), 10);
    let coline = Individu::creer("Coline".to_string(), 18);
    let arthur = Individu::creer("Arthur".to_string(), 14);
    let lena = Individu::creer("Léna".to_string(), 21);

    let mut individus : Vec<Individu> = vec![ arthur, coline, hector, lena];
    println!("Avant tri : {:?}", individus);

    // Version avec trait // individus.sort();
    // Version avec trait // println!("Après tri : {:?}", individus);

    tri(&mut individus);
    println!("Après tri avec closure : {:?}", individus);
}

fn fn_premiere_closure() {

    let est_un_nombre_pair = |x : i64| x % 2 == 0;

    let quatorze : i64 = 14;
    let treize : i64 = 13;

    println!(" 14 est un nombre pair ? {}", est_un_nombre_pair(quatorze));
    println!(" 13 est un nombre pair ? {}", est_un_nombre_pair(treize));
    println!(" 12 est un nombre pair ? {}", est_un_nombre_pair(12));
}

fn main() {

    // fn_premiere_closure();
    
    // fn_closure_relation_ordre();

    // fn_closure_autre_exemple();

    fn_closure_move_keyword();
}
