#![allow(unused)]
enum Personnage{
    Heros,
    Fantome { points_de_vie : u32, indice_invisibite : u32},
    Combattant { points_de_vie : u32},
    Magicien(u32, u32),
}

fn filtrage_par_motif(entree : Personnage) -> String {
    match entree{
        Personnage::Heros 
        => format!("Le héros va très bien."),
        Personnage::Fantome{ points_de_vie, indice_invisibite }  
        => format!("Point de vie {} et indice d'invisibilité {} pour le fantôme.", points_de_vie, indice_invisibite),
        Personnage::Combattant{ points_de_vie }  
        => format!("Point de vie {} pour le combattant.", points_de_vie),
        Personnage::Magicien( a, b)
        => format!("Indice de magie {} et âge {} pour le magicien.", a, b),
    }
}

fn main() {
    let h = Personnage::Heros;
    println!("{}", filtrage_par_motif(h));
   
    let f = Personnage::Fantome{points_de_vie : 25, indice_invisibite : 12};
    println!("{}", filtrage_par_motif(f));

    let c = Personnage::Combattant{points_de_vie : 35};
    println!("{}", filtrage_par_motif(c));

    let m = Personnage::Magicien(34, 78);
    println!("{}", filtrage_par_motif(m));

    let annee : u32 = 2010;
    match annee {
        1998 => {
            println!("Première victoire en coupe du monde de l'équipe de France de football.");
        }
        2018 => {
            println!("Deuxième victoire en coupe du monde de l'équipe de France de football.");
        }
        autre => {
            println!("Ni 1998 ni 2018, pas de victoire en {}.", autre);
        }
    }

    let annee2 : i32 = 2011;
    match annee2 {
        annee2 if annee2 >= 2000 => {
            println!("C'est déjà le 21e siècle.");
        }
        annee2 if annee2 < 2000 && annee2 >= 1900 => {
            println!("On est au 20e siècle.");
        }
        _ => {
            println!("On est à un autre siècle");
        }
    }

    match annee2 {
        2000..=2099 => println!("L'année est compris entre l'an 2000 et l'an 2099 (incluse)."),
        autre => println!("L'année {} n'est pas dans cet intervalle", autre)
    };

    let prenom = "Hector";
    match prenom {
        "Arthur" | "Sophie" | "Hector" => println!("Quel joli prénom."),
        autre => println!("Quel prénom joli.")
    };

    struct Personne{
        nom : String,
        numero_service: u32
    }
}
