use serde_json::{self, Value};
use std::{
    error::Error,
    fs,
    process::{Command, Output},
};

const EWW_DIR: &str = "$HOME/.config/eww";

pub struct Config {
    theme: String,
    colorscheme: String,
    eww_dir: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // unnecessary first arg
        args.next();

        let mut extracted_args: Vec<String> = vec![];
        let mut options: Vec<String> = vec![];

        for arg in args {
            if arg.starts_with("--") {
                // it's an optional argument
                options.push(arg);
            } else {
                // it's an argument
                extracted_args.push(arg);
            }
        }

        if extracted_args.len() < 1 {
            return Err("Not enough arguments (1 minimum: the theme's name)");
        }

        let mut extracted_args_iter = extracted_args.into_iter();
        let options_iter = options.into_iter();

        let theme = match extracted_args_iter.next() {
            Some(v) => v,
            None => String::from("default"),
        };
        let colorscheme = match extracted_args_iter.next() {
            Some(v) => v,
            None => String::from("default"),
        };

        let mut eww_dir = get_eww_dir();

        for arg in options_iter {
            if arg.starts_with("--config=") {
                eww_dir = parse_env(&arg[9..]);
            }
        }

        Ok(Config {
            theme,
            colorscheme,
            eww_dir,
        })
    }
}

pub fn parse_env(content: &str) -> String {
    let mut options = envmnt::ExpandOptions::new();
    options.expansion_type = Some(envmnt::ExpansionType::Unix);
    envmnt::expand(content, Some(options))
}

pub fn get_eww_dir() -> String {
    parse_env(EWW_DIR)
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let res = fs::read_to_string(path)?;
    Ok(res)
}

pub fn update_var(key: &String, value: &Value) -> Result<Output, Box<dyn Error>> {
    let mut binding = Command::new("eww");

    let command = binding.arg("update");

    let mut stringed_value = value.to_string();
    let mut arg: String = String::from(key);
    if value.is_string() {
        // remove
        stringed_value.remove(0);
        stringed_value.pop();

        arg = arg + "=" + stringed_value.as_str();
    } else {
        arg = arg + "=" + stringed_value.as_str();
    }

    println!("{}", arg);

    let output = command.arg(arg).output()?;

    Ok(output)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("eww dir: {}", config.eww_dir);
    println!("theme: {}", config.theme);
    println!("colorscheme: {}", config.colorscheme);

    let path = config.eww_dir + "/themes/" + config.theme.as_str() + "/eww-vars.json";
    let content = read_file(path.as_str())?;

    let object: serde_json::Value = serde_json::from_str(content.as_str())?;

    let eww_vars = object.as_object();

    let mut variables: Option<&Value> = None;

    if let Some(ev) = eww_vars {
        for (key, value) in ev {
            if key.eq(&config.colorscheme) {
                variables = Some(value);
            } else if key.eq("default") && variables.is_none() {
                variables = Some(value);
            }
        }

        if let Some(vars) = variables {
            println!();
            for (key, value) in vars.as_object().unwrap() {
                update_var(key, value)?;
            }
        }
    }

    println!();
    println!("ok");

    Ok(())
}
