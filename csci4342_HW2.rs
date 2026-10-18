use std::env;
use std::fs;

fn main() {
   // get the file name from the command line
   let args: Vec<String> = env::args().collect();
   let file_name = &args[1];
   println!("{file_name}");

   let diagnostics_report_values: Vec<String> = load_daignostics(file_name);
   check_life_support(diagnostics_report_values);

   let s: &str = "Hello";
   println!("{s}");
   let nth_char = &s[0..2];
   println!("{nth_char}");

}

// read the contents of the diagnostics report into a vector of strings
fn load_daignostics(file_name: &String) -> Vec<String>{

   fs::read_to_string(file_name).expect("file doesnt exist.").lines().map(String::from).collect() 
   
}

// this function checks the life support
fn check_life_support(diagnostics_report_values: Vec<String>){

   dbg!(&diagnostics_report_values);
   let num = &diagnostics_report_values[0];
   println!("{num}");

   let mut _values_with_ones_as_current_bit: Vec<String> = Vec::new();
   let mut _values_with_zeros_as_current_bit: Vec<String> = Vec::new();
   //for i in 0..&diagnostics_report_values[0].len(){
      for value in diagnostics_report_values{
        if &value[0..(0+1)] == "1"{
            _values_with_ones_as_current_bit.push(value);
        } 


      }
      dbg!(&_values_with_ones_as_current_bit);
      _values_with_ones_as_current_bit.clear();
      dbg!(&_values_with_ones_as_current_bit);
   //}

}