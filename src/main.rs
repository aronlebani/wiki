use std::fmt::Debug;
use regex::Regex;
use structopt::StructOpt;

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
    let mut re_str = "";

    if opt.tags != "" {
        for tag in opt.tags.split(" ") {
            re_str += format!("(?=.*_{}[^_]*_)", tag);
        }
    }

    if opt.query != "" {
        for keyword in opt.query.split(" ") {
            re_str += format!("(?=.*{}*)", keyword);
        }
    }

    if opt.anchor {
        re_str += "(?=.*\[.*\]\(.*\))";
    }

    if opt.check {
        re_str =+ "- [ ]"
    }

    return Regex::new(re_str).unwrap();
}

enum Colour {
    Green = "32",
    Blue = "34",
}

fn colorize(string: String, colour: Colour) -> String {
    return format!("\e[{}m{}\e[0m", colour_code, string);
}

fn colorize_green(string: String) -> String {
    return colorize(string, Colour::Green);
}

fn colorize_blue(string: String) -> String {
    return colorize(string, Colour::Blue);
}

fn main() {
    println!("Hello, world!");
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
