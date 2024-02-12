use std::{error::Error, fs};

pub struct Cfg {
    query: String,
    path: String,
}

impl Cfg {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].to_string();
        let path = args[2].to_string();

        Ok(Cfg { path, query })
    }
}

pub fn run(cfg: Cfg) -> Result<(), Box<dyn Error>> {
    fs::read_to_string(cfg.path)?
        .split("\n")
        .filter(|x| x.contains(cfg.query.as_str()))
        .for_each(|line| println!("{line}"));
    Ok(())
}
