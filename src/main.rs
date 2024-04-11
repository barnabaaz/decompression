use decompression::{run, Config};

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        std::process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application Error : {}", e);
        std::process::exit(1)
    }
}
