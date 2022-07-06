fn main() {
    struct Point (i64, i64, String);

    let point_a = Point(1, 2, String::from("A"));
    println!("{}", point_a.0);
    println!("{}", point_a.1);
    println!("{}", point_a.2);

    // Version publique de la structure.
    pub struct Point2 (pub i64, pub i64, pub String);
    let point_b = Point2(3, 4, String::from("B"));
    println!("{}", point_b.0);
    println!("{}", point_b.1);
    println!("{}", point_b.2);

    // Structure de type unité.
    struct Unite;
    let u = Unite;
}
