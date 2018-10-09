// 
// -> Cargo Jam <-
// 
// Copyright @ udtrokia
//
use cjam::Toml;

fn main() {
    // if Cargo.toml here
    let mut dir = Toml::root();
    dir.pop();
    let paths = Toml::pathes(dir);

    if paths.len() == 0 {
        println!("No crates found.");
        return;
    }
    println!("Alias Pathes:\n ");
    for p in paths {
        let t = Toml::detail(p);
        println!("{} —— {:?}", t.name, t.alias);
    }
}
