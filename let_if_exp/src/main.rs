fn main() {
    #[derive(Debug)]
    struct DoubleVal(u32, u32);
    let val: Option<u32> = Some(2);
    let dval = DoubleVal(3, 5);

    //if let must be intrpreted from right to left

    if let Some(_i) = val {
        println!("Number is correct!");
    } else {
        println!("Numer is wrong, should be {}", Some(4).unwrap());
    };

    if let DoubleVal(6, 10) = dval {
        println!("Number is correct!");
    } else {
        println!("Value should have been {:?}", dval);
    };
}
