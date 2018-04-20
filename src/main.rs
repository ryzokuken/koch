extern crate reqwest;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
// use serde_json::Value;

// fn get_text(website: &str) {
//     let text = reqwest::get(website).unwrap().text().unwrap();
//     let v: Value = serde_json::from_str(&text).unwrap();
//     for k in v["tree"].as_array().unwrap() {
//         println!("{}", k["path"]);
//     }
// }

fn load_recipe_cache() -> Vec<String> {
    let mut path = env::home_dir().unwrap();
    path.push(".koch");

    let f = File::open(&path);

    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create(path) {
            Ok(fc) => fc,
            Err(e) => panic!(
                "Tried to create cache file but there was a problem: {:?}",
                e
            ),
        },
        Err(error) => panic!("There was a problem opening the cache file: {:?}", error),
    };

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    return s.split("\n").map(|s| s.to_string()).collect();
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // match args[1].as_str() {
    //     "install" => get_text("https://api.github.com/repos/Homebrew/homebrew-core/git/trees/ae7c06e4b7c363a68df4ed010f8afbb02a8abf24"),
    //     _ => panic!("FAIL!"),
    // }

    let recipes = load_recipe_cache();
    for recipe in recipes {
        println!("{}", recipe);
    }
}
