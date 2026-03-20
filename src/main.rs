use std::{env, fs};
use serde::{Deserialize, Serialize};
use clap::Parser;
use rand::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Passwd<'a> {
    app: &'a str,
    password: Vec<u8>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// length of the password
    #[arg(short, long, default_value_t = 12)]
    length: u8,

    /// name of app
    #[arg(short, long, default_value = "SavedPassword")]
    name: String,

    /// view password
    #[arg(short, long)]
    toggle: bool,
    // app_name: String,

}

fn main() {
    let combination = "1234567890qwertyuioplkjhgfdsazxcvbnm".chars();

    let args = Args::parse();
    if args.toggle {
        
    }

    let mut password = String::new();

    for _ in 0..args.length {
        let mut rng = rand::rng();

        let random: char = combination.clone().choose(&mut rng)
            .unwrap();
        password.push(random);
    }

    let encrypted = encrypt(&password);

    let passwd = Passwd {
        app: &args.name,
        password: encrypted,
    };


    let serialized = serde_json::to_string(&passwd)
        .unwrap();
    println!("Your password is: {:?}", password);

    let _ = save_password(args.name, serialized);

}

fn save_password(name: String, password: String) -> std::io::Result<()> {
    let home = if cfg!(target_os = "windows"){
        env::var("USERPROFILE").expect("not set")
    } else {
        env::var("HOME").expect("not set")
    };

    let dir = format!("{home}/.pmanager/passwords");
    fs::create_dir_all(&dir)?;

    let path = format!("{dir}/{name}.txt");
    println!("Writing to : {}", path);
    fs::write(path, password)?;

    Ok(())
}

fn encrypt(password: &str) -> Vec<u8>{
    use ring::rand::{SecureRandom, SystemRandom};

    let rng = SystemRandom::new();

    let mut key = [0u8; 32];
    rng.fill(&mut key).unwrap();

    let encrypted: Vec<u8> = password
        .bytes()
        .map(|b| b ^ key[0])
        .collect();

    encrypted

}
