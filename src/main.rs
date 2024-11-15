use std::fs::{read_to_string, set_permissions, File, OpenOptions, Permissions};
use std::io::Write;
use std::path::Path;
use serde::Deserialize;
use clap::Parser;
use std::os::unix::fs::PermissionsExt;
use dirs::home_dir;

#[derive(Parser, Debug, Clone)]
struct Args {
    /// Install script to local profile, global profile or custom path [local_profile, global_profile, <path>]
    #[arg(short, long)]
    install: Option<String>,

    ///Remove script from local profile and global profile or from custom path [local_profile, global_profile, <path>]
    #[arg(short, long)]
    uninstall: Option<String>,

    ///Generate run script
    #[arg(short, long)]
    gen_script: bool,

    ///set joke category. Details https://sv443.net/jokeapi/v2/
    #[arg(long)]
    category: Option<String>,

    ///set joke language. Details https://sv443.net/jokeapi/v2/
    #[arg(short, long)]
    language: Option<String>,

    ///set joke black_list flags. Details https://sv443.net/jokeapi/v2/
    #[arg(short, long)]
    black_list: Option<String>,

    ///set joke type. Details https://sv443.net/jokeapi/v2/
    #[arg(short, long)]
    joke_type: Option<String>,

    ///set joke content. Details https://sv443.net/jokeapi/v2/
    #[arg(long)]
    content: Option<String>,

    ///set export location
    #[arg(short, long, default_value = "/etc/motd.d/jokes")]
    export_path: String
}

#[derive(Deserialize)]
struct Joke {
    joke: Option<String>,
    setup: Option<String>,
    delivery: Option<String>,
}

const BASE_URL: &'static str = "https://v2.jokeapi.dev/joke/";
const LINE_ENDING: &'static str = "\n";
const USER_SHELL_PATHS: [&'static str; 9] = [
    "/.zshrc",
    "/.bashrc",
    "/.bash_profile",
    "/.bash_login",
    "/.profile",
    "/.config/fish/config.fish",
    "/.kshrc",
    "/.cshrc",
    "/.tcshrc",
];
const GLOBAL_SHELL_PATHS: [&'static str; 9] = [
    "/etc/zshrc",
    "/etc/zsh/zshrc",
    "/etc/bash.bashrc",
    "/etc/profile",
    "/etc/fish/config.fish",
    "/etc/kshrc",
    "/etc/csh.cshrc",
    "/etc/csh.login",
    "/etc/profile",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.gen_script {
        generate_script(args.clone())?;
    } else if let Some(install) = args.clone().install {
        generate_script(args.clone())?;
        if install == "local_profile" {
            add_to_profile(USER_SHELL_PATHS.map(|e| (home_dir().unwrap().to_str().unwrap_or_default().to_owned() + e)).to_vec());
        }else if install == "global_profile" {
            add_to_profile(GLOBAL_SHELL_PATHS.map(|e| e.to_string()).to_vec());
        } else {
            match add_to_file(install.as_str()) {
                Ok(_) => println!("Added to {}", install),
                Err(e) => eprintln!("Failed adding to {} {LINE_ENDING} {e}", install),
            }
        }
    } else if let Some(uninstall) = args.uninstall {
        if uninstall == "local_profile" {
            remove_from_profile(USER_SHELL_PATHS.map(|e| (home_dir().unwrap().to_str().unwrap_or_default().to_owned() + e)).to_vec());
        }else if uninstall == "global_profile" {
            remove_from_profile(GLOBAL_SHELL_PATHS.map(|e| e.to_string()).to_vec());
        } else {
            match remove_from_file(uninstall.as_str()) {
                Ok(_) => println!("Added to {}", uninstall),
                Err(e) => eprintln!("Failed adding to {} {LINE_ENDING} {e}", uninstall),
            }
        }
    } else {
        fetch_joke(generate_url(args.clone()).as_str(), args.export_path.as_str()).await?;
    }

    Ok(())
}

