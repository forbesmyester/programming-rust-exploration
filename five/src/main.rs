use std::env;
use std::str::FromStr;
use std::collections::HashMap;


type Table = HashMap<String, Vec<u8>>;


enum Input {
    S(String),
    I(u8),
}


fn to_input(s: &String) -> Input {
    // Input::S(s.to_string())
    match u8::from_str(s) {
        Ok(n) => Input::I(n),
        Err(_e) => Input::S(s.to_string())
    }
}


fn to_table(tab: &mut Table, input: Input) -> &mut Table {
    let k: String = "a".to_string();
    match tab.get(&k.to_string()) {
        Some(mut v) => v.append(0),
        None => {
            tab.insert(k.to_string(), vec![0]);
        }
    }
    tab
}


fn print_table(tab: &Table) {

}


fn main() {
    let args: Vec<String> = env::args().collect();

    let inputs: Vec<Input> = args[1..].into_iter().map(to_input).collect();

    // println!("Sum: {:?}", inputs);
}
