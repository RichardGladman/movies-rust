use crate::model::MovieModel;
use crate::output;

use std::fs;
use std::env;

fn get_file_path() -> String {
    match env::home_dir() {
        Some(home) => {
            return format!("{}/Documents/movies-rust.txt", home.to_str().unwrap());
        },
        None => {
            return String::from("/tmp/movies-rust.txt");
        }
    }
}

pub fn load() -> Vec<MovieModel> {
    let mut data = Vec::new();
    let contents = fs::read_to_string(get_file_path());
    match contents {
        Ok(content) => {
            let lines = content.split("\n");
            for line in lines {
                let fields: Vec<&str> = line.split("||").collect();
                if fields.len() != 5 { continue }
                data.push(MovieModel::new(
                    String::from(fields[0]), 
                    String::from(fields[1]),
                    String::from(fields[2]), 
                    fields[3].parse().unwrap(), 
                    fields[4].parse().unwrap()));
            }
        },
        Err(message) => output::error_message(&format!("Failed to load data file: {}", message).to_string()),
    };

    return data;
}

pub fn save(movies: &Vec<MovieModel>) {
    let mut data = String::new();

    for movie in movies {
        let mut line = &movie.title;
        line.push_str("||");
        line.push_str(&movie.format);
        line.push_str("||");
        line.push_str(&movie.certificate);
        line.push_str("||");
        line.push_str(&movie.rating.to_string());
        line.push_str("||");
        line.push_str(&movie.running_time.to_string());
        line.push('\n');

        data.push_str(line);
    }

    fs::write(get_file_path(), data);
}
