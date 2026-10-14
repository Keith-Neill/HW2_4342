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
   // AI was used to help me figure out the syntax of how to put the file contents in a vector of string
   let contents: Vec<String> = fs::read_to_string(file_name).expect("file doesnt exist.").lines().map(String::from).collect(); 
   //.lines().collect();
   //.expect("file doesnt exist.")

   dbg!(&contents);
   let num = &contents[0];
   println!("{num}");
   
}