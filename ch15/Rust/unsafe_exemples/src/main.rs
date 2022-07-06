fn pointeur_dereference(){

    let mut nombre : i64 = 42;

    let ptr1 = &nombre as *const i64;
    let ptr2 = &mut nombre as *mut i64;
    unsafe{

        println!("Valeur du nombre pointé : {}", *ptr1);
        println!("Valeur du nombre pointé : {}", *ptr2);
    }
}

static mut PI: f64 = 3.14;

fn modifier_PI() {

    unsafe {
        PI += 0.00159;
    }

    unsafe {
        println!("PI : {}", PI);
    }
}

fn main() {
    pointeur_dereference();

    modifier_PI();
}
