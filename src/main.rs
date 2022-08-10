mod shell;

use shell::shell;
use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn");
    }
    pretty_env_logger::init();
    let mut flags: Vec<String> = Vec::<String>::new();
    let mut expecting_flags = true;
    let args: Vec<String> = env::args().skip(1).collect();
    let mut child: Option<String> = None;
    let mut child_args: Vec<String> = Vec::<String>::new();
    for arg in args {
        if expecting_flags {
            if arg.starts_with('-') {
                flags.push(arg);
            } else {
                child = Some(arg);
                expecting_flags = false;
            }
        } else {
            child_args.push(arg)
        }
    }

    if let Some(p) = child {
        let res = shell(&p, &child_args);
        if res.is_err() {

        }
        println!("{:#?}", res);
    } else {
        error!("No child to execute. See usage instructions.");
    }
}

