#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    xx : i64,
    yy  : i64,
}

fn main() {

    let p1 = Point{xx: 2, yy:3};

    // Grâce au trait Clone
    let p2 = p1.clone(); 
    println!("xx après clonage : {}", p2.xx);
    println!("yy après clonage : {}", p2.yy); 

    // Grâce au trait Copy
    let p3 = p2;
    println!("xx après clonage : {}", p3.xx);
    println!("yy après clonage : {}", p3.yy); 

    // Grâce au trait Debug
    println!("p3 : {:?}", p3);

    // Grâce au trait PartialEq
    if p3 == p1{
        println!("PartialEq : {}", p3 == p1);
    }
}
