struct Cercle{
    coord_centre_x : i64,
    coord_centre_y : i64,
    rayon_cercle : i64,
    nom_cercle : String,
}

fn main() {
    let cercle_1 = Cercle{
        coord_centre_x : 5,
        coord_centre_y : 5,
        rayon_cercle : 2,
        nom_cercle : String::from("cercle_1"),
    };
    
    println!("{}", cercle_1.coord_centre_x);
    println!("{}", cercle_1.coord_centre_y);
    println!("{}", cercle_1.rayon_cercle);
    println!("{}", cercle_1.nom_cercle);
}
