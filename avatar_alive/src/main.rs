mod game_start;
use std::path::Path;
use std::io::{self, Write};

fn main() {
  let mut start = 1u32; //must declare mutable for no errors in the following line
  let start_game = &mut start;
  let mut command = String::new();

  while (command.trim() != "y") && (command.trim() != "n") {
    print!("\n\nStart new game?  y/n >>> ");
    let _suppress_warning = io::stdout().flush();
    command = String::from("");
    io::stdin().read_line(&mut command)
        .expect("failed to read line");
    //println!("user input: {}", command.trim());
  }

  let refridge_path = Path::new("refridge.txt");
  let refridge_exists = refridge_path.exists();

  if (command.trim() == "y") || !refridge_exists {
    game_start::refridgerator::create_refridge(refridge_path);
    //println!("refridge: {:?}", refridge_vec);
    //println!("refridge: {}", refridge_vec[0]);
  }

  loop {
    match start{
    1 => game_start::first_start(start_game),
    2 => {game_start::begin_game(); break},
    _ => break,
    }
  }
}
