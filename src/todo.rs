use super::json::{self, JsonValue};
use super::json::object::Object;

use std::path::PathBuf;
use std::fs::File;
use std::fmt;
use std::ops::Drop;
use std::io::{Read, Write};
use std::process::Command;
use std::env;

const TOKEN_ENV_KEY: &'static str = "GITHUB_ACCESS_TOKEN";

pub struct Todo {
    file_path: PathBuf,
    gist_id: Option<String>,
    items: Vec<Item>,
}

#[derive(PartialEq, Eq)]
struct Item {
    content: String,
}

impl Todo {
    pub fn new(path: &str) -> Todo {
        let mut todo = Todo {
            file_path: PathBuf::from(path),
            gist_id: None,
            items: Vec::new(),
        };
        todo.load();
        todo
    }

    fn load(&mut self) {
        if !self.file_path.is_file() { self.init_file(); }
        let mut file = File::open(&self.file_path).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let mut data = json::parse(&buffer).unwrap();
        self.gist_id = data["gist_id"].take_string();
        for item in data["todolist"].take().members_mut() {
            if let Some(val) = item.take_string() {
                self.items.push(Item { content: val });
            }
        }
    }

    fn save(&self) {
        let mut file = File::create(&self.file_path).unwrap();

        let mut data = Object::new();
        let todolist = JsonValue::Array(self.items.iter()
                                            .map(|x| x.to_string())
                                            .map(JsonValue::String)
                                            .collect());
        let gist_id = if self.gist_id.is_some() {
            JsonValue::String(self.gist_id.clone().unwrap())
        } else {
            JsonValue::Null
        };
        data.insert("todolist", todolist);
        data.insert("gist_id", gist_id);
        let data = JsonValue::Object(data);
        
        let buffer = json::stringify_pretty(data, 4);
        file.write(buffer.as_bytes()).unwrap();
    }

    fn init_file(&self) {
        self.save();
    }

    pub fn add(&mut self, content: &str) {
        self.items.insert(0, Item { content: content.to_string() });
    }

    pub fn done(&mut self, id: usize) {
        self.items.remove(id - 1);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Download from gist by gist_id and merge it with local items, and then
    /// upload the merged list to gist if have authorization. (if gist_id is
    /// null, create a new gist if have personal access tokens of github, or
    /// do nothing.)
    pub fn sync(&mut self) {
        println!("Sync...");

        if self.gist_id.is_none() {
            self.gist_id = match self.create_gist() {
                Some(val) => Some(val),
                None => {
                    println!("failed to create gist id");
                    return;
                },
            }
        }

        let content = match download_gist(&self.gist_id.clone().unwrap()) {
            Some(val) => val,
            None => {
                println!("can not download the gist");
                return;
            }
        };

        let mut content = json::parse(&content).unwrap();

        if let Some(val) = content["gist_id"].take_string() {
            self.gist_id = Some(val);
        }

        // merge
        let mut new_items = Vec::new();
        for item in content["todolist"].take().members_mut().rev() {
            if let Some(val) = item.take_string() {
                let item = Item { content: val };
                if self.items.iter().all(|ref x| **x != item) {
                    new_items.push(item);
                }
            }
        }
        new_items.append(&mut self.items);
        self.items = new_items;

        // save merged list
        self.save();

        // upload it
        self.upload();

        println!("Done!");
    }

    pub fn upload(&self) {
        let gist_id = self.gist_id.clone().unwrap();
        let user = match get_user_info() {
            Some(val) => val,
            None => {
                println!("cannot get github user info");
                return;
            },
        };
        let mut content = String::new();
        File::open(&self.file_path).unwrap()
             .read_to_string(&mut content).unwrap();

        match upload_gist(&gist_id, &user, &content) {
            Some(_) => println!("Success to upload gist"),
            None => println!("Failed to upload gist"),
        };
    }

    /// create a gist and return its id
    fn create_gist(&self) -> Option<String> {
        println!("Creating a gist...");
        
        let data = object!{
            "description" => "auto-generated gist by todo app",
            "public" => false,
            "files" => object!{
                ".todo" => object!{
                    "content" => "{\"todolist\":[], \"gist_id\":null}"
                }
            }
        };
        let data = data.dump();

        let user = match get_user_info() {
            Some(val) => val,
            None => {
                println!("cannot get github user info");
                return None;
            },
        };

        let result = Command::new("curl")
                          .arg("--user").arg(user)
                          .arg("--data").arg(&data)
                          .arg("https://api.github.com/gists")
                          .output()
                          .expect("failed to run process `curl`");
    
        if !result.status.success() {
            println!("`curl` exit with non-zero status.");
            return None;
        }

        let result = String::from_utf8(result.stdout).unwrap();
        let mut response = json::parse(&result).unwrap();
        response["id"].take_string()
    }
}

fn get_access_token() -> Option<String> {
    match env::var(TOKEN_ENV_KEY) {
        Ok(val) => Some(val),
        Err(_) => {
            println!("please set environment variable `{}`.", TOKEN_ENV_KEY);
            None
        }
    }
}

fn get_github_username() -> Option<String> {
    let result = Command::new("git").arg("config").arg("user.name")
                                    .output().unwrap();
    if result.status.success() {
        Some(String::from_utf8(result.stdout).unwrap())
    } else {
        println!("please set git username by `git config \
                  --global user.name <yourname>`)");
        None
    }
}

fn get_user_info() -> Option<String> {
    let username = match get_github_username() {
        Some(val) => val,
        None => {
            println!("cannot get github username");
            return None;                
        }
    };

    let token = match get_access_token() {
        Some(val) => val,
        None => {
            println!("cannot get personal access token");
            return None;
        }
    };

    Some(username + ":" + &token)
}

/// Download the gist by gist id and parse the response data by json, and then
/// return the file content. 
fn download_gist(id: &str) -> Option<String> {
    println!("Waitting for gist download...");

    let res = Command::new("curl")
                      .arg(&format!("https://api.github.com/gists/{}", id))
                      .output()
                      .expect("failed to run process `curl`");
    
    if !res.status.success() {
        println!("`curl` exit with non-zero status.");
        return None;
    }

    let data = String::from_utf8(res.stdout).unwrap();
    let data = json::parse(&data).unwrap();
    let content = match data["files"][".todo"]["content"].as_str() {
        Some(val) => val,
        None => {
            println!("cannot find file `.todo` in `files` field in \
                      response json data. \n(maybe the gist id is invalid, just \
                      set it to `null` in storage file and sync again, that is \
                      `\"gist_id\": null`.)");
            return None;
        }
    };
    Some(content.to_string())
}

fn upload_gist(id: &str, user: &str, content: &str) -> Option<()> {
    println!("Waitting for gist upload...");

    let data = object!{
        "files" => object!{
            ".todo" => object!{
                "content" => content
            }
        }
    };

    let res = Command::new("curl")
                      .arg("--request").arg("PATCH")
                      .arg("--user").arg(user)
                      .arg("--data").arg(&data.dump())
                      .arg(&format!("https://api.github.com/gists/{}", id))
                      .output()
                      .expect("failed to run process `curl`");

    if res.status.success() {
        Some(())
    } else {
        println!("`curl` exit with non-zero status.");
        None
    }

}


impl Drop for Todo {
    fn drop(&mut self) {
        self.save();
    }
}


impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, item) in self.items.iter().enumerate() {
            write!(f, " [{:2}] {} \n", count + 1, item)?;
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
