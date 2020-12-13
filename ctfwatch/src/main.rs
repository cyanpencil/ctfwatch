//use chrono::Utc;
use chrono::{FixedOffset, Duration, DateTime};
//use ctftimebot::{CtfEvent, CONFIG};
//use log::{error, info};
//use slack_hook::{PayloadBuilder, Slack};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CtfTeam {
    id: usize,
    name: String
}

#[derive(Debug, Deserialize)]
pub struct CtfEvent {
    title: String,
    ctftime_url: String,
    id: usize,
    start: DateTime<FixedOffset>,
    finish: DateTime<FixedOffset>,
    logo: Option<String>,
    url: Option<String>,
    format: String,
    public_votable: bool,
    weight: f32,
    live_feed: Option<String>,
    restrictions: String,
    location: Option<String>,
    onsite: bool,
    organizers: Vec<CtfTeam>,
    ctf_id: usize,
    participants: usize,
}

fn format_duration(d: &Duration) -> String {
    let mut d = *d;
    let mut tmp = Vec::with_capacity(4);
    if d.num_hours() > 48 {
        tmp.push(format!("{} days", d.num_days()));
        d = d + Duration::days(-d.num_days());
    }
    if d.num_hours() > 0 {
        tmp.push(format!("{} hours", d.num_hours()));
        d = d + Duration::hours(-d.num_hours());
    }
    if d.num_minutes() > 0 {
        tmp.push(format!("{} minutes", d.num_minutes()));
        d = d + Duration::minutes(-d.num_minutes());
    }
    if d.num_seconds() > 0 {
        tmp.push(format!("{} seconds", d.num_seconds()));
    }
    tmp.join(" ")
}

fn strip_ansi(s: &String) -> String {
    String::from_utf8(strip_ansi_escapes::strip(s).unwrap()).unwrap()
}

fn extra_bytes (s1: &String) -> usize {
    s1.len() - strip_ansi(&s1).chars().count()
}

fn double_line (s1: &String, s2: &String) -> String {
    let mut fin = format!("│ {}", s1);
    let fin_len = strip_ansi(&fin).chars().count();
    let fin_len2 = strip_ansi(&s2).chars().count();
    if fin_len > LEN {
        return fin;
    }
    fin.push_str(&format!(" {: >1$} │", format!(" {}", s2), LEN - 1 - fin_len + (s2.chars().count() - fin_len2)));
    fin
}

fn blue<S: AsRef<str>>(s: S) -> String {
    format!("{}{}{}", "\x1b[38;2;174;129;255m", s.as_ref(), "\x1b[0m")
}

fn bold(s: impl std::fmt::Display) -> String {
    format!("{}{}{}", "\x1b[1m", format!("{}", s), "\x1b[0m")
}

fn pink(s: impl std::fmt::Display) -> String {
    format!("{}{}{}", "\x1b[38;2;249;38;114;1m", format!("{}", s), "\x1b[0m")
}

fn gray(s: impl std::fmt::Display) -> String {
    format!("{}{}{}", "\x1b[38;2;102;217;239m", format!("{}", s), "\x1b[0m")
}

fn green(s: impl std::fmt::Display) -> String {
    format!("{}{}{}", "\x1b[38;2;166;226;46m", format!("{}", s), "\x1b[0m")
}

static LEN: usize = 60;
fn justdoit() -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<CtfEvent> = reqwest::blocking::get("https://ctftime.org/api/v1/events/?limit=10")?.json().unwrap();

    for event in v {
        let mut text : Vec<String>  = vec![];
        let title = format!(" {} ", pink(event.title));
        text.push(format!("┌{:─^1$}┐", title, LEN + extra_bytes(&title)));

        let mut duration = format_duration(&event.finish.signed_duration_since(event.start));
        duration.truncate(8);
        text.push(double_line(&format!("{}: {}", blue("Date"), bold(event.start)),
                              &format!("{}: {}", blue("Duration"), bold(duration))));

        text.push(double_line(&format!("{}: {}", blue("Organizers"), green(&event.organizers[0].name)),
                              &format!("{}: {}", blue("Category"), bold(event.format))));

        text.push(double_line(&format!("{}: {}", blue("Restrictions"), bold(event.restrictions)),
                              &format!("{}: {}", blue("Weight"), bold(event.weight))));

        text.push(double_line(&format!("{}: {}", blue("Onsite"), bold(event.onsite)),
                              &format!("{}: {}", blue("Participants"), bold(event.participants))));

        text.push(double_line(&format!("{}: {}", blue("ctftime url"), gray(event.ctftime_url)),
                              &format!("")));

        text.push(double_line(&format!("{}: {}", blue("website"), gray(event.url.unwrap_or(String::from("None")))),
                              &format!("")));

        text.push(format!("└{:─^1$}┘", "",  LEN));

        let mut downloaded = false;
        let img : Vec<u8>;

        if let Some(url) = event.logo {
            if url.len() > 5 {
                let mut buf: Vec<u8> = Vec::new();
                reqwest::blocking::get(&url)?.copy_to(&mut buf)?;

                let mut file = File::create("/tmp/catastrophic_failure")?;
                file.write_all(&buf)?;

                downloaded = true;
            }
        }



        if downloaded {
            img = std::process::Command::new("chafa").arg("/tmp/catastrophic_failure")
                                            .arg("--size").arg(&format!("40x{}", text.len()))
                                            .output().expect("Failed running chafa").stdout;
        }
        else {
            img = vec![b'\n'; text.len()];
        }

        for (i, s) in img.split(|c| *c == b'\n').enumerate() {
            if i < text.len() {print!("{}  ", text[i]);}
            if downloaded {
                std::io::stdout().write_all(&s)?;
            }
            println!();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("# author: cyanpencil      source: github.com/cyanpencil/ctfwatch");
    println!("# data pulled from ctftime.org");
    println!();
    justdoit()
}
