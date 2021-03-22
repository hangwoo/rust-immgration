// use serde_json::{Value};
// use std::fs::read_to_string;
// use immigration::helper::*;
use clap::Clap;

/// json 파일이 하나의 언어로 이루어져 있는지 검사합니다
#[derive(Clap)]
#[clap(version = "1.0", author = "hangwoo Kim. <marong61@gmail.com>")]
struct Opts {
    /// json 파일이 있는 directory 경로를 받습니다.
    #[clap(short, long)]
    path: String,
    /// json 파일의 경로를 받습니다. directory path 이 있는 경우 무시됩니다.
    #[clap(short, long, default_value = "default.json")]
    file: String,
    /// verbose 단계는 나중에 구현하자...
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let opt: Opts = Opts::parse();
    println!("config: {}", opt.path);
    println!("input: {}", opt.file);
    println!("verbose: {}", opt.verbose);
    // let json_file = read_to_string("common.json")?.parse::<String>()?;
    // let json = serde_json::from_str::<Value>(&json_file)?;
    // if json.is_object() {
    //     iterate_object(&json);
    // }
    Ok(())
}
