// 
// -> Cargo Jam <-
// 
// Copyright @ udtrokia
//
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use regex::Regex;

#[derive(Debug)]
pub struct Toml {
    pub name: String,
    pub alias: Vec<String>,
}

impl Toml {
    pub fn root() -> PathBuf {
        let mut path = PathBuf::default();
        let mut root = PathBuf::from("./");
        
        while root.to_string_lossy() != "/" {
            root = root.canonicalize().unwrap();        
            let toml = root.join("Cargo.toml");
            root.pop();

            if toml.exists() {
                path = toml.to_owned();
            }
        }
        
        return path;
    }

    pub fn pathes(dir: PathBuf) -> Vec<PathBuf> {
        let mut pathes = vec![];

        for entry in dir.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if entry.path().ends_with("Cargo.toml") {
                    pathes.push(entry.path());
                }

                if entry.path().is_file() { continue }
                if entry.path().ends_with("target") { continue }
                if entry.path().ends_with(".git") { continue }

                pathes.append(&mut Toml::pathes(entry.path()))
            }
        }
        
        return pathes;
    }

    pub fn detail(path: PathBuf) -> Self {
        let name: String;
        let mut alias = vec![String::new()];

        let mut content = String::new();
        let mut file = File::open(path.as_path()).unwrap();
        file.read_to_string(&mut content).unwrap();

        let mut re = Regex::new(r#"name = "(.*)""#).unwrap();
        name = String::from(&re.captures(&content).unwrap()[1]);
        
        re = Regex::new(r#"(.*)\s+=\s+\{\s+path\s+=\s+"(.*)"\s+\}"#).unwrap();
        for cap in re.captures_iter(&content){
            alias.push(cap[2].to_string())
        }
        Toml{ name, alias }
    }
}
