// Use clap and serde_json to parse and pretty-print user information
// These are the defacto libraries, but are heavyweight and will add to compile time

use clap::Clap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Clap, Debug)]
struct MyOpts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short)]
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProfilePreferences {
    food: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    age: i32,
    weight: Option<f32>,
    preferences: ProfilePreferences,
}

// Run this with `cargo run -- -i input.json`
fn main() {
    // Read CLI args
    // https://github.com/clap-rs/clap#quick-example
    // Derive is the easiest
    let opts: MyOpts = MyOpts::parse();
    dbg!(&opts);

    // Read the file https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let contents = fs::read_to_string(opts.input).expect("Something went wrong reading the file");

    // Parse the JSON
    let profile: Profile = serde_json::from_str(&contents).expect("Failed to parse profile");
    dbg!(&profile);

    // Print it!
    println!("{}", emoji_food_preferences(&profile));
}

fn emoji_food_preferences(profile: &Profile) -> String {
    let mut mapping: HashMap<String, &str> = HashMap::new();
    mapping.insert("taco".into(), "🌮");
    mapping.insert("pizza".into(), "🍕");
    mapping.insert("cookie".into(), "🍪");

    let mut emojified_food: Vec<String> = vec![];

    for food in profile.preferences.food.iter() {
        if let Some(ref emoji) = mapping.get(food) {
            emojified_food.push(emoji.to_string())
        } else {
            emojified_food.push(food.to_string());
        }
    }

    emojified_food.join(" ")
}
