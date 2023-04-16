use medman::cli::CliArguments;
use medman::scan::scan;
use std::io;
use std::path::Path;
use lofty::{Accessor, AudioFile, Probe};


fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = Path::new(path.trim());

	

	let tagged_file = Probe::open(path)
				.expect("ERROR: Bad path provided!")
				.read()
				.expect("ERROR: Failed to read file!");
		
			let tag = match tagged_file.primary_tag() {
				Some(primary_tag) => primary_tag,
				// If the "primary" tag doesn't exist, we just grab the
				// first tag we can find. Realistically, a tag reader would likely
				// iterate through the tags to find a suitable one.
				None => tagged_file.first_tag().expect("ERROR: No tags found!"),
			};
		
			println!("--- Tag Information ---");
			println!("Title: {}", tag.title().unwrap_or("None"));
			println!("Artist: {}", tag.artist().unwrap_or("None"));
			println!("Album: {}", tag.album().unwrap_or("None"));
			println!("Genre: {}", tag.genre().unwrap_or("None"));
		
			let properties = tagged_file.properties();
		
			let duration = properties.duration();
			let seconds = duration.as_secs() % 60;
		
			let duration_display = format!("{:02}:{:02}", (duration.as_secs() - seconds) / 60, seconds);
		
			println!("--- Audio Properties ---");
			println!("Duration: {}", duration_display);
		
    

	let args = CliArguments::new();
		println!("{:?}", args);
	
		let music_files = scan(args.path());
		for music_file in music_files {
			println!("{:?}", music_file)
		}
}
