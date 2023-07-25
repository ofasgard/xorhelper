use std::fs;
use std::io::Error;
use std::io::ErrorKind;

/// A wrapper around `Result<T,E>` that contains metadata about whether the argument was parsed as a file or a string.

pub enum ArgResult {
	FromPath(Vec<u8>),
	FromString(Vec<u8>),
	Failed(Error)
}

/// Parse an argument as either a filepath or, failing that, as a raw string. Returns a `Vec<u8>` of byte values.

pub fn parse_argument(arg : &str) -> ArgResult {
	// Try to read it as a file path.
	let result = fs::read(arg);
	
	// Match block to check if the file read was successful.
	// If not, we either treat argument as a string or we propagate an error.
	let output = match result {
		Ok(vector) => ArgResult::FromPath(vector),
		Err(e) => match e.kind() {
			ErrorKind::NotFound => {
				let s = String::from(arg);
				let b = s.into_bytes();
				ArgResult::FromString(b)
			},
			_ => ArgResult::Failed(e)
		}
	};
	
	output
}

/// Translate a vector of byte values by XORing with a repeating key.

pub fn xor_translate(plaintext : &Vec<u8>, key : &Vec<u8>) -> Vec<u8> {
	let mut output : Vec<u8> = Vec::new();
	if (plaintext.len() == 0) | (key.len() == 0) { return output; }
	
	let mut key_index = 0;
	for text_byte in plaintext {
		let key_byte : &u8 = match key.get(key_index) {
			Some(value) => value,
			None => {
				key_index = 0;
				key.get(key_index).unwrap()
			}
		};
		output.push(text_byte ^ key_byte);
		key_index += 1;
	}
	output
}

