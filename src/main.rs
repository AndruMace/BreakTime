#![feature(path_try_exists)]
use std::env;
use rand::Rng;
use std::fs;
use std::path::Path;
use open;

fn main() {

    match Path::new("urls.txt").try_exists() {
        Ok(exists) => match exists {
            true => (),
            false => initialize()
        },
        Err(err) => panic!("{}", err) 
    }

    // let args: Vec<&str> = env::args().collect().iter().map(|a| a.as_str()).collect();
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| a.as_str()).collect();

    match &args[1..] {
        ["clear"] => clear(),
        ["list"] => list(),
        ["add", url] => add_url(url),
        [] => open_url(), 
        _ => println!("Unexpected input")
    };
}

fn initialize() {
    let defaults: [&'static str; 4] = [
        "https://www.reddit.com", 
        "https://news.ycombinator.com/", 
        "https://www.youtube.com/", 
        "https://doc.rust-lang.org/stable/book/"
    ];

    let mut filestring = String::from("");

    for url in defaults {
        let fmt_str = format!("\n{}", url);
        filestring.push_str(&fmt_str)
    }

    match fs::write("urls.txt", filestring) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

fn open_url() {
    let url = get_url();
    open::that(url).expect("Error opening url")
}

fn add_url(url: &str) {
    let mut file_str = fs::read_to_string("urls.txt").expect("Error reading file");
    let formatted_addendum = format!("\n{}", url);
    file_str.push_str(&formatted_addendum);

    fs::write("urls.txt", file_str).expect("Error writing to file");

    file_str = fs::read_to_string("urls.txt").expect("Error reading file");
    println!("New file is: {}", file_str)
}

fn get_url() -> String {
    let file_str = fs::read_to_string("urls.txt").expect("Error reading file");
    let urls: Vec<&str> = file_str.split("\n").collect();

    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..urls.len());

    return urls[rand_index].to_owned()
}

fn clear() {
    fs::write("urls.txt", "").expect("Error clearing file");
    println!("urls.txt has been cleared! Start adding back your own urls.")
}

fn list() {
    let file_str = fs::read_to_string("urls.txt").expect("Error reading file");
    println!("{}", file_str);
}