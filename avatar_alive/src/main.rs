mod game_start;
use std::fs::File;
use std::path::Path;


fn main() {
  let mut start = 1u32; //must declare mutable for no errors in the following line
  let start_game = &mut start;

  /*let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

  // Open a file in write-only mode, returns `io::Result<File>`
  let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}",
                         display,
                         why.description()),
      Ok(file) => file,
  }; */

  let refridge_path = Path::new("refridge.txt");
  let refridge_exists = refridge_path.exists();
  //let display = refridge_path.display();

  if refridge_exists {
    let mut _refridge_file = File::open(&refridge_path);
  }
  else {
    let mut _refridge_file = File::create(&refridge_path);
  }

  /*let mut refridge_file = match File::create(&refridge_path) {
    Err(why) => panic!("couldn't create {}: {}",
                       display,
                       why.description()),
    Ok(file) => file,
  };*/


  /*match OpenOptions::new().read(true).write(true).create(true).open("refridge_food.txt") {
          Ok(ref mut refridge_food_file) => {
            writeln!(refridge_food_file, "Hello!").unwrap();
          }
  }; */


  //println!("refridge_food_file created: {:?}", refridge_food_file);

  loop {
    match start{
    1 => game_start::first_start(start_game),
    2 => {game_start::begin_game(); break},
    _ => break,
    }
  }
}
