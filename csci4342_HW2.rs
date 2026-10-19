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

   let mut _values_with_ones_as_current_bit: Vec<String> = Vec::new();
   let mut _values_with_zeros_as_current_bit: Vec<String> = Vec::new();
   let mut potential_o2_rate_values: Vec<String> = diagnostics_report_values;
   
   let mut num_values = potential_o2_rate_values.len();
   let j = potential_o2_rate_values[0].len() as i32;
   //println!(j);

   //while num_values >= 2{
    for i in 0..potential_o2_rate_values[0].len() as i32{
      for value in potential_o2_rate_values{
        if &value[i as usize..(i+1) as usize] == "1"{
            _values_with_ones_as_current_bit.push(value.to_string());
        }
        else{
         _values_with_zeros_as_current_bit.push(value.to_string());
        } 


      }
      

      if _values_with_ones_as_current_bit.len() >= _values_with_zeros_as_current_bit.len(){
         potential_o2_rate_values = _values_with_ones_as_current_bit.clone();
      }
      else{
         potential_o2_rate_values = _values_with_zeros_as_current_bit.clone();
      }
      num_values = potential_o2_rate_values.len();
      _values_with_ones_as_current_bit.clear();
      _values_with_zeros_as_current_bit.clear();
   }
   dbg!(&potential_o2_rate_values);

}