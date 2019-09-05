#[derive(Debug)]
enum Prova {
    Val1(String),
    Val2(u32),
}

fn main() {
    let real_enum = Prova::Val1(String::from("ciao"));
    let real_enum2 = Prova::Val2(123);
    println!("First val is {:?}, second value is {:?}", real_enum, real_enum2);
}
