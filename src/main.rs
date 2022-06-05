#![feature(path_try_exists)]
use std::env;
use rand::Rng;
use std::fs;
use std::path::{PathBuf};
use open;
use directories_next::ProjectDirs;


fn main() {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
    let mut project_data_dir = PathBuf::from(project_dirs.data_dir());
    project_data_dir.push("urls.txt");


    match fs::try_exists(project_data_dir){
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
        [] => open_url(), 
        _ => println!("Unexpected input")
    };
}

fn initialize() {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
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
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
    let project_data_dir = project_dirs.data_dir().display();
    let filepath = format!("{}/{}", project_data_dir, "urls.txt");

    let mut file_str = fs::read_to_string(&filepath).expect("Error reading file");
    let formatted_addendum = format!("\n{}", url);
    file_str.push_str(&formatted_addendum);

    fs::write(&filepath, file_str).expect("Error writing to file");

    file_str = fs::read_to_string(&filepath).expect("Error reading file");
    println!("New file is: {}", file_str)
}

fn get_url() -> String {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
    let mut path = PathBuf::from(project_dirs.data_dir());
    path.push("urls.txt");

    let file_str = fs::read_to_string(path).expect("Error reading file");
    let urls: Vec<&str> = file_str.split("\n").collect();

    let mut rng = rand::thread_rng();

    println!("!!! {} !!!", urls.len());
    
    return match urls.len() {
        0 | 1 => String::from("https://github.com/AndruMace/BreakTime"),
        _ => urls[rng.gen_range(1..urls.len())].to_owned()
    }
}

fn clear() {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
    let project_data_dir = project_dirs.data_dir().display();
    let filepath = format!("{}/{}", project_data_dir, "urls.txt");

    fs::File::create(filepath).expect("Error clearing file");
    println!("urls.txt has been cleared! Start adding back your own urls.")
}

fn list() {
    let project_dirs = ProjectDirs::from("ind", "Andru Mace", "breaktime").expect("Some error!");
    let project_data_dir = project_dirs.data_dir().display();
    let filepath = format!("{}/{}", project_data_dir, "urls.txt");

    let file_str = fs::read_to_string(filepath).expect("Error reading file");
    println!("\n{}", file_str);
}