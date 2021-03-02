use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use flate2::read::GzDecoder;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::Deserialize;

pub struct Fdns {
    file: String,
    options: Options,
}

pub struct Options {
    value: bool, // Should match on the value field, if false we'll use the name field
    regex: regex::Regex,
    kind: String, // A, AAAA, CNAME, PTR, NS
    allow_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub timestamp: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entry_kind: String,
    pub value: String,
}

impl Options {
    pub fn new(value: bool, regex: regex::Regex, kind: String, allow_list: Vec<String>) -> Options {
        Options {
            value,
            regex,
            kind,
            allow_list,
        }
    }
}

impl Fdns {
    pub fn new(file: String, options: Options) -> Fdns {
        Fdns { file, options }
    }

    fn is_allowed(&self, d: String) -> bool {
        let list = self.options.allow_list.clone();
        list.is_empty() || list.into_par_iter().any(|p| d.ends_with(&p))
    }

    pub fn read(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        let file = File::open(self.file.clone())?;

        Ok(BufReader::new(GzDecoder::new(file))
            .lines()
            .par_bridge()
            .filter_map(|line| serde_json::from_str(&line.expect("msg")).ok())
            .filter(|e: &Entry| {
                if e.entry_kind != self.options.kind {
                    return false;
                }

                match self.options.value {
                    true => {
                        self.is_allowed(e.value.clone()) && self.options.regex.is_match(&e.value)
                    }
                    false => {
                        self.is_allowed(e.name.clone()) && self.options.regex.is_match(&e.name)
                    }
                }
            })
            .collect::<Vec<Entry>>())
    }
}
