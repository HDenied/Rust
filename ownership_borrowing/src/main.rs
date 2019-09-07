#[derive(Debug)]
struct Prova(u32, u32);

fn main() {
    let val = Prova(12, 34);
    let val1 = &val;

    // Structure are on the heap
    println!("Value of val is {:?}", val);
}
