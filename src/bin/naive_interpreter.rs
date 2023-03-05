#[macro_use]
extern crate afl;
extern crate url;

extern crate naive_hashmap;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut hash_map = naive_hashmap::HashMap::new();

    let n = io::stdin();
    for line in n.lock().lines() {
        if let Ok(line) = line {
            let cmd : Vec<&str> = line.as_str().split(' ').collect();
            if cmd.len() == 0 {
                continue;
            }
            match cmd[0] {
                "Lookup" => {
                    if cmd.len() != 2 {
                        println!("Lookup needs one argument");
                        continue;
                    }
                    let key = cmd[1].parse::<i32>().unwrap();
                    let _ = hash_map.lookup(key);
                    //println!("{:?}", value);
                }
                "Insert" => {
                    if cmd.len() != 3 {
                        println!("Insert needs two arguments");
                        continue;
                    }
                    let key = cmd[1].parse::<i32>().unwrap();
                    let value = cmd[2].parse::<i32>().unwrap();
                    hash_map.insert(key, value);
                }
                _ => {
                    println!("Unknown command {}", cmd[0]);
                }
            }
        }
    }
}