pub mod figures {
    pub mod quadrilatero;
}

use figures::quadrilatero::solid::{self, Triangle as Tri};

fn stampa(val: &solid::Parallelepipedo) {
    println!("Dimensions of parallelepipedo are {:?}", val);
}

fn main() {
    let mut paral: solid::Parallelepipedo = solid::Parallelepipedo {
        base: 3.2,
        altezza: 4.7,
        name: String::from("parallelepipedo"),
    };
    println!("Area of parallelepipedo is {}", paral.area());
    //Println is a macro therefore doesn't implement move
    println!("Dimensions of parallelepipedo are {:?}", paral);
    paral.base = 7.3;
    println!("Area of parallelepipedo is {}", paral.area());
    stampa(&paral);
    paral.base = 2f32;
    stampa(&paral);
    let tri: Tri = Tri::init(2.3, 3.6, "Triangolo".to_string());
    println!("Triangle dimensions are {:?}", tri);
}
