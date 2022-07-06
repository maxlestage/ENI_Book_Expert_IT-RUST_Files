#![allow(unused)]

#[derive(Debug)]
enum Departement{
    Ain = 1,
    Aisne,
    Allier,
    AlpesDeHauteProvence,
    HautesAlpes,
    AlpesMaritimes,
    Ardeche,
    Ardennes,
    Ariege
}

fn reste_division(numerateur: i64, denominateur: i64) -> Option<i64> {
    if denominateur == 0 {
        None
    } else {
        Some(numerateur % denominateur)
    }
}

fn obtenir_valeur_numerique(valeur: Option<i64>) -> i64 {
    let mut ret : i64 = -1;
    match valeur {
        Some(p) => ret = p,
        None => println!("obtenir_valeur_numerique : pas de valeur (division par zéro)"),
    }
    ret
}

fn i64_vers_enum(valeur: i64) -> Option<Departement> {

    match valeur {
        1 => Some(Departement::Ain),
        2 => Some(Departement::Aisne),
        3 => Some(Departement::Allier),
        4 => Some(Departement::AlpesDeHauteProvence),
        5 => Some(Departement::HautesAlpes),
        6 => Some(Departement::AlpesMaritimes),
        7 => Some(Departement::Ardeche),
        8 => Some(Departement::Ardennes),
        9 => Some(Departement::Ariege),                       
        _ => None,
    }
}

impl Departement{

    fn conversion_i64(self) -> i64{
        let ret = self as i64;
        ret
    }

    fn conversion_str(self) -> &'static str {
        match self {
            Departement::Ain => "Ain",
            Departement::Aisne => "Aisne",
            Departement::Allier => "Allier",
            Departement::AlpesDeHauteProvence => "Alpes-de-Haute-Provence",
            Departement::HautesAlpes => "Hautes-Alpes",
            Departement::AlpesMaritimes => "Alpes-Maritimes",
            Departement::Ardeche => "Ardèche",
            Departement::Ardennes => "Ardennes",
            Departement::Ariege => "Ariège", 
        }                    
    }
}

fn main() {

    let ain : Departement = Departement::Ain;
    println!("{:?}", ain);
    
    let ain2 : i64 = Departement::Ain as i64;
    println!("{}", ain2);

    let resultat_reste_1 = reste_division(13, 9);
    println!("{:?}", resultat_reste_1);

    let resultat_reste_2 = reste_division(13, 0);
    println!("{:?}", resultat_reste_2);

    println!("Valeur entière : {}", obtenir_valeur_numerique(resultat_reste_1));
    println!("Valeur entière : {}", obtenir_valeur_numerique(resultat_reste_2));

    let departement_1 : Option<Departement> = i64_vers_enum(5);
    println!("Département correspondant au numéro 5 : {:?}", departement_1);

    let departement_135 : Option<Departement> = i64_vers_enum(135);
    println!("Département correspondant au numéro 135 : {:?}", departement_135);
    
    let aisne : Departement = Departement::Aisne;
    let aisne_i64 : i64 = aisne.conversion_i64();
    println!("Utilisation d'une méthode d'énumération : {}", aisne_i64);

    let alpesmaritimes : Departement = Departement::AlpesMaritimes;
    let alpesmaritimes_str = alpesmaritimes.conversion_str();
    println!("Méthode d'énumération utilisation un match : {}", alpesmaritimes_str);

}
