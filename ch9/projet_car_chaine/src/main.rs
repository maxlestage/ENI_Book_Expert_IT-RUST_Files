fn fonction_char() {
        // Des caractères.
        let quatre = '4';
        println!("'4' est numérique ? : {}", quatre.is_numeric());
    
        let a = 'a';
        println!("'a' est alphabétique ? : {}", a.is_alphabetic());
    
        let etoile = '*';
        println!("'*' est alphénumérique ? : {}", etoile.is_alphanumeric());
    
        let espace = ' ';
        println!("Le caractère 'espace' est est un espace ? : {}", espace.is_whitespace());
        println!("Le caractère 'espace' est est un caractère de contrôle ? : {}", espace.is_control());
    
        let passage_ligne = '\n';
        println!("Le caractère 'slash n' est est un espace ? : {}", passage_ligne.is_whitespace());
        println!("Le caractère 'slash n' est est un caractère de contrôle ? : {}", passage_ligne.is_control());
    
        // Casse des caractères.
        println!("'a' est casse basse ? : {}", a.is_lowercase());
        let mut A = a.to_uppercase();
        println!("Après passage en majuscule : {}", A);
        A.next();
        println!("Itération de l'itérateur : {}", A);
    
        // Conversion entiers.
        let mut valeur_u64 = quatre as u64;
        let mut valeur_u32 = quatre as u32;
        let mut valeur_u8 = quatre as u8;
        let mut valeur_i8 = quatre as i8;
    
        println!("valeur_u64 = {} ", valeur_u64);
        println!("valeur_u32 = {} ", valeur_u32);
        println!("valeur_u8 = {} ", valeur_u8);
        println!("valeur_u8 = {} ", valeur_i8);
        println!("Addition valeur_u8 + 1 = {} ", valeur_i8 + 1);
    
        // Conversion depuis des entiers.
        let ch : Option<char> = char::from_u32(valeur_u32);
        println!("ch : {:?} ", ch);
}
fn fonction_str() {
    let valeur_str = "bonjour";
    println!("valeur_str : {}", valeur_str);

    let valeur_str_2 : &'static str = "BONJOUR";
    println!("valeur_str_2 : {}", valeur_str_2);

    let valeur_str_3 = valeur_str_2.to_owned();
    println!("valeur_str_3 : {}", valeur_str_3);

    println!("&valeur_str_3[..3] : {}", &valeur_str_3[..3]);
    println!("&valeur_str_3[3..] : {}", &valeur_str_3[3..]);
    println!("&valeur_str_3[3..4] : {}", &valeur_str_3[3..4]);
    println!("valeur_str_3[3..4].len() : {}", valeur_str_3[..3].len());
    println!("valeur_str_3[3..4].contains('o') : {}", valeur_str_3[..3].contains("o"));

    // Opérations diverses.
    println!(" --- Opérations diverses ---");
    let mut str = "Je joue au ballon";
    let resultat_split = str.split(' ');
    for s in resultat_split{ // Itérateur.
        println!("{}", s);
    }
    str = "    Je joue au ballon.   ";
    let resultat_trim = str.trim();
    println!("{}", resultat_trim);
}

fn fonction_String() {

    let mut nouvelle = String::new();   
    println!("Chaîne de caractères 'nouvelle' : {}", nouvelle);
    println!("Taille courante de la chaîne de caractères nommée 'nouvelle' : {}", nouvelle.len());
    println!("Capacité de la chaîne de caractères nommée 'nouvelle' : {}", nouvelle.capacity());

    let mut nouvelle2 = String::with_capacity(5);   
    println!("Chaîne de caractères 'nouvelle2' : {}", nouvelle2);
    println!("Taille courante de la chaîne de caractères nommée 'nouvelle2' : {}", nouvelle2.len());
    println!("Capacité de la chaîne de caractères nommée 'nouvelle2' : {}", nouvelle2.capacity());

    let mut nouvelle3 : String = "nouvelle3".to_string(); 
    println!("Chaîne de caractères 'nouvelle3' : {}", nouvelle3);
    println!("Taille courante de la chaîne de caractères nommée 'nouvelle3' : {}", nouvelle3.len());
    println!("Capacité de la chaîne de caractères nommée 'nouvelle3' : {}", nouvelle3.capacity());
}

