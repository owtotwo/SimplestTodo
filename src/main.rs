#[macro_use]
extern crate json;

use std::env;

mod todo;

fn main() {
    let env_key = "TODO_PATH";

    let path: String = match env::var(env_key) {
        Ok(val) => val,
        Err(_) => {
            println!("Please Set Environment Variable `{}`.", env_key);
            return;
        }
    };

    let mut todolist = todo::Todo::new(&path);

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print!("{}", todolist),
        2 => {
            // if args[1] is a number, regrads it as an index
            if let Ok(id) = args[1].parse() {
                if id > 0 && id <= todolist.len() {
                    todolist.done(id);
                } else {
                    println!("Note: Maybe It needs a Correct Number.");
                }
            // want to sync or add a note
            } else {
                // update it
                match args[1] {
                    ref s if s == "help" || s == "--help" => print_help(),
                    ref s if s == "sync" => todolist.sync(),
                    ref s if s == "upload" => todolist.upload(),
                // regrads it as a note
                    ref s => todolist.add(s),
                };
            }
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: todo [<note>/<id>/sync/upload/help].\n\
              i.e. :  todo 'I love rust'  -- add a note \n        \
                      todo 2              -- delete the second item \n        \
                      todo upload         -- upload local `.todo` file to gist \n        \
                      todo sync           -- sync todolist by gist\n\
              -- Copyright (c) 2016 sysu_AT <owtotwo@163.com> --");
}
