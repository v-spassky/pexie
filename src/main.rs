//! A CLI tool that generates HTML representation
//! of a directory for Github`s README.md.

#[macro_use]
extern crate clap;
use clap::App;
use serde::Serialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

/// Main function.
fn main() {
    let cwd = env::current_dir().unwrap();

    println!("");
    println!(
        "Launching pexie in the current directory: {}",
        cwd.display()
    );

    let cli_app_config = load_yaml!("cli.yml");
    let cli_args = App::from_yaml(cli_app_config).get_matches();

    let config = PexieConfig {
        entries_to_ignore: cli_args
            .values_of("ignore")
            .unwrap_or(clap::Values::default())
            .collect(),
        sorting_order: cli_args.value_of("order").unwrap_or("dirs-first"),
        output_mode: cli_args.value_of("result").unwrap_or("print"),
    };

    println!("Entries to ignore: {:?}", config.entries_to_ignore);
    println!("Sorting order: {}", config.sorting_order);
    println!("Output mode: {}", config.output_mode);

    let dir_contents = build_dir_contents(Path::new("./"), &config);

    let tera = match Tera::new("templates/**/*.html") {
        Ok(tera_instance) => tera_instance,
        Err(error) => {
            println!("Failed to initialize Tera instance: {:?}", error);
            std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("dir_contents", &dir_contents);

    match tera.render("entrypoint_template.html", &context) {
        Ok(result) => {
            if config.output_mode == "print" {
                println!("Result:");
                println!("{result}");
            } else if config.output_mode == "save" {
                let mut output_file = File::create("pexieoutput.html").unwrap();
                match write!(output_file, "{}", result) {
                    Err(error) => println!(
                        "An error occured when trying to save the result to file: {error:?}"
                    ),
                    _ => (),
                }
                println!("Saved the result to: {}/pexieoutput.html", cwd.display());
            }
        }
        Err(error) => {
            println!("Failed to render template: {:?}", error)
        }
    };

    println!("");
}

/// Represents an entry in a file system.
#[derive(Serialize)]
struct CustomDirEntry {
    name: String,
    is_dir: bool,
    contents: Option<Box<Vec<CustomDirEntry>>>,
}

/// Represents config given fron CLI arguments.
struct PexieConfig<'a, T> {
    entries_to_ignore: Vec<T>,
    sorting_order: &'a str,
    output_mode: &'a str,
}

/// Recursively runs through a given folder and returns representation of it
/// as a vector of CustomDirEntry objects.
fn build_dir_contents(entry_path: &Path, config: &PexieConfig<&str>) -> Vec<CustomDirEntry> {
    let paths = fs::read_dir(entry_path).unwrap();

    let mut dirs = vec![];
    let mut files = vec![];

    for path in paths {
        let entry_name = path
            .as_ref()
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();

        if config.entries_to_ignore.contains(&&entry_name[..]) {
            continue;
        }

        let entry_is_dir = path.as_ref().unwrap().metadata().unwrap().is_dir();

        if entry_is_dir {
            let contents = Box::new(build_dir_contents(&path.unwrap().path().as_path(), &config));
            let entry = CustomDirEntry {
                name: entry_name,
                is_dir: true,
                contents: Some(contents),
            };
            dirs.push(entry);
        } else {
            let entry = CustomDirEntry {
                name: entry_name,
                is_dir: false,
                contents: None,
            };
            files.push(entry);
        }
    }

    let mut entries = vec![];

    if config.sorting_order == "dirs-first" {
        entries.append(&mut dirs);
        entries.append(&mut files);
    } else if config.sorting_order == "files-first" {
        entries.append(&mut files);
        entries.append(&mut dirs);
    }

    entries
}
