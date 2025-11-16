use std::fs::OpenOptions;
use std::io::Write;
use std::env;

fn main() -> std::io::Result<()> {

	// Arguments
	let args: Vec<String> = env::args().collect();
	if args.len() != 4 {
		panic!("use arguments");
	}
	
	// Convert abbreviation
	let mut bytes : u64 = args[2].parse::<u64>().expect("use numbers");
	match args[3].chars().last().unwrap_or('B') {
		'B' => {}
		'K' => bytes *= 1024,
		'M' => bytes *= 1024u64.pow(2),
		'G' => bytes *= 1024u64.pow(3),
		'T' => bytes *= 1024u64.pow(4),
		_ => panic!("use an actual value"),
	}

	// File creating and writing
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(args[1].clone())
		.expect("cannot open file");

	let mut written: u64 = 0;
	let chunk_size = 1024 * 1024 * 1024;
	let buffer = vec![b'w'; chunk_size];
	while written < bytes {
		let to_write = std::cmp::min(chunk_size as u64, bytes - written) as usize;
		file.write_all(&buffer[..to_write])?;
		written += to_write as u64;

		// Progress bar
		let percent = written as f64 / bytes as f64 * 100.0;
		let bar_len = 50;
		let filled_len = (bar_len as f64 * percent / 100.0) as usize;
		let bar = "=".repeat(filled_len) + &" ".repeat(bar_len - filled_len);
		print!("\r[{}] {:.2}%", bar, percent);
		std::io::stdout().flush()?;
	}
	
	Ok(())
}
