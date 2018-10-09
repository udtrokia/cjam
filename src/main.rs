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
    let pathes = Toml::pathes(dir);

    println!("Alias Pathes:\n ");
    for p in pathes {
        let t = Toml::detail(p);
        println!("{} —— {:?}", t.name, t.alias);
    }
}
