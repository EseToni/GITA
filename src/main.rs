mod diff;
mod gita_auto;
mod gita_base;
mod gita_restore;
mod utils;

use gita_auto::gita_auto;
use gita_base::gita_base;
use gita_restore::gita_restore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "restore" {
            gita_restore();
        } else if args[1] == "auto" {
            if args.contains(&"-t".to_string()) {
                let file_types: Vec<&str> = args[2..].iter().map(|x| x.as_str()).collect();
                gita_auto(file_types);
            } else {
                gita_auto(vec![]);
            }
        }
    } else {
        gita_base();
    }
}
