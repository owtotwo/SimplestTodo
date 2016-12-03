mod todo;

use std::env;

fn main() {
    let key = "TODO_PATH";
    let path: String;

    match env::var(key) {
        Ok(val) => path = val,
        Err(_) => {
            println!("Please Set Environment Variable `{}`.", key);
            return;
        }
    }

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
            // regrads it as a note
            } else {
                todolist.add(args[1].clone()).unwrap();
            }
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: todo <note/id>. (note to add, id to remove)");
}
