use std::env;
use std::fs;

fn main() {
   // get the file name from the command line
   let args: Vec<String> = env::args().collect();
   let file_name = &args[1];
   println!("{file_name}");

   let contents: Vec<String> = load_daignostics(file_name);
   dbg!(&contents);
   let num = &contents[0];
   println!("{num}");
}

// read the contents of the diagnostics report into a vector of strings
fn load_daignostics(file_name: &String) -> Vec<String>{

   fs::read_to_string(file_name).expect("file doesnt exist.").lines().map(String::from).collect() 
  
   
}