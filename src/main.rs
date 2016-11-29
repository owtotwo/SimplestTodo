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
            Some(ref arg) if arg.parse::<usize>().and_then(|id| {
                if id > 0 && id <= t.len() { Ok(id) }
                else { "".parse::<usize>() } // hack
            }).is_ok() => {
                match arg.parse::<usize>() {
                    Ok(id)  => t.done(id),
                    _ => unreachable!(),
                };
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
