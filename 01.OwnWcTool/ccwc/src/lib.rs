use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum CountOption {
    Words,
    Lines,
    Bytes,
    Characters,
}

#[derive(Debug)]
pub struct Config {
    pub count_option: Vec<CountOption>,
    pub file_path: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            return Err("not enough arguments");
        }

        let mut file_path: Vec<String> = Vec::new();
        let mut count_option: Vec<CountOption> = Vec::new();

        for arg in args.iter().skip(1) {
            if arg.starts_with('-') {
                match arg.as_str() {
                    "-w" => count_option.push(CountOption::Words),
                    "-l" => count_option.push(CountOption::Lines),
                    "-c" => count_option.push(CountOption::Bytes),
                    "-m" => count_option.push(CountOption::Characters),
                    _ => return Err("invalid count option"),
                }
            } else {
                file_path.push(arg.clone());
            }
        }

        if count_option.is_empty() {
            count_option.push(CountOption::Lines);
            count_option.push(CountOption::Words);
            count_option.push(CountOption::Bytes);
        }

        Ok(Config {
            count_option,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut num: Vec<usize> = Vec::new();
    let mut total: Vec<usize> = vec![0; config.count_option.len()];

    if config.file_path.is_empty() {
        return run_standard_input(config);
    }

    for file_path in config.file_path.iter() {
        let contents = fs::read_to_string(file_path)?;
        for count_option in &config.count_option {
            num.push(count_words(count_option, &contents));
            print!("{}\t", num.last().unwrap());
        }
        for i in 0..num.len() {
            total[i] += num[i];
        }
        println!("{}", &file_path);
        num.clear();
    }

    if config.file_path.len() > 1 {
        for i in 0..total.len() {
            print!("{}\t", total[i]);
        }
        println!();
    }

    Ok(())
}

pub fn run_standard_input(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = io::read_to_string(std::io::stdin())?;
    for count_option in &config.count_option {
        print!("{}\t", count_words(count_option, &contents));
    }
    println!();
    Ok(())
}

pub fn count_words(count_option: &CountOption, contents: &str) -> usize {
    match count_option {
        CountOption::Words => contents.split_whitespace().count(),
        CountOption::Lines => contents.lines().count(),
        CountOption::Bytes => contents.len(),
        CountOption::Characters => contents.chars().count(),
    }
}
