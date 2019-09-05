use std::io;

fn main() {
    let mut num = String::new();
    println!("Please enter a number to evaluate fibonacci series");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to get a number");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let res = fibonacci(num);
    println!("Result is {}", res)
}

fn fibonacci(num: u32) -> u32 {
    let mut f0: u32 = 1;
    let mut f1: u32 = 1;

    if num > 2 {
        let mut res = 0;
        for _ in 2..num {
            res = f0 + f1;
            f1 = f0;
            f0 = res;
        }
        return res;
    } else if num == 0 {
        return 0;
    } else {
        return 1;
    }
}
