mod shell;

use shell::shell;
use std::{env, io, process::{Stdio, Command}, path::{Path, PathBuf}};

extern crate pretty_env_logger;
#[macro_use] extern crate log;


extern crate notify;
use notify::{Watcher, RecursiveMode, watcher};

struct App {
    flags: Vec<String>,
    child_args: Vec<String>,
    child: Option<String>,
    expecting_flags: bool,
    watched_files: Vec<PathBuf>
}

impl App {

    fn new() -> Self {
        Self {
            flags: Vec::<String>::new(),
            child_args: Vec::<String>::new(),
            child: None,
            expecting_flags: true,
            watched_files: Vec::<PathBuf>::new()
        }
    }

    fn main(&mut self) {
        // init logging
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "warn");
        }
        pretty_env_logger::init();

        
        let args: Vec<String> = env::args().skip(1).collect();

        for arg in args {
            self.handle_arg(arg);
        }

        for flag in self.flags.clone() {
            
            if let Some(split_values) = flag.split_once("=") {
                let key = split_values.0.trim_start_matches('-');
                let value = split_values.1;
                self.handle_config(key, value);
        
            } else {
                self.handle_single_flag(&flag);
            }
        }

        if let Some(p) = &self.child {
            self.run_child_process(p);
        } else {
            error!("No child to execute. See usage instructions.");
        }
    }

    fn run_child_process(&self, p: &String) {

        let res = Command::new(&p)
            .args(self.child_args.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match res {
            Err(e) => {
                error!("{}", e.to_string());
            }
            Ok(mut child) => {
                let child_exit = child.wait().unwrap();
                debug!("Child exit status: {}", child_exit);
            },
        }
    }

    fn handle_single_flag(&self, flag: &str) {
        todo!()
    }

    fn handle_arg(&mut self, arg: String) {
        if self.expecting_flags {
            if arg.starts_with('-') {
                self.flags.push(arg);
            } else {
                self.child = Some(arg);
                self.expecting_flags = false;
            }
        } else {
            self.child_args.push(arg)
        }
    }

    fn handle_config(&mut self, key: &str, value: &str) {
        match key {
            "watch" => {
                let potential_file = Path::new(value);
                if potential_file.exists() {
                    self.watched_files.push(potential_file.to_path_buf());
                } else {
                    warn!("File '{}' does not exist.", value);
                }
            }
            _ => {
                error!("Unknown config key: '{}'", key);
            }
        } 
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut app = App::new();
    app.main();
}

