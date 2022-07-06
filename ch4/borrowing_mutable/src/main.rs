fn main() {
    let mut chaine3 = String::from("ENI");
    modifier_chaine(&mut chaine3);

    println!("{}", chaine3);
}

fn modifier_chaine(ch: &mut String){
    ch.push_str(" !");
}