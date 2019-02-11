use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

const SOURCE_CSV: &str = "/Users/p772733/Downloads/sieble.CSV";
const DESTINATION_DIR: &str = "/Users/p772733/Downloads/sieble";

fn main() {
    let now = Instant::now();
    split().unwrap();
    println!("{}", now.elapsed().as_secs());
}

fn split() -> std::io::Result<()> {
    if !file_exists(SOURCE_CSV) {
        panic!("{} doesn't exist.", SOURCE_CSV);
    }

    let f = File::open(SOURCE_CSV)?;
    let mut reader = BufReader::new(f);
    let mut line = String::with_capacity(100);
    let mut submissions: Vec<Vec<String>> = Vec::with_capacity(200);
    let mut old_group_id = String::from("");
    let mut first_run = true;
    let pool = threadpool::ThreadPool::new(8);
    loop {
        let len = reader.read_line(&mut line)?;

        // EOF
        if len == 0 {
            // do something with sumissions of same group
            let job = submissions.clone();
            pool.execute(move || {
                let json = create_json(&job);
                create_file(old_group_id.as_str(), &json).unwrap();
            });
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
            if first_run {
                first_run = false;
            } else {
                // do something with sumissions of same group
                let job = submissions.clone();
                pool.execute(move || {
                    let json = create_json(&job);
                    create_file(old_group_id.as_str(), &json).unwrap();
                });
                old_group_id = fields[6].clone();
                line.clear();
                submissions.clear();
                submissions.push(fields);
                continue;
            }
            old_group_id = fields[6].clone();
        }

        submissions.push(fields);

        line.clear();
    }
    pool.join();
    Ok(())
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn create_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut path = String::from(DESTINATION_DIR);
    path.push_str("/");
    path.push_str(filename);
    path.push_str(".json");
    let mut buffer = File::create(path.as_str())?;
    buffer.write_all(content.as_bytes())?;
    Ok(())
}

fn create_json(data: &[Vec<String>]) -> String {
    format!("{:#?}", data)
}
