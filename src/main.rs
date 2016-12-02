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

    let mut t = todo::Todo::new(&path);

    match env::args().len() {
        1 => print!("{}", t),
        2 => match env::args().nth(1) {
            Some(ref arg) if arg.parse::<usize>().is_ok() => {
                let id = arg.parse::<usize>().unwrap();
                if id > 0 && id <= t.len() {
                    t.done(id);
                } else {
                    println!("Note: Maybe It needs a Correct Number.");
                    return;
                }
            },
            Some(arg) => {
                t.add(arg).unwrap();
            }
            _ => print_help(),
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: todo <note/id>. (note to add, id to remove)");
}
