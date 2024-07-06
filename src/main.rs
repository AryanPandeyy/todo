use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // create new file to store todos
    let _db_file = "todos.txt";
    let _db_path = "/home/ap/code/fun/erus/todo/";

    let mut todo_file = File::create("/home/ap/code/fun/erus/todo/todos.txt")?;

    // DirBuilder::new().create(db_path).unwrap();

    if args[1] == "add" {
        println!("ADD FUNCTION");
        println!("{}", args[2]);
        todo_file.write(args[2].as_bytes()).expect("SOMETHING");
    } else if args[1] == "remove" {
    }
    Ok(())
}
