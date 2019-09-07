#[derive(Debug)]
enum Prova {
    Val1(String),
    Val2(u32),
    ProvaStr { a: String, b: u32 },
}

fn main() {
    let real_enum: Prova = Prova::Val1(String::from("ciao"));
    let real_enum2: Prova = Prova::Val2(123);
    let mix = Prova::ProvaStr {
        a: String::from("che bello"),
        b: 12,
    };
    expore(real_enum);
    expore(real_enum2);
    expore(mix);
}

fn expore(val: Prova) {
    match val {
        Prova::Val1(x) => println!("Questa stringa e' {}", x),
        Prova::Val2(x) => println!("Questo e' un numero {}", x),
        Prova::ProvaStr { a, b } => println!("Questa e' una combinazione {} {}", a, b),
    }
}
