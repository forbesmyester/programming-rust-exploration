use std::env;
use std::str::FromStr;


fn padovan() {
    let mut r = vec![1, 1, 1,];
    for i in 3..30 {
        let n = r[i - 3] + r[i - 2];
        r.push(n);
    }
    println!("P(1..10) = {:?}", r);
}


fn to_int_o(s: &String) -> Option<u8> {
    u8::from_str(s).ok()
}


fn to_int(s: &String) -> u8 {
    u8::from_str(s).unwrap_or(0)
}


fn sum(acc: u8, n: u8) -> u8 {
    acc + n
}


fn sum_o(acc: u8, o: Option<u8>) -> u8 {
    match o {
        Option::Some(n) => acc + n,
        Option::None => acc,
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // Step by step
    let vargs: Vec<Option<u8>> = args[1..].into_iter().map(to_int_o).collect();
    let result = vargs.into_iter().fold(0, sum_o);

    // Or in one with basic
    // let result = args[1..].into_iter().map(to_int).fold(0, sum);
    
    // Or with enums
    // let result = args[1..].into_iter().map(to_int_o).fold(0, sum_o);


    println!("Sum: {:?}", result);
    padovan()
}
