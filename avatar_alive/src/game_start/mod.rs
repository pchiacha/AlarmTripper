pub mod command_check;
pub mod refridgerator;
use std::io::{self, Write};

pub fn first_start(start: &mut u32) {
  println!("\n\n  Hello! You just volunteered to take care of a virtual avatar. \n
  The how part is for you to find out yourself. :D \n\n");
  press_enter();

  println! ("\n  Here are some basic commands that you can input: \n
  \teat, \n
  \twork, \n
  \tsleep, \n
  \tplay \n
  It is possible for the commands to be more specific as you continue \n
  to take care of the avatar. For example, \n
  \teat the banana

  \n\n  Good luck trying to figuring out this program!\n");
  press_enter();

  *start = 2;
} //first_start

pub fn begin_game() {
  let choice = options();
  println!("{}", choice);
} //begin_game

fn options() -> &'static str {
  println!("\n\n\tAvatar is sitting on the floor...
  \n\tInput a command or choose a number from below
   \t\t1: go to grocery store
   \t\t2: go to pet store\n\n");

   print!("\n >>> ");
   let _suppress_warning = io::stdout().flush();

   let mut command_is = String::new();
   let command = &mut command_is;
   io::stdin().read_line(command)
       .expect("failed to read line");

   command_check::find_command(command);

   "success"
} //options

fn press_enter() {
    print!(" >>> ");
    let _suppress_warning = io::stdout().flush();

    let mut command = String::new();

    io::stdin().read_line(&mut command)
        .expect("failed to read line");

    println!("------------------")
} //press_enter
