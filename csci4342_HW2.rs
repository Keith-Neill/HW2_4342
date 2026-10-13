use std::env;
use std::fs;

fn main() {
   // get the file name from the command line
   let args: Vec<String> = env::args().collect();
   let file_name = &args[1];
   println!("{file_name}");

   load_daignostics(file_name);
}

fn load_daignostics(file_name: &String){
   let contents = fs::read_to_string(file_name).expect("file doesnt exist.");

   println!("{contents}");
}