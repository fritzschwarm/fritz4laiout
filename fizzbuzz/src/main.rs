/// fizzbuzz
/// ---
/// returns “Fizz” if n is divisible by 3
/// returns “Buzz” if n is divisible by 5
/// returns “FizzBuzz” if n is divisible by both 3 and 5
/// returns an empty string in any other case
pub fn fizzbuzz(n: usize) -> String {
	
	return format!("{}{}", if n%3==0 {"Fizz"} else {""}, if n%5==0 {"Buzz"} else {""});
}

pub fn fizzbuzz_alt(n: usize) -> String {
	
	//! Note: The order of match arms matters
	match n {
		// Or just: n if n % 15 == 0
		n if (n % 3 == 0) && (n % 5 == 0) => { return "FizzBuzz".to_string(); }
		
		n if  n % 3 == 0                  => { return "Fizz".to_string(); }
		
		n if  n % 5 == 0                  => { return "Buzz".to_string(); }
		
		_ => { return "".to_string(); }
	}
}

// Test the function by adding a mod tests module.
#[cfg(test)]
mod tests {
	
	use crate::fizzbuzz;
	
	// Theoretically all possible input values could be checked by setting N_MAX to usize::MAX
	static N_MAX: usize = 100000;
	
	#[test]
	fn divisibile_by_3() {
		
		for n in (3..N_MAX+1).step_by(3) {
			
			if n % 5 != 0 {
				
				assert_eq!(fizzbuzz(n), "Fizz".to_string(), "{} is divisible by {}", n, 3);
			}
		}
	}
	
	#[test]
	fn divisible_by_5() {
		
		for n in (5..N_MAX+1).step_by(5) {
		
			if n % 3 != 0 {
			
				assert_eq!(fizzbuzz(n), "Buzz".to_string(), "{} is divisible by {}", n, 5);
			}
		}
	}
	
	#[test]
	fn divisible_by_3_and_5() {
		
		for n in (15..N_MAX+1).step_by(15) {
			
			assert_eq!(fizzbuzz(n), "FizzBuzz".to_string(), "{} is divisible by {} and {}", n, 3, 5);
		}
	}
	
	#[test]
	fn divisible_by_neither_3_nor_5() {
		
		for n in 1..N_MAX+1 {
			
			if (n % 3 != 0) && (n % 5 != 0) {
				
				assert_eq!(fizzbuzz(n), "".to_string(), "{} is devisible neither by {} nor by {}", n, 3, 5);
			}
		}
	}
}

// Sample application calling the function with various input values
fn main() {
	
	for n in 1..100 {
		
		println!("{} -> {}", n, fizzbuzz(n));
	}
}
