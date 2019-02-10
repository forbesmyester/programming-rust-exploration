use std::collections::HashMap;
use std::env;
use std::str::FromStr;

type Table = HashMap<String, Vec<u8>>;

enum Input {
    S(String),
    I(u8),
}

fn convert_to_input(s: &String) -> Input {
    match u8::from_str(s) {
        Ok(n) => Input::I(n),
        Err(_e) => Input::S(s.to_string())
    }
}


struct TableBuilder {
    table: Table,
    active_key: String,
}

impl TableBuilder {
    fn get_active_key(&self) -> String {
        self.active_key.to_string()
    }
}


fn to_table_builder(mut tab: TableBuilder, input: Input) -> TableBuilder {
    let k = TableBuilder::get_active_key(&tab);
    match input {
        Input::S(s) => { tab.active_key = s }
        Input::I(n) => {
            if k != "_".to_string() {
            tab.table.entry(k).and_modify(|v| v.push(n)).or_insert(vec![n]);
            }
        }
    }
    tab
}


fn print_table(tab: &Table) {
    let mut first = true;
    for (k, vs) in tab {
        let mut pre = "\n\n".to_string();
        if first { pre = "".to_string(); }
        first = false;
        println!("{}{}", pre, k);
        for v in vs {
            println!(" * {}", v);
        }
    }
    println!("\n");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let tab = TableBuilder {
        table: HashMap::new(),
        active_key: "_".to_string(),
    };

    let table = args[1..].into_iter()
        .map(convert_to_input)
        .fold(tab, to_table_builder)
        .table;
    // println!("Sum: {:?}", inputs);

    print_table(&table);

}
