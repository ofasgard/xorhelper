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

pub fn xor_translate(plaintext : &[u8], key : &[u8]) -> Result<Vec<u8>,String> {
	let mut output : Vec<u8> = Vec::new();
	
	if plaintext.len() == 0 { return Err("plaintext has a length of zero!".to_string()); }
	if key.len() == 0 { return Err("key has a length of zero!".to_string()); }
	
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
	Ok(output)
}

/// A convenience function that wraps [xor_translate()] to handle `&str` parameters.

pub fn xor_translate_str(plaintext : &str, key : &str) -> Result<Vec<u8>,String> {
	let plaintext_bytes : Vec<u8> = plaintext.to_string().into_bytes();
	let key_bytes: Vec<u8> = key.to_string().into_bytes();
	let result : Vec<u8> = xor_translate(&plaintext_bytes, &key_bytes)?;
	Ok(result)
}

