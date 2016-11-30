extern crate json;

use std::path::Path;
use std::fs::File;
use std::fmt;
use std::result;
use std::ops::Drop;
use std::io::{Read, Write};

use self::json::JsonValue;

pub struct Todo<'a> {
    file_path: &'a Path,
    items: Vec<Item>,
}

struct Item {
    content: String,
}

type Result<T> = result::Result<T, &'static str>;

impl<'a> Todo<'a> {
    pub fn new(path: &str) -> Todo {
        let mut todo = Todo {
            file_path: Path::new(path),
            items: Vec::new(),
        };
        todo.load().expect("Fail to load");
        todo
    }

    fn load(&mut self) -> Result<()> {
        if !self.file_path.is_file() {
            File::create(self.file_path).unwrap()
                 .write("[]".as_bytes()).unwrap();
        }
        let mut file = File::open(self.file_path).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let mut data = json::parse(&buffer).unwrap();

        for item in data.members_mut() {
            if let Some(val) = item.take_string() {
                self.items.push(Item { content: val });
            }
        }
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let mut file = File::create(self.file_path).unwrap();
        let mut data: JsonValue = JsonValue::Array(Vec::new());
        for item in &self.items {
            data.push(item.to_string()).unwrap();
        }
        
        let buffer = json::stringify_pretty(data, 4);
        file.write(buffer.as_bytes()).unwrap();
        Ok(())
    }

    pub fn add(&mut self, content: String) -> Result<()> {
        self.items.insert(0, Item { content: content });
        Ok(())
    }

    pub fn done(&mut self, id: usize) {
        self.items.remove(id - 1);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}


impl<'a> Drop for Todo<'a> {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}


impl<'a> fmt::Display for Todo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, item) in self.items.iter().enumerate() {
            try!(write!(f, " [{:2}] {} \n", count + 1, item));
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
