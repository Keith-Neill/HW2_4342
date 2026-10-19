use std::env;
use std::fs;


fn main() {
   // get the file name from the command line
   let args: Vec<String> = env::args().collect();
   let file_name = &args[1];

   let diagnostics_report_values: Vec<String> = load_daignostics(file_name);
   check_life_support(diagnostics_report_values);


}

// read the contents of the diagnostics report into a vector of strings
fn load_daignostics(file_name: &String) -> Vec<String>{

   fs::read_to_string(file_name).expect("file doesnt exist.").lines().map(String::from).collect() 
   
}

// this function checks the life support
fn check_life_support(diagnostics_report_values: Vec<String>){

   let mut _values_with_ones_as_current_bit: Vec<String> = Vec::new();
   let mut _values_with_zeros_as_current_bit: Vec<String> = Vec::new();
   
   // make the potential ratings = to the origiobal diagnostics report, this is a starting point, these vectors will change over time 
   let mut potential_o2_rate_values: Vec<String> = diagnostics_report_values.clone();
   let mut potential_co2_rate_values: Vec<String> = diagnostics_report_values.clone();

   // length of the values
   let j = potential_o2_rate_values[0].len() as i32;
   

   // figure out 02 rating
   for i in 0..j as i32{

      // add to potential o2 rating vector
      for value in potential_o2_rate_values{
        if &value[i as usize..(i+1) as usize] == "1"{
            _values_with_ones_as_current_bit.push(value.to_string());
        }
        else{
         _values_with_zeros_as_current_bit.push(value.to_string());
        } 

      }

      // compare length of both vectors, potential o2 ratings are values of vectors with greater length, or the most common bit in the current position
      if _values_with_ones_as_current_bit.len() >= _values_with_zeros_as_current_bit.len(){
         potential_o2_rate_values = _values_with_ones_as_current_bit.clone();
      }
      else{
         potential_o2_rate_values = _values_with_zeros_as_current_bit.clone();
      }

      _values_with_ones_as_current_bit.clear();
      _values_with_zeros_as_current_bit.clear();

      // stop the loop early if there is one potential rating left before the loop reaches the last bit
      if potential_o2_rate_values.len() == 1 && i < j{
         break
      }
   }

   // figure out co2 rating
   for i in 0..j as i32{
      
      // add to potential co2 rating values
      for value in potential_co2_rate_values{
         if &value[i as usize..(i+1) as usize] == "1"{
            _values_with_ones_as_current_bit.push(value.to_string());
         }
         else{
            _values_with_zeros_as_current_bit.push(value.to_string());
         } 
      }

      // add values with least common bits to potential co2 rate vector
      if (_values_with_zeros_as_current_bit.len() <= _values_with_ones_as_current_bit.len()) || (_values_with_ones_as_current_bit.len() == 0){
         potential_co2_rate_values = _values_with_zeros_as_current_bit.clone();
      }
      else{
         potential_co2_rate_values = _values_with_ones_as_current_bit.clone();
      }

      _values_with_ones_as_current_bit.clear();
      _values_with_zeros_as_current_bit.clear();

      // stop the loop early if there is one potential rating left before the loop reaches the last bit
      if potential_co2_rate_values.len() == 1 && i < j{
         break
      }
   }
   let o2_rating: &String = &potential_o2_rate_values[0];
   println!("{o2_rating}");

   let co2_rating: &String = &potential_co2_rate_values[0];
   println!("{co2_rating}");
   //let co2_rating: &String = &potential_co2_rate_values[0];
   

}

/*fn calculate_co2_rating(potential_co2_rate_values: Vec<String>, i: i32) -> Vec<String>{
   let _values_with_ones_as_current_bit: Vec<String> = Vec::new();
   let _values_with_zeros_as_current_bit: Vec<String> = Vec::new();

   for value in potential_co2_rate_values{
      if &value[i as usize..(i+1) as usize] == "1"{
         _values_with_ones_as_current_bit.push(value.to_string());
      }
      else{
         _values_with_zeros_as_current_bit.push(value.to_string());
      } 
   }

   if _values_with_zeros_as_current_bit.len() <= _values_with_ones_as_current_bit.len(){
      potential_co2_rate_values = _values_with_zeros_as_current_bit.clone();
   }
   else{
      potential_co2_rate_values = _values_with_ones_as_current_bit.clone();
   }

   potential_co2_rate_values

}*/