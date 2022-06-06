use cached::proc_macro::cached;

/// fib
/// ---
/// fib(0) = 0
/// fib(1) = 1
/// Note that it should be n > 1 in the following!
/// fib(n) = fib(n-1) + fib(n-2) for all n >= 0
pub fn fib(n: usize) -> usize {
	
	return fib_rec(n);
}

#[cached]
fn fib_rec(n: usize) -> usize {
	
	if n < 2 {
		return n
	} else {
		return fib_rec(n - 1) + fib_rec(n - 2)
	}
}

#[cfg(test)]
mod tests {
	
	use crate::fib;
	
	#[test]
	fn fib_test() {
		
		// Test execution takes ~3.21s without memoization and ~0.00s with memoization (on my computer)
		let input  = [ 0, 1, 2, 3, 4, 5, 6,  7,  8,  9, 10, 11,  12,    23,        42 ];
		let output = [ 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 28657, 267914296 ];
		
		for i in 0..input.len() {
			
			assert_eq!(fib(input[i]), output[i], "fib({}) = {}", input[i], output[i])
		}
	}
}
