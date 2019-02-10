use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const SOURCE_CSV: &str = "/Users/jiqiang/Development/sieble_file_splitter/sieble.tiny.CSV";

fn main() {
    match split() {
        Err(e) => println!("Error: {}", e),
        _ => println!("Done"),
    };
}

fn split() -> std::io::Result<()> {
    if !file_exists(SOURCE_CSV) {
        panic!("{} doesn't exist.", SOURCE_CSV);
    }

    let f = File::open(SOURCE_CSV)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut submissions: Vec<Vec<String>> = Vec::new();
    let mut old_group_id = String::from("");
    let mut first_run = true;
    loop {
        let len = reader.read_line(&mut line)?;

        // EOF
        if len == 0 {
            // do something with sumissions of same group
            println!("{:?}", submissions.len());
            break;
        }

        // skip first line
        if line.starts_with("EBL_ROW_ID") {
            line.clear();
            continue;
        }

        // split line into fields
        let fields: Vec<String> = line
            .split("\",\"")
            .map(|field| field.trim().trim_matches('"').to_owned())
            .collect();

        // skip invalid line
        if fields.len() != 84 {
            line.clear();
            continue;
        }

        // group submissions by same group id
        if old_group_id.ne(&fields[6]) {
            old_group_id = fields[6].clone();

            if first_run {
                first_run = false;
            } else {
                // do something with sumissions of same group
                println!("{:?}", submissions.len());

                line.clear();
                submissions.clear();
                submissions.push(fields);
                continue;
            }
        }

        submissions.push(fields);

        line.clear();
    }

    Ok(())
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
