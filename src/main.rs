#![feature(path_try_exists)]
use std::env;
use rand::Rng;
use std::fs;
use std::path::PathBuf;
use open;
use directories_next::ProjectDirs;

fn main() {
    let project_data_dir = get_path();

    match fs::try_exists(project_data_dir) {
        Ok(v) => match v {
            true => (),
            false => initialize()
        },
        Err(e) => panic!("{}",e)
    }

    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| a.as_str()).collect();

    match &args[1..] {
        ["clear"] => clear(),
        ["list"] => list(),
        ["add", url] => add_url(url),
        ["rm"] => remove(),
        ["length"] => how_long(),
        [] => open_url(), 
        _ => println!("Unexpected input")
    };
}

fn initialize() {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Error initializing");
    let mut path = PathBuf::from(project_dirs.data_dir());

    fs::create_dir_all(&path).expect("Error initializing");
    path.push("urls.txt");
    
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

    match fs::write(path, filestring) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

fn open_url() {
    let url = get_url();
    open::that(url).expect("Error opening url");
}

fn add_url(url: &str) {
    let filepath = get_path();

    let mut file_str = fs::read_to_string(&filepath).expect("Error reading file");
    let formatted_addendum = format!("\n{}", url);
    file_str.push_str(&formatted_addendum);

    fs::write(&filepath, file_str).expect("Error writing to file");

    println!("Successfully added {}", url);
}

fn get_url() -> String {
    let filepath = get_path();

    let file_str = fs::read_to_string(filepath).expect("Error reading file");
    let urls: Vec<&str> = file_str.split("\n").collect();

    let mut rng = rand::thread_rng();
    
    return match urls.len() {
        0 | 1 => String::from("https://github.com/AndruMace/BreakTime"),
        _ => urls[rng.gen_range(1..urls.len())].to_owned()
    }
}

fn clear() {
    let filepath = get_path();

    fs::File::create(filepath).expect("Error clearing file");
    println!("urls.txt has been cleared! Start adding back your own urls.")
}

fn list() {
    let filepath = get_path();

    let file_str = fs::read_to_string(filepath).expect("Error reading file");
    println!("\n{}", file_str);
}

fn remove() {
    let filepath = get_path();
    let mut file_str = fs::read_to_string(&filepath).expect("Error reading file");
    let mut urls: Vec<&str> = file_str.split("\n").collect();
    urls.pop();

    file_str = urls.join("\n");
    fs::write(&filepath, file_str).expect("Error removing last url");
}

fn how_long() {
    // let mut rng = rand::thread_rng();
    let r_int = rand::thread_rng().gen_range(10..30);
    println!("{}",r_int.to_string());
}

fn get_path() -> PathBuf {
    return ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("could not retrieve home directory from OS").data_dir().join("urls.txt");
}