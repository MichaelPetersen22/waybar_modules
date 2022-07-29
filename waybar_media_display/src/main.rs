// Requires otf-fontawesome (For ICON), playerctl to be installed

use std::{process::Command};

// STR_MAX Length of String before it is trimmed and appended ...
const STR_MAX: u8 = 40;
// Newline character in Decimal
const NEWLINE: u8 = 10;

// CUSTOM SETTINGS
const ICON: char = '';

// Syntax is --player=yourplayer, leave blank if you want it to try and run with any media player currently playing
static PLAYERCMD: &'static str = "";

fn main() {
    let mut text = String::new();
    let status: String = get_status();
    // Decide what to do depending on status output
    if status == "Playing" {
        let mut append: &str = "";
        let data: String = get_data();

        if data.len() as u8 >= STR_MAX {
            append = "...";
        }

        text = format!("{}{} {}", data.trim(), append, ICON)
    } else if status == "Paused" {
        text = ICON.to_string();
    }

    println!("{{\"text\":\"{}\", \"class\":\"{}\"}}", text, status);
}

fn get_status() -> String {
    let status = Command::new("playerctl")
        .arg("status")
        .arg(PLAYERCMD)
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

fn get_data() -> String {
    let details = Command::new("playerctl")
        .arg("metadata")
        .arg(PLAYERCMD)
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
