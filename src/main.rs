use std::env;
use std::io::Write;
use std::io::Read;

use xorhelper::ArgResult::FromPath;
use xorhelper::ArgResult::FromString;
use xorhelper::ArgResult::Failed;

fn usage(path : &str) {
	eprintln!("Usage:\n\t$ echo 'hello, world' | {} 'my super secret password' > obfuscated.txt\n", path);
	eprintln!("The data to be translated is read from STDIN.");
	eprintln!("The translated data is returned to STDOUT.");
}

fn main() {
	// Retrieve commandline arguments.
	let args : Vec<String> = env::args().collect();
	
	if args.len() < 2 { 
		usage(&args[0]); 
		return;
	}
	
	// Read the plaintext from STDIN.
	let stdin = std::io::stdin();
	let mut plain : Vec<u8> = Vec::new();
	
	for byte in stdin.bytes() {
		plain.push(byte.unwrap());
	}
	eprintln!("Successfully read {} bytes from STDIN", plain.len());
	
	// Use commandline argument to read the keyfile or interpret the key string.
	let arg = &args[1];
	let key : Vec<u8> = match xorhelper::parse_argument(arg) {
		FromPath(vector) => {
			eprintln!("Successfully read a {}-byte key from '{}'", vector.len(), arg);
			vector
		},
		FromString(vector) => {
			eprintln!("Using the following string as a key: '{}'", arg);
			vector
		},
		Failed(error) => {
			eprintln!("Failed with error: {}", error);
			return;
		}
	};
	
	// Translate using the provided key and dump to STDOUT.
	let result = xorhelper::xor_translate(&plain, &key);
	match result {
		Ok(value) => {
			let mut stdout = std::io::stdout();
			let _ = stdout.write(&value);
			let _ = stdout.flush();
		},
		Err(message) => {
			eprintln!("Could not perform translation: {}", message);
		}
	}	
}
