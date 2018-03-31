use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "install" => println!("PASS!"),
        _ => panic!("FAIL!"),
    }
}
