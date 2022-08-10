mod shell;

use shell::shell;
use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


extern crate notify;
use notify::{Watcher, RecursiveMode, watcher};

struct App {
    flags: Vec<String>,
    child_args: Vec<String>,
    child: Option<String>,
    expecting_flags: bool
}

impl App {

    fn new() -> Self {
        Self {
            flags: Vec::<String>::new(),
            child_args: Vec::<String>::new(),
            child: None,
            expecting_flags: true
        }
    }

    fn main(&mut self) {
        // init logging
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "warn");
        }
        pretty_env_logger::init();

        
        // init local vars
        let args: Vec<String> = env::args().skip(1).collect();

        // parse args
        for arg in args {
            self.handle_arg(arg);
        }

        for flag in &self.flags {
            self.handle_flag(&flag);
        }

        if let Some(p) = &self.child {
            self.run_child_process(p);
        } else {
            error!("No child to execute. See usage instructions.");
        }
    }

    fn run_child_process(&self, p: &String) {
        let res = shell(&p, &self.child_args);
        if res.is_err() {

        }
        println!("{:#?}", res);
    }

    fn handle_flag(&self, flag: &String) {
        if let Some(split_values) = flag.split_once("=") {
            let key = split_values.0.trim_start_matches('-');
            let value = split_values.1;

            println!("key: {} value: {}", key, value);
    
        } else {
            self.handle_single_flag(flag);
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

