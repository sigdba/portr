use std::process;

fn main() {
    if let Err(e) = portr::run() {
        eprintln!("Fatal error: {}", e);
        process::exit(-1);
    }
}
