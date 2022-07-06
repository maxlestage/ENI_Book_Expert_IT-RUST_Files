#![allow(unused)]

enum ArbreBinaire<T>{
    Terminaison,
    NonTerminaison(Box<Noeud<T>>)
}

struct Noeud<T>{
    valeur : T,
    gauche : ArbreBinaire<T>,
    droite : ArbreBinaire<T>
}

fn main() {
    let noeud1 : Noeud<i64> = Noeud{ valeur : 1, gauche : ArbreBinaire::Terminaison, droite : ArbreBinaire::Terminaison };
    let arbre1 : ArbreBinaire<i64> = ArbreBinaire::NonTerminaison(Box::new(noeud1));

    let noeud2 : Noeud<i64> = Noeud{ valeur : 2, gauche : ArbreBinaire::Terminaison, droite : ArbreBinaire::Terminaison };
    let arbre2 : ArbreBinaire<i64> = ArbreBinaire::NonTerminaison(Box::new(noeud2));

    let noeud3 : Noeud<i64> = Noeud{ valeur : 3, gauche : arbre1, droite : arbre2 };
    let arbre3 : ArbreBinaire<i64> = ArbreBinaire::NonTerminaison(Box::new(noeud3));
}

