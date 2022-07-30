mod args;

use std::{process::Command};
use args::Args;
use clap::Parser;
 
// STR_MAX Length of String before it is trimmed and appended ...
const STR_MAX: u8 = 40;
// Newline character in Decimal
const NEWLINE: u8 = 10;

fn main() {
    // Parses Argument provided to the script
    let args: Args = Args::parse();
    // Initializes the values that will be returned at the end of the program
    let mut text = String::new();
    let status: String = get_status(&args.media);

    // Decide what to do depending on status output
    if status == "Playing" {
        // Initialize blank append variable (Modified if string is too long)
        let mut append: &str = "";
        let data: String = get_data(&args.media);

        if data.len() as u8 >= STR_MAX {
            append = "...";
        }

        text = format!("{}{} {}", data.trim(), append, args.icon)
    } else if status == "Paused" {
        text = args.icon;
    }

    println!("{{\"text\":\"{}\", \"class\":\"{}\"}}", text, status);
}

fn get_status(player: &String) -> String {
    let status = Command::new("playerctl")
        .arg("status")
        .arg(format!("--player={}", player))
        .output()
        .expect("Status command failed, Is playerctl installed?");
    // status.stdout returns Vec<u8>
    let mut out: Vec<u8> = Vec::new();
    for i in status.stdout {
        if i == NEWLINE {
            continue;
        }
        out.push(i);
    }
    String::from_utf8_lossy(&out).to_string()
}

fn get_data(player: &String) -> String {
    let details = Command::new("playerctl")
        .arg("metadata")
        .arg(format!("--player={}", player))
        .arg("--format")
        .arg("{{artist}} - {{title}}")
        .output()
        .expect("Data Command Failed");
    let mut out: Vec<u8> = Vec::new();
    let mut index: u8 = 0;
    for i in details.stdout {
        // Don't include newline characters
        if i == NEWLINE {
            index += 1;
            continue;
        }
        // If Length exceeds Max String Length, Stop appending to string
        if index >= STR_MAX {
            break;
        }
        out.push(i);
        index += 1;
    }

    String::from_utf8_lossy(&out).to_string()
}
