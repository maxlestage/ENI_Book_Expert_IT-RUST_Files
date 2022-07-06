pub fn perimetre_cercle(rayon: f64) -> f64 {
    assert!(rayon >= 0.0);

    let perimetre = 2.0 * std::f64::consts::PI * rayon;
    perimetre
}

pub fn surface_cercle(rayon: f64) -> f64 {
    assert!(rayon >= 0.0);

    let surface = std::f64::consts::PI * rayon * rayon;
    surface
}

pub fn surface_sphere(rayon: f64) -> f64 {
    assert!(rayon >= 0.0);

    let surface_sph = 4.0 * std::f64::consts::PI * rayon * rayon;
    surface_sph
}

pub fn volume_sphere(rayon: f64) -> f64 {
    assert!(rayon >= 0.0);

    let volume_sph = 4.0 / 3.0 * std::f64::consts::PI * f64::powf(rayon, 3.0);
    volume_sph
}

#[test]
fn test_volume_sphere() {
    let resultat = volume_sphere(5.0);
    let mut test = false;
    if resultat > 523.0 && resultat < 524.0 {
        test = true;
    }
    assert_eq!(test, true);
}

// #[test]
// fn test_echec() {
//     let test = false;
//     assert_eq!(test, true);
// }
