use std::env;
use std::io::Write;
use std::io::Read;
use std::ffi::OsString;

use xorhelper::ArgResult::FromPath;
use xorhelper::ArgResult::FromString;
use xorhelper::ArgResult::Failed;

fn usage(path : &OsString) {
	let path_str = path.to_str().unwrap();
	eprintln!("Usage:\n\t$ echo 'hello, world' | {} 'my super secret password' > obfuscated.txt\n", path_str);
	eprintln!("The data to be translated is read from STDIN.");
	eprintln!("The translated data is returned to STDOUT.");
}

fn main() {
	// Retrieve commandline arguments.
	let args : Vec<OsString> = env::args_os().collect();
	
	if args.len() < 2 { 
		usage(&args[0]); 
		return;
	}
	
	let key_arg = match args[1].to_str() {
		Some(s) => s,
		None => {
			eprintln!("Could not parse {:?} as a valid UTF-8 string!", &args[1]);
			return;
		}
	};
	
	// Read the plaintext from STDIN.
	let stdin = std::io::stdin();
	let mut plain : Vec<u8> = Vec::new();
	
	for byte in stdin.bytes() {
		plain.push(byte.unwrap());
	}
	eprintln!("Successfully read {} bytes from STDIN", plain.len());
	
	// Use commandline argument to read the keyfile or interpret the key string.
	let key : Vec<u8> = match xorhelper::parse_argument(key_arg) {
		FromPath(vector) => {
			eprintln!("Successfully read a {}-byte key from '{}'", vector.len(), key_arg);
			vector
		},
		FromString(vector) => {
			eprintln!("Using the following string as a key: '{}'", key_arg);
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
			eprintln!("Successfully translated {} bytes", value.len());
			let mut stdout = std::io::stdout();
			stdout.write_all(&value).expect("Could not write output to STDOUT!");
			stdout.flush().expect("Could not flush STDOUT after writing output!");
		},
		Err(message) => {
			eprintln!("Could not perform translation: {}", message);
		}
	}	
}
