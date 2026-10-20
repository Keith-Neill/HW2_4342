/* 
Student Name: Keith Neill
Program Description:
Date: 10/24/2026
*/

use std::env;
use std::fs;
use std::process;


fn main() {
   // get the file name from the command line
   let args: Vec<String> = env::args().collect();
   let file_name = &args[1];

   // get the diagnostics report values from the file
   let diagnostics_report_values: Vec<String> = load_daignostics(file_name);

   // get the O2 generator rating, the CO2 scrubber rating, and the life support rating
   let (o2_generator_rating, co2_scrubber_rating, life_support) = check_life_support(diagnostics_report_values);

   println!("o2 {}", o2_generator_rating);
   println!("co2 {}", co2_scrubber_rating);
   println!("life support {}", life_support);

}

// read the contents of the diagnostics report file into a vector of strings
fn load_daignostics(file_name: &String) -> Vec<String>{

   fs::read_to_string(file_name).expect("file doesnt exist.").lines().map(String::from).collect() 
   
}

// this function checks the life support rating
fn check_life_support(diagnostics_report_values: Vec<String>) -> (i32, i32, i32){

   let mut _values_with_ones_as_current_bit: Vec<String> = Vec::new();
   let mut _values_with_zeros_as_current_bit: Vec<String> = Vec::new();
   
   // make the potential ratings = to the origiobal diagnostics report, this is a starting point, these vectors will change over time 
   let mut potential_o2_generator_rating_values: Vec<String> = diagnostics_report_values.clone();
   let mut potential_co2_scrubber_rating_values: Vec<String> = diagnostics_report_values.clone();

   // length of the values
   let j = potential_o2_generator_rating_values[0].len() as i32;
   

   // figure out 02 generator rating
   for i in 0..j as i32{

      // sort the values with 0 and 1 as current bit into seperate vectors
      for value in potential_o2_generator_rating_values{
         if &value[i as usize..(i+1) as usize] != "1" || &value[i as usize..(i+1) as usize] != "0"{
            eprintln!("Error: One of the values in this file is not a binary number.");
            process::exit(1);
        }
        else if &value[i as usize..(i+1) as usize] == "1"{
            _values_with_ones_as_current_bit.push(value.to_string());
        }
        else{
         _values_with_zeros_as_current_bit.push(value.to_string());
        } 

      }

      // compare length of both vectors, potential o2 generator ratings are values of vectors with greater length, or the most common bit in the current position
      if _values_with_ones_as_current_bit.len() >= _values_with_zeros_as_current_bit.len(){
         potential_o2_generator_rating_values = _values_with_ones_as_current_bit.clone();
      }
      else{
         potential_o2_generator_rating_values = _values_with_zeros_as_current_bit.clone();
      }

      _values_with_ones_as_current_bit.clear();
      _values_with_zeros_as_current_bit.clear();

      // stop the loop early if there is one potential rating left before the loop reaches the last bit
      if potential_o2_generator_rating_values.len() == 1 && i < j{
         break
      }
   }

   // figure out co2 scrubber rating
   for i in 0..j as i32{
      
      // sort the values with 0 and 1 as current bit into seperate vectors
      for value in potential_co2_scrubber_rating_values{
         if &value[i as usize..(i+1) as usize] == "1"{
            _values_with_ones_as_current_bit.push(value.to_string());
         }
         else{
            _values_with_zeros_as_current_bit.push(value.to_string());
         } 
      }

      // add values with least common bits to potential co2 rate vector
      if _values_with_ones_as_current_bit.len() == 0{
         potential_co2_scrubber_rating_values = _values_with_zeros_as_current_bit.clone();
      }
      else if _values_with_zeros_as_current_bit.len() == 0{
         potential_co2_scrubber_rating_values = _values_with_ones_as_current_bit.clone();
      }
      else if _values_with_zeros_as_current_bit.len() <= _values_with_ones_as_current_bit.len(){
         potential_co2_scrubber_rating_values = _values_with_zeros_as_current_bit.clone();
      }
      else{
         potential_co2_scrubber_rating_values = _values_with_ones_as_current_bit.clone();
      }

      _values_with_ones_as_current_bit.clear();
      _values_with_zeros_as_current_bit.clear();

      // stop the loop early if there is one potential rating left before the loop reaches the last bit
      if potential_co2_scrubber_rating_values.len() == 1 && i < j{
         break
      }
   }

   // convert the binary strings into 32-bit integers
   let o2_generator_rating = i32::from_str_radix(&potential_o2_generator_rating_values[0], 2).unwrap();
   let co2_scrubber_rating = i32::from_str_radix(&potential_co2_scrubber_rating_values[0],2).unwrap();

   // calculate life support rating by multiplying the int values of the o2 generator rating and the co2 scrubber rating
   let life_support_rating = o2_generator_rating * co2_scrubber_rating;
   
   // put the integer values into a tuple and return to main()
   (o2_generator_rating, co2_scrubber_rating, life_support_rating)

}
