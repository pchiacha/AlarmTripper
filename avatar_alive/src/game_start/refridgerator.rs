use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn create_refridge(refridge_path: &Path) /*-> Vec<&str>*/{
  let mut _refridge_file = File::create(&refridge_path).unwrap();
  //println!("refridge_file created here: {:?}", _refridge_file)
  let text = format!("egg (x1)\r\n");
  let _x = _refridge_file.write_all(text.as_bytes());
  let text = format!("pancake (x1)\r\n");
  let _x = _refridge_file.write_all(text.as_bytes());

  /*let mut refridge_vec = Vec::new();
  refridge_vec.push("egg\n");
  refridge_vec.push("pancake");
  refridge_vec*/
}
