extern crate fn_time as ft;
extern crate serde;
extern crate serde_json;
extern crate clap;

mod plotting;

use ft::StreamData;
use std::path::PathBuf;
use std::ops::Range;
use std::fs::File;
use clap::{Arg, App};

pub fn display(path: PathBuf, amount: Range<usize>) -> std::io::Result<()> {
    let input = File::open(path)?;
    let json_data = read_json(input, amount);
    plotting::show(json_data);
    Ok(())
}

fn read_json(ref input: File, amount: Range<usize>) -> Vec<StreamData> {
    let mut stream = serde_json::Deserializer::from_reader(input).into_iter::<StreamData>();
    stream.nth(amount.start);
    stream.take(amount.end - amount.start).map(|v| v.unwrap()).collect()
}

fn get_args() -> (Range<usize>, Option<PathBuf>) {
    let matches = App::new("fn_time_reader")
        .version("0.1.0")
        .author("Tom G. <tomrgowan@gmail.com>")
        .about("Turns output from fn_time into charts")
        .arg(Arg::with_name("file")
             .value_name("FILE")
             .help("log file output from fn_time")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("start")
             .value_name("START")
             .help("Start chart from this point: default is 0")
             .takes_value(true))
        .arg(Arg::with_name("end")
             .value_name("END")
             .help("End chart at this point: default is 10")
             .takes_value(true))
        .get_matches();
    let file_name = matches.value_of("file");
    let start: usize = matches.value_of("start")
        .map_or(0, |v| v.parse::<usize>().unwrap_or(0));
    let end: usize = matches.value_of("end")
        .map_or(10, |v| v.parse::<usize>().unwrap_or(0));
    let amount: Range<usize> = start..end; 
    (amount, 
     file_name.map(|f| {
         let mut path = std::env::current_dir().unwrap();
         path.push(f);
         path
     }))
}

fn main() {
    let (amount, file_path) = get_args();
    if let Some(f) = file_path { display(f, amount).expect("file doesn't exist") }
}
