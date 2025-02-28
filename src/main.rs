use std::fs::File;
use rodio::{Decoder, OutputStream, Source, Sink};
use std::io::{self, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let user_choice = Arc::new(Mutex::new(String::new())); // Shared user input

    loop {
        println!("Hvad vil du?");
        println!("1: Høre en sang");
        println!("2: Lukke programmet");

        let mut valgmulighed = String::new();
        io::stdin().read_line(&mut valgmulighed).expect("Fejl ved læsning af input");

        let valgmulighed = valgmulighed.trim();

        match valgmulighed {
            "1" => {
                println!("Indtast sangnavnet (Lige nu kun 'Traitor'): ");
                let mut song_name = String::new();
                io::stdin().read_line(&mut song_name).expect("Fejl ved læsning af sangnavn");
                let song_name = song_name.trim();

                // Directly construct the path by prepending 'src/music/' to the song name
                let song_path = format!("src/music/{}.mp3", song_name);
                println!("Afspiller: {}", song_name);

                // Handle the Result from play_music and print any errors
                if let Err(e) = play_music(song_path) {
                    eprintln!("Fejl under afspilning af musik: {}", e);
                }
            },
            "2" => {
                println!("Lukker programmet");
                break; // Exit the loop and terminate the program
            },
            _ => println!("Vælg en gyldig mulighed."),
        }
    }
}

fn play_music(path: String) -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to initialize the output stream and handle errors
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Create a Sink using try_new(), which returns a Result
    let sink = Sink::try_new(&stream_handle)?;

    // Try to open the file, which can fail if the file doesn't exist or is invalid
    let file = BufReader::new(File::open(&path)?);

    // Try to decode the file, which can fail if the file is not a valid audio format
    let source = Decoder::new(file)?;

    // Add the source to the sink (this plays the entire song)
    sink.append(source);

    // Wait for the music to finish playing
    sink.sleep_until_end();
    Ok(())
}

// vi forstår ikke helt implementeringen af arc og mutex, og vi var ikke i stand til at implementere pause stop osv.
