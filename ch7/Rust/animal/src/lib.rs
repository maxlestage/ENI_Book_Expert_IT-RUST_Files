#![allow(unused)]

pub mod mod_animal {

    pub trait Animal {
        
        fn creer(nom: String) -> Self;
    
        fn emettre_son(&self) -> String;
        fn obtenir_nom(&self) -> String;
    
        fn afficher(&self) {
            println!("{} : {}", self.obtenir_nom(), self.emettre_son());
        }
    }

    pub struct Chien {
        nom: String
    }

    pub struct Chat {
        nom: String
    }

    impl Chat {

        pub fn dormir(&self) {
            println!("Comme tous les chatons, j'aime dormir.")
        }
    }

    impl Animal for Chien {

        fn creer(nom: String) -> Chien {
            Chien { nom: nom }
        }
    
        fn emettre_son(&self) -> String {
           "aboiement".to_string()
        }
    
        fn obtenir_nom(&self) -> String {
            let copie = self.nom.clone();
            copie
        }

        // Surcharge de l'implémentation.
        fn afficher(&self) {
            println!("Moi le chien {}, j'émets un {}.", self.obtenir_nom(), self.emettre_son());
        }
    }

    impl Animal for Chat {

        fn creer(nom: String) -> Chat {
            Chat { nom: nom }
        }
    
        fn emettre_son(&self) -> String {
           "miaulement".to_string()
        }
    
        fn obtenir_nom(&self) -> String {
            let copie = self.nom.clone();
            copie
        }
    }
}