fn generate_script(args: Args) -> Result<(), Box<dyn std::error::Error>>{
    let current_path = std::env::current_dir()?;
    let mut file = File::create(current_path.as_path().join(Path::new("start.sh")))?;
    file.write_all(format!("{}/jokeGreeting {} &> /dev/null", current_path.to_str().unwrap(), generate_args_string(args)).as_bytes())?;

    set_permissions(current_path.as_path(), Permissions::from_mode(0o755))?;
    Ok(())
}

fn generate_args_string(args: Args) -> String {
    let mut args_vec = Vec::new();

    if let Some(category) = args.category {
        args_vec.push(format!("--category {}", capitalize_words(category.as_str())));
    }
    if let Some(language) = args.language {
        args_vec.push(format!("--language {}", language));
    }
    if let Some(black_list) = args.black_list {
        args_vec.push(format!("--black-list {}", black_list.to_lowercase()));
    }
    if let Some(joke_type) = args.joke_type {
        args_vec.push(format!("--joke-type {}", joke_type));
    }
    if let Some(content) = args.content {
        args_vec.push(format!("--content {}", content));
    }

    args_vec.push(format!("--export-path {}", args.export_path));

    args_vec.join(" ")
}

fn add_to_profile(data: Vec<String>) {
    for i in data.iter() {
        match add_to_file(i) {
            Ok(_) => println!("Added to {}", i),
            Err(e) => eprintln!("Failed adding to {} {LINE_ENDING} {e}", i),
        }
    }
}

fn remove_from_profile(data: Vec<String>) {
    for i in data.iter() {
        match remove_from_file(i) {
            Ok(_) => println!("Removed from {}", i),
            Err(e) => eprintln!("Failed removing from {} {LINE_ENDING} {e}", i),
        }
    }
}

fn add_to_file(path: &str) -> Result<(), Box<dyn std::error::Error>>{
    remove_from_file(path)?;
    let current_path = std::env::current_dir()?;
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;
    file.write(format!("{}/start.sh {LINE_ENDING}", current_path.to_str().unwrap()).as_bytes())?;
    Ok(())
}

fn remove_from_file(path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let data = read_to_string(path)?;
    let current_path = std::env::current_dir()?;
    let new_contents: String = data
        .lines()
        .filter(|&l| l != format!("{}/start.sh {LINE_ENDING}", current_path.to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(LINE_ENDING);

    let mut file = File::create(path)?;
    file.write(&new_contents.as_bytes())?;

    Ok(())
}



async fn fetch_joke(url: &str, path: &str)-> Result<(), Box<dyn std::error::Error>>{
    let mut response = String::new();

    let joke = reqwest::get(url)
        .await?
        .json::<Joke>()
        .await?;

    if let Some(joke) = joke.joke {
        response += format!("{joke}\n").as_str();
    }
    if let Some(setup) = joke.setup {
        response += format!("{setup}\n").as_str();
    }
    if let Some(delivery) = joke.delivery {
        response += format!("{delivery}\n").as_str();
    }

    response += LINE_ENDING;

    println!("{}", response);

    let mut file = File::create(path)?;
    file.write_all(response.as_bytes())?;

    Ok(())
}

fn generate_url(args: Args) -> String {
    let mut url = BASE_URL.to_string();
    if let Some(category) = args.category{
        url += capitalize_words(category.as_str()).as_str();
    } else {
        url += "Any"
    }

    url += "?";

    if let Some(language) = args.language{
        url += format!("lang={language}&").as_str();
    }
    if let Some(black_list) = args.black_list{
        url += format!("blacklistFlags={}&", black_list.to_lowercase()).as_str();
    }
    if let Some(joke_type) = args.joke_type{
        url += format!("type={joke_type}&").as_str();
    }
    if let Some(content) = args.content{
        url += format!("contains={content}&").as_str();
    }

    url
}

fn capitalize_words(input: &str) -> String {
    input
        .split(',')
        .map(|word| {
            let mut chars = word.trim().chars();
            match chars.next() {
                Some(first) => first.to_uppercase().to_string() + chars.as_str().to_lowercase().as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}