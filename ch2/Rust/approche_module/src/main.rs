mod calcul;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rayon_str = &args[1];
    let rayon = rayon_str.parse::<f64>().unwrap();

    let p = calcul::perimetre_cercle(rayon);
    println!(">>> Le périmètre du cercle est égale à : {}", p);

    let s = calcul::surface_cercle(rayon);
    println!(">>> La surface du cercle est égale à : {}", s);

    let s_sph = calcul::surface_sphere(rayon);
    println!(">>> La surface de la sphère est égal à : {}", s_sph);

    let v_sph = calcul::volume_sphere(rayon);
    println!(">>> Le volume de la sphère est égal à : {}", v_sph);
}
