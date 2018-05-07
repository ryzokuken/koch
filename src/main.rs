extern crate reqwest;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::io::ErrorKind;
use serde_json::Value;
use std::io::prelude::*;

fn get_recipes() -> Vec<String> {
    let mut recipes = Vec::new();
    let url = "https://api.github.com/repos/Homebrew/homebrew-core/git/trees/ae7c06e4b7c363a68df4ed010f8afbb02a8abf24";
    let text = reqwest::get(url).unwrap().text().unwrap();
    let v: Value = serde_json::from_str(&text).unwrap();

    let patterns: &[_] = &['.', 'r', 'b'];

    for k in v["tree"].as_array().unwrap() {
        recipes.push(
            k["path"]
                .as_str()
                .unwrap()
                .trim_matches(patterns)
                .to_string(),
        );
    }
    return recipes;
}

fn update(args: Vec<String>, path: PathBuf) {
    if args.len() != 2 {
        panic!("Update expects no arguments.");
    }

    println!("[*] Updating Koch, fetching recipes");
    let recipes = get_recipes();
    println!("[+] Recipes fetched successfully, writing to local disk");
    let text = recipes.join("\n");

    let mut f = File::create(&path).unwrap();
    f.write_all(text.as_bytes()).unwrap();
    println!("[+] Recipes written successfully to local disk");
}

fn fetch_instructions(program: &str) {
    let url = format!(
        "https://api.github.com/repos/Homebrew/homebrew-core/contents/Formula/{}.rb",
        program
    );
    let text = reqwest::get(url.as_str()).unwrap().text().unwrap();
    let v: Value = serde_json::from_str(&text).unwrap();
    println!("{}", v["content"].as_str().unwrap())
}

fn install(args: Vec<String>, recipes: &Vec<String>) {
    if args.len() != 3 {
        panic!("Update expects a single argument.");
    }

    let mut iter = recipes.into_iter();
    println!("[*] Attempting to install {}", args[2]);
    if iter.find(|&x| x == &args[2]).is_none() {
        println!("[-] Recipe {} not found", args[2]);
        panic!("Recipe not found")
    }

    println!(
        "[+] Recipe {} found successfully, fetching instructions",
        args[2]
    );
    fetch_instructions(args[2].as_str());
}

fn load_recipe_cache(path: &PathBuf) -> Vec<String> {
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
    let mut path = env::home_dir().unwrap();
    path.push(".koch");

    let recipes = load_recipe_cache(&path);
    // for recipe in recipes {
    //     println!("{}", recipe);
    // }

    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "update" => update(args, path),
        "install" => install(args, &recipes),
        _ => panic!("FAIL!"),
    }

    // for recipe in recipes {
    //     println!("{}", recipe);
    // }
}
