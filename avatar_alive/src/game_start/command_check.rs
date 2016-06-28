use std::fs::File;

pub fn find_command(command: &mut str) {
  if command.trim().find("eat") == Some(0) {
    println!("looks in refrigerator..."); eat();
  }
}

fn eat() {
  //let mut f = try! (File::create("refrigerator.txt"));
}
