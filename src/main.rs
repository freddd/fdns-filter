use crate::fdns::Entry;
use std::fs;

use clap::{App, Arg};
use fdns::Options;
use prettytable::{Cell, Row, Table};
use regex::Regex;

mod fdns;
fn main() {
    let matches = App::new("fdns-filter")
        .version("1.0")
        .author("freddd")
        .about("filter rapid7 fdns files")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("table or json output")
                .default_value("table")
                .env("FDNS_OUTPUT"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("path to fdns gzip file")
                .required(true)
                .env("FDNS_PATH"),
        )
        .arg(
            Arg::with_name("regex")
                .short("r")
                .long("regex")
                .takes_value(true)
                .required(true)
                .help("regex pattern to use as filter")
                .env("FDNS_REGEX"),
        )
        .arg(
            Arg::with_name("kind")
                .short("k")
                .long("kind")
                .takes_value(true)
                .required(true)
                .help("which kind to look for A, AAAA, TXT, MX or CNAME")
                .env("FDNS_KIND"),
        )
        .arg(
            Arg::with_name("value")
                .short("v")
                .long("value")
                .help("filter on value field (if omitted it automatically uses the name field)"),
        )
        .arg(
            Arg::with_name("allow-list")
                .short("al")
                .long("allow-list")
                .takes_value(true)
                .help("path to txt containing allowed domains")
                .env("FDNS_ALLOW_LIST"),
        )
        .get_matches();

    let regex = matches.value_of("regex").unwrap();
    let re = Regex::new(regex).unwrap();

    let kind = matches.value_of("kind").unwrap();
    let fdns_file = matches.value_of("path").unwrap();

    let allow_list = match matches.value_of("allow-list") {
        Some(path) => read_csv(path),
        None => {
            vec![]
        }
    };

    let result = fdns::Fdns::new(
        fdns_file.to_string(),
        Options::new(
            matches.is_present("value"),
            re,
            kind.to_lowercase(),
            allow_list,
        ),
    )
    .read();

    match result {
        Ok(entries) => match matches.value_of("output").unwrap() {
            "json" => println!("{:#?}", serde_json::to_string(&entries).unwrap()),
            "table" => {
                print_as_table(entries);
            }
            _ => unreachable!(),
        },
        Err(e) => {
            println!("{:#?}", e)
        }
    }
}

fn print_as_table(entries: Vec<Entry>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("NAME"),
        Cell::new("VALUE"),
        Cell::new("TYPE"),
        Cell::new("TIMESTAMP"),
    ]));

    for entry in entries {
        table.add_row(Row::new(vec![
            Cell::new(&entry.name),
            Cell::new(&entry.value),
            Cell::new(&entry.kind),
            Cell::new(&entry.timestamp),
        ]));
    }

    table.printstd();
}

fn read_csv(path: &str) -> Vec<std::string::String> {
    let content = fs::read_to_string(path).unwrap();
    content
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
