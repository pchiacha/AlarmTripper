use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

pub fn find_command(command: &mut str) {
  if command.trim().find("eat") == Some(0) {
    println!("\n\n\tlooks in refrigerator...");
    let refridge_food: HashMap<String, String> = get_refridge_food();
    if command.trim().contains(" ") {
      let command_val: Vec<&str> = command.split(" ").collect();
      let command_len = command_val.len();
      println!("command_len: {}", command_len);
    }
    else {

    }
  }
}

fn get_refridge_food() -> HashMap<String, String>{
  let refridge_path = Path::new("refridge.txt");
  let mut refridge_file = File::open(&refridge_path).unwrap();
  let mut refridge_buff = String::new();
  refridge_file.read_to_string(&mut refridge_buff);
  println!("refridge_buff: {:?}", refridge_buff);

  let mut refridge_food: Vec<&str> = refridge_buff.split("\r\n").collect();
  println!("refridge_food: {:?}", refridge_food);
  refridge_food = refridge_food[0].split(" (x").collect();
  println!("refridge_food: {:?}", refridge_food);
  let mut refridge_map = HashMap::new();
  let mut index = 0;
  let total_food = refridge_food.len();
  /*for i in 1...total_food+1 {
    index_count = refridge_food[i].find("(") + 1;
    index_name = refridge_food[i].find("(") - 1;

  }*/
  refridge_map.insert(refridge_food[0].to_string(), refridge_food[1].to_string());
  refridge_map
}
