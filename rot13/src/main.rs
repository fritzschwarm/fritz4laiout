// Note that BufWriter is just used in this example because it was included in the problem description.
// In this case there is no advantage in using it over directly writing to the buffer (since the buffer
// is in memory anyway.

use std::io::BufWriter;
use std::io::Write;

struct Rot13Writer<T>
where
	T: Write,
{
//	stream: T
	stream: BufWriter<T>
}

impl<T> Rot13Writer<T>
where
	T: Write,
{
	pub fn new(inner: T) -> Self {
		
		Self {
//			stream: inner
			stream: BufWriter::new(inner)
		}
	}
}

impl<T> Write for Rot13Writer<T>
where
	T: Write,
{
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		
		let mut written = 0;
		let mut cur: u8;
		
		for i in 0..buf.len() {
			
			cur = buf[i];
			
			// Only alphabetic letters are processed
			if (cur >= b'a' && cur <= b'z') || (cur >= b'A' && cur <= b'Z') {
			
				cur += 13;
				
				if (buf[i] <= b'z' && cur > b'z') || 
				   (buf[i] <= b'Z' && cur > b'Z')  {
					
					cur -= 26;
				}
			}
			
			written += self.stream.write(&[cur]).unwrap();
		}
			
		Ok(written)
	}
	
	fn flush(&mut self) -> std::io::Result<()> {
		
		// This is required only if BufWriter is used
		self.stream.flush().unwrap();
		
		Ok(())
	}
}

fn main() {
	
	let mut content = Vec::<u8>::default();
	
	// This additional closure is need when using BufWriter instead of directly writing to the output.
	{
		let mut buff = Rot13Writer::new(&mut content);
		
		buff.write(b"Lbh penpxrq zl fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst")
		    .unwrap();
		
		if !buff.flush().is_ok() {
			
			println!("Failed to flush buffer");
		}
	}
	
	println!(
		"result: {:?}",
		content.iter().map(|x| *x as char).collect::<String>()
	);
}

#[cfg(test)]
mod tests {
	
	use std::io::Write;
	use crate::Rot13Writer;
	
	#[test]
	fn test_rot13() {
		
		let input  = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ .";
		let output = b"nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM .";
		
		let mut content = Vec::<u8>::default();
		
		// This additional closure is need when using BufWriter instead of directly writing to the output.
		{
			let mut buff = Rot13Writer::new(&mut content);
			
			buff.write(input)
			    .unwrap();
		}
		
		for i in 0..output.len() {
			
			assert_eq!(content[i], output[i], "rot13({:?}) = {:?}", input[i], output[i]);
		}
	}
}