fn fonction_String_outils(){

    // 1.
    println!("--- 1. ---");
    let mut chaine = String::with_capacity(2);
    println!("Capacité : {}", chaine.capacity());
    println!("Taille : {}", chaine.len());
    println!("Vide ? : {}", chaine.is_empty());

    // 2.
    println!("--- 2. ---");
    chaine.push('B');
    println!("Capacité : {}", chaine.capacity());
    println!("Taille : {}", chaine.len());
    println!("Vide ? : {}", chaine.is_empty());
       
    // 3.
    println!("--- 3. ---");
    chaine = "Benoît".to_string();
    println!("Capacité : {}", chaine.capacity());
    println!("Taille : {}", chaine.len());
    println!("Vide ? : {}", chaine.is_empty());

        // 4.
        println!("--- 4. ---");
        chaine.pop();
        println!("Capacité : {}", chaine.capacity());
        println!("Taille : {}", chaine.len());
        println!("Vide ? : {}", chaine.is_empty());        
}

fn fonction_String_insertion(){
    
    // 1.
    println!("--- 1 ---");
    let mut ma_chaine = "Paris".to_string(); // String
    let lyon = "-Lyon"; // &str
    ma_chaine.push_str(&lyon[..]);
    println!("{}", ma_chaine);
    println!("Capacité : {}", ma_chaine.capacity());
    println!("Taille : {}", ma_chaine.len());   
    
    // 2.
    println!("--- 2 ---");
    let marseille = "-Marseille"; // &str
    ma_chaine.insert_str(5, &marseille[..]);
    println!("{}", ma_chaine);
    println!("Capacité : {}", ma_chaine.capacity());
    println!("Taille : {}", ma_chaine.len());  
    
    // 3.
    println!("--- 3 ---");
    let lille = "-Lille".to_string(); // String
    ma_chaine += &lille;
    println!("{}", ma_chaine);
    println!("Capacité : {}", ma_chaine.capacity());
    println!("Taille : {}", ma_chaine.len()); 
}

fn fonction_String_suppression(){

        // 1.
        println!("--- 1 ---");
        let mut ma_chaine = "Paris".to_string(); // String
        ma_chaine.clear();
        println!("{}", ma_chaine);
        println!("Capacité : {}", ma_chaine.capacity());
        println!("Taille : {}", ma_chaine.len()); 

        // 2.
        println!("--- 2 ---");
        ma_chaine = "Paris".to_string(); // String
        ma_chaine.remove(3);
        println!("{}", ma_chaine);
        println!("Capacité : {}", ma_chaine.capacity());
        println!("Taille : {}", ma_chaine.len()); 

        // 3.
        println!("--- 3 ---");
        let iterateur = ma_chaine.drain(1..3);
        let extraction = iterateur.collect::<String>();
        println!("extraction : {}", extraction);
        println!("ma_chaine : {}", ma_chaine);
        println!("Capacité : {}", ma_chaine.capacity());
        println!("Taille : {}", ma_chaine.len()); 
}

fn fonction_String_recherche_remplacement(){


    // 1. find. Citation de Victor Hugo.
    println!("--- 1 ---");
    let chaine = "Il vient une heure où protester ne suffit plus : après la philosophie, il faut l’action.".to_string();
    let resultat_heure = chaine.find("heure");
    println!("{:?}", resultat_heure);
    let mut ii_resultat_heure : i64 = -1;
    match resultat_heure {
            Some(valeur) => { ii_resultat_heure = valeur as i64; },
            None  => { ii_resultat_heure = -1 as i64; },
    }
    println!("Index résultat : {}", ii_resultat_heure);

    let resultat_jour = chaine.find("jour");
    println!("{:?}", resultat_jour);

    // 2. remplacements
    println!("--- 2 ---");
    let chaine2 = "C'est le jour, jour, jour !!!".to_string();

    let remplacement1 = chaine2.replace("jour", "matin");
    println!("remplacement1 : {}", remplacement1);

    let remplacement2 = chaine2.replacen("jour", "matin", 2);
    println!("remplacement2 : {}", remplacement2);
}

extern crate regex;
use regex::Regex;
fn fonction_regex(){

    // Expression régulière pour un email.
    let date_exemple = "2022-01-17";

    let re = Regex::new(r"(?x)
    (?P<annee>\d{4})  
    -
    (?P<mois>\d{2}) 
    -
    (?P<jour>\d{2})   
    ").unwrap();

    let parsing_date = re.captures(date_exemple).unwrap();

    println!("Année : {}", &parsing_date["annee"]);
    println!("Mois : {}", &parsing_date["mois"]);
    println!("Jour : {}", &parsing_date["jour"]);
}

fn main() {
    
    // Travaillons avec char.
    //fonction_char();

    // Travaillons avec &str.
    //fonction_str();

    // Travaillons avec String.
    //fonction_String();

    // Les outils autour de String.
    //fonction_String_outils();

    // L'insertion avec String.
    //fonction_String_insertion();

    // La suppression avec String.
    //fonction_String_suppression();

    // La recherche et remplacement avec String.
    //fonction_String_recherche_remplacement();

    // Les expressions régulières.println!
    fonction_regex();
}