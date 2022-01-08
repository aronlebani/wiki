use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::process::Command;
use walkdir::{WalkDir, DirEntry};
use fancy_regex::Regex;
use structopt::StructOpt;
use colored::Colorize;
use chrono::offset::Local;

#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(help = "Open memo")]
    Memo(Memo),

    #[structopt(help = "Open todo")]
    Todo,

    #[structopt(help = "Query items in wiki")]
    Query(Query),
}

#[derive(Debug, StructOpt)]
struct Memo {
    #[structopt(short, long, help = "Create new item")]
    new: bool,
}

#[derive(Debug, StructOpt)]
struct Query {
    #[structopt(short, long, default_value = "", help = "List of tags to query")]
    tags: String,

    #[structopt(short, long, default_value = "", help = "Arbitrary query string")]
    query: String,

    #[structopt(short, long, help = "Include only unchecked checkbox items")]
    check: bool,

    #[structopt(short, long, help = "Include only links")]
    anchor: bool,
}

fn get_re(opt: Query) -> Regex {
    let mut re_str: String = String::from("");

    if opt.tags != "" {
        for tag in opt.tags.split(" ") {
            re_str = format!("{}(?=.* _{}[^_]*_)", re_str, tag);
        }
    }

    if opt.query != "" {
        for keyword in opt.query.split(" ") {
            re_str = format!("{}(?=.*{}*)", re_str, keyword);
        }
    }

    if opt.anchor {
        re_str = format!("{}(?=.*\\[.*\\]\\(.*\\))", re_str);
    }

    if opt.check {
        re_str = format!("- \\[ \\]{}", re_str);
    }

    Regex::new(&re_str).unwrap()
}

fn query(opt: Query) -> () {
    let re = get_re(opt);

    let is_file = |entry: &DirEntry| -> bool {
        entry.path().is_file()
    };

    let is_markdown = |entry: &DirEntry| -> bool {
        match entry.path().extension() {
            Some(ext) => ext == "md",
            _ => false,
        }
    };

    let print_match = |path: &str, line_no: &str, line: &str| -> () {
        println!("{}:{}: {}", path.green(), line_no.cyan(), line);
    };

    let parse_line = |path: &Path, line_no: u8, line: &str| -> () {
        let is_match = re.is_match(line).unwrap(); 
        match is_match {
            true => print_match(path.to_str().unwrap(), &format!("{}", line_no), line),
            false => (),
        }
    };

    let parse_file = |entry: DirEntry| -> () {
        let path = entry.path();
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
            .enumerate()
            .for_each(|(line_no, line)| parse_line(path, line_no as u8, &line.unwrap()));
    };

    let path = "/Users/aron/Dropbox/Wiki";

    WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_file(e))
            .filter(|e| is_markdown(e))
            .for_each(|e| parse_file(e));
}

fn memo(opt: Memo, path: String) {
    let full_path = format!("{}memo.md", path);

    if opt.new {
        let date = Local::now();
        let heading = date.format("\n## [%d-%m-%Y]\n").to_string();
        let cmd = format!("echo \"{}\" >> {} && vim {}", heading, full_path, full_path);

        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("Error: Failed to run editor")
                .wait()
                .expect("Error: Editor returned a non-zero status");
    } else {
        let cmd = format!("vim {}", full_path);

        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("Error: Failed to run editor")
                .wait()
                .expect("Error: Editor returned a non-zero status");
    }
}

fn todo(path: String) {
    let full_path = format!("{}todo.md", path);
    let cmd = format!("vim {}", full_path);

    Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("Error: Failed to run editor")
            .wait()
            .expect("Error: Editor returned a non-zero status");
}

fn get_config() -> String {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.wiki", home);

    std::fs::read_to_string(config_path)
        .expect("Error: Could not read file")
        .replace("\n", "")
}

fn main() {
    let opt = Opt::from_args();
    let config = get_config();

    match opt {
        Opt::Query(opt) => query(opt),
        Opt::Memo(opt) => memo(opt, config),
        Opt::Todo => todo(config),
    };
}
