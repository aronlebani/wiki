use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::default::Default;
use serde::{Deserialize};
use walkdir::{WalkDir, DirEntry};
use fancy_regex::Regex;
use structopt::StructOpt;
use colored::Colorize;
use chrono::offset::Local;
use toml;

#[derive(Debug, Deserialize)]
struct Cfg {
    path: String,
    editor: String,
}

impl Default for Cfg {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();
        Self { path: home, editor: "vim".to_string() }
    }
}

#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(help = "Open memo")]
    Memo(Memo),

    #[structopt(help = "Open todo")]
    Todo,

    #[structopt(help = "Open notes")]
    Notes,

    #[structopt(help = "Query items in wiki")]
    Query(Query),

    #[structopt(help = "List tags")]
    Tags,

    #[structopt(help = "Go to line in file")]
    Go(Go),

    #[structopt(help = "Open home")]
    Home
}

#[derive(Debug, StructOpt)]
struct Memo {
    #[structopt(short, long, help = "Create new item")]
    new: bool,
}

#[derive(Debug, StructOpt)]
struct Go {
    #[structopt(short, long, help = "File path")]
    file: String,

    #[structopt(short, long, help = "Line number")]
    line: String,
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

fn execute(cmd: String) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
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
            re_str = format!("{}(?i)(?=.*{}*)", re_str, keyword);
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

fn pad(string: String, chars: u8) -> Result<String, String> {
    match string.chars().count() {
        x if x as u8 > chars => Err(string),
        x if x as u8 == chars => Ok(string),
        _ => pad(format!("{} ", string), chars),
    }
}

fn query(opt: Query, cfg: Cfg) -> () {
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
        println!("{}:{}: {}", path.green(), pad(String::from(line_no), 4).unwrap().cyan(), line);
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

        reader
            .lines()
            .enumerate()
            .for_each(|(line_no, line)| parse_line(path, line_no as u8, &line.unwrap()));
    };

    WalkDir::new(cfg.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_file(e))
        .filter(|e| is_markdown(e))
        .for_each(|e| parse_file(e));
}

fn tags(cfg: Cfg) -> () {
    let re_str = "_[^_]*_";
    let re = Regex::new(&re_str).unwrap();

    let is_file = |entry: &DirEntry| -> bool {
        entry.path().is_file()
    };

    let is_markdown = |entry: &DirEntry| -> bool {
        match entry.path().extension() {
            Some(ext) => ext == "md",
            _ => false,
        }
    };

    let print_match = |line: &str| -> () {
        println!("{}", line);
    };

    let parse_line = |line: &str| -> () {
        let is_match = re.is_match(line).unwrap(); 
        match is_match {
            true => print_match(line),
            false => (),
        }
    };

    let parse_file = |entry: DirEntry| -> () {
        let path = entry.path();
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .enumerate()
            .for_each(|(_, line)| parse_line(&line.unwrap()));
    };

    WalkDir::new(cfg.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_file(e))
        .filter(|e| is_markdown(e))
        .for_each(|e| parse_file(e));
}

fn memo(opt: Memo, cfg: Cfg) {
    if opt.new {
        let date = Local::now();
        let heading = date.format("\n## [%d-%m-%Y]\n").to_string();
        let full_path = format!("{}memo.md", cfg.path);
        let cmd = format!("echo \"{}\" >> {} && cd {} && {} memo.md", heading, full_path, cfg.path, cfg.editor);

        execute(cmd);
    } else {
        let cmd = format!("cd {} && {} memo.md", cfg.path, cfg.editor);

        execute(cmd);
    };
}

fn todo(cfg: Cfg) {
    let cmd = format!("cd {} && {} todo.md", cfg.path, cfg.editor);

    execute(cmd);
}

fn notes(cfg: Cfg) {
    let cmd = format!("cd {}notes && {} .", cfg.path, cfg.editor);

    execute(cmd);
}

fn home(cfg: Cfg) {
    let cmd = format!("cd {} && {} index.md", cfg.path, cfg.editor);

    execute(cmd);
}

fn go(opt: Go, cfg: Cfg) {
    let cmd = format!("cd {} && {} +{} {}", cfg.path, cfg.editor, opt.line, opt.file);

    execute(cmd);
}

fn get_config() -> Cfg {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.wiki", home);
    let config_content = std::fs::read_to_string(config_path).expect("Error reading config file");

    toml::from_str(&*config_content).expect("Error parsing config file")
}

fn main() {
    let opt = Opt::from_args();
    let cfg = get_config();

    match opt {
        Opt::Query(opt) => query(opt, cfg),
        Opt::Memo(opt) => memo(opt, cfg),
        Opt::Todo => todo(cfg),
        Opt::Notes => notes(cfg),
        Opt::Go(opt) => go(opt, cfg),
        Opt::Home => home(cfg),
        Opt::Tags => tags(cfg),
    };
}
