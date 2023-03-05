#[macro_use]
extern crate afl;

extern crate naive_hashmap;

use std::io;
use std::io::prelude::*;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut hash_map = naive_hashmap::HashMap::new();
        let content = String::from_utf8_lossy(data);
        let lines = content.lines();
        for line in lines {
            let cmd: Vec<&str> = line.split(' ').collect();
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
    });
}

