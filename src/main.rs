#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize,Serialize};
use serde_json::{Result,Value, Map};

#[derive(Serialize, Deserialize, Debug)]
struct Container {
    name: String,
    age: i16
}

#[derive(Debug)]
pub enum JsonValue {
    IntValue(usize),
    StringValue(String),
    IntArray(Vec<usize>),
    StringArray(Vec<String>),
    Object(JsonObject),
    ObjectArr(Vec<JsonObject>),
}

#[derive(Debug)]
pub struct JsonObject {
   entries: Vec<JsonObjectEntry>
}

#[derive(Debug)]
pub struct JsonObjectEntry {
    key: String,
    value: Box<JsonValue>,
}

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        return Stack {
            stack: Vec::new()
        };
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        return self.stack.pop()
    }

    fn peek(&mut self) -> Option<&T> {
        return self.stack.last()
    }

}

fn main() {
    // parse_typed_json(&json_string);
    // parse_raw_json(&json_string);
    // find_fields(&json_string);

    // let json = JsonObject {
    //     key: String::from("some key"),
    //     value: Box::new(JsonValue::IntArray(vec![1,2,3,4,5]))
    // };

    // Example json string
    // {
    //     "name": "Apu",
    //     "age": 39,
    //     "address": {
    //       "location": "Dhaka",
    //       "phone": ["2", "3"]
    //     },
    //     "groups": [{"name": "g1"}, {"name": "g2", "role": ["r1", "r2"]}]
    // }

    // let json_string = String::from(
    //     "{\"name\": \"Apu\", \
    //       \"age\": 39, \
    //       \"address\": { \
    //       \"location\": \"Dhaka City\", \
    //       \"phone\": [\"2\", \"3\", \"4\": {\"a\": \"b\", \"c\": 99}]\
    //       },\
    //       \"groups\":  [{\"name\": \"g1 has no shame\"}, {\"name\": \"g2\", \"role\": [\"r1\", \"r2\"]}]\
    //       }");

    let json_string = String::from(
        "{\"name\": \"Apu\", \
          \"age\": [2,3,4,5], \
          }");
    let tokens = tokenize(&json_string);
    deserialize(tokens);
}

fn deserialize(tokens: Vec<String>) {
    let mut key_stack:Stack<String> = Stack::new();
    let mut object_stack: Stack<JsonObject> = Stack::new();
    let mut root:Option<JsonObject> = None;

    for token in tokens {
        println!("{:?}", token);
        let tr:&str = &token;
        match tr {
            "{" => {
                let json_object = JsonObject {
                    entries: Vec::new()
                };
                object_stack.push(json_object);
            },
            "}" => {
                let obj = object_stack.pop();
                let key = key_stack.pop();
                match obj {
                    Some(o) => {
                        match key {
                            Some(k) => {
                                match object_stack.pop() {
                                    Some(mut parent) => {
                                        parent.entries.push(JsonObjectEntry {
                                            key: k,
                                            value: Box::new(JsonValue::Object(o)),
                                        });
                                        object_stack.stack.push(parent);
                                    }
                                    None => {}
                                }
                            },
                            None => {
                                match object_stack.peek() {
                                    Some(x) => (),
                                    None => {
                                        root = Some(o);
                                    }
                                }
                            }
                        }

                    }
                    None => {}
                }
            },
            "[" => {
                let json_arr = JsonValue::ObjectArr(Vec::new());
                object_stack.push(json_arr);
            },
            "]" => {

            },
            other_values => {
                if key_stack.stack.len() < object_stack.stack.len() {
                    key_stack.push(tr.to_string());
                    // println!(" >> {:?}", key_stack);
                } else {
                    match key_stack.peek() {
                        Some(value) => {
                            let key = key_stack.pop().unwrap();
                            let obj = object_stack.pop();
                            match obj {
                                Some(mut o) => {
                                    let json_val = JsonValue::StringValue(other_values.to_string());
                                    o.entries.push(JsonObjectEntry {
                                        key,
                                        value: Box::new(json_val),
                                    });
                                    object_stack.stack.push(o);
                                },
                                None => ()
                            }
                        },
                        None => {
                            key_stack.push(tr.to_string())
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", root)

}

fn tokenize(json_string: &String) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut string_val = String::new();

    for ch in json_string.chars() {
        match ch {
            '{' | '['  => {
                stack.push(ch.to_string());
            },
            ']' | '}' => {
                if string_val.len() > 0 {
                    stack.push(string_val.clone());
                    string_val.clear();
                }
                stack.push(ch.to_string());
            }
            ':' => {
                stack.push(string_val.clone());
                string_val.clear();
            },
            ',' => {
                if string_val.len() > 0 {
                    stack.push(string_val.clone());
                    string_val.clear();
                }
            },
            ' ' => {
                if string_val.len() > 0 {
                    string_val.push(ch);
                }
            },
            _ => {
                string_val.push(ch);
            }
        }
    }

    return stack;
}

fn parse_raw_json(json_string: &String) {
    let json_result: Result<Value> = serde_json::from_str(json_string);
    let container_json = match json_result {
        Ok(j) => j,
        Err(error) => panic!("Problem parsing json {:?}", error)
    };
    println!("Raw json: name: {} and age: {}", container_json["name"], container_json["age"]);
}

fn parse_typed_json(json_string: &String) {
    let json_result: Result<Container> = serde_json::from_str(json_string);

    let container_json = match json_result {
        Ok(j) => j,
        Err(error) => panic!("Problem parsing json {:?}", error)
    };

    println!("name: {} and age: {}", container_json.name, container_json.age);
}

fn find_fields(json_string: &String) {
    let json_result: Result<Value> = serde_json::from_str(json_string);
    let container_json = match json_result {
        Ok(j) => j,
        Err(error) => panic!("Problem parsing json {:?}", error)
    };

    print_json(container_json.as_object().unwrap(), String::from(""));
}

fn print_json(obj: &Map<String, Value>, indent: String) {
    for (key, val) in obj.iter() {
        if val.is_object() {
            println!("{}key: {}", indent, key);
            let mut new_indent = String::new();
            new_indent.push_str(" ");
            print_json(val.as_object().unwrap(), new_indent);
        }
        else {
            println!("{}key: {} val: {}", indent, key, val)
        }
    }
}

