use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use walkdir::{WalkDir, DirEntry};
use fancy_regex::Regex;
use structopt::StructOpt;
use colored::Colorize;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "t", long = "tags", default_value = "")]
    tags: String,

    #[structopt(short = "q", long = "query", default_value = "")]
    query: String,

    #[structopt(short = "c", long = "check")]
    check: bool,

    #[structopt(short = "a", long = "anchor")]
    anchor: bool,

    #[structopt(short = "l", long = "list")]
    list: bool,

    #[structopt(short = "v", long = "validate")]
    validate: bool,
}

fn get_re(opt: Opt) -> Regex {
    let mut re_str: String = String::from("");

    if opt.tags != "" {
        for tag in opt.tags.split(" ") {
            re_str = format!("{}(?=.*_{}[^_]*_)", re_str, tag);
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

    println!("{}", re_str);

    Regex::new(&re_str).unwrap()
}

fn query(opt: Opt) -> () {
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

    WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_file(e))
        .filter(|e| is_markdown(e))
        .for_each(|e| parse_file(e));
}

fn list(opt: Opt) -> () {

}

fn validate(opt: Opt) -> () {
}

fn main() {
    let opt = Opt::from_args();

    if opt.list {
        list(opt);
    } else if opt.validate {
        validate(opt);
    } else {
        query(opt);
    };
}
