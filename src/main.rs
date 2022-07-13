use lib;

fn main() {
    match lib::run() {
        Ok(_) => {},
        Err(e) => println!("Application error: {}", e),
    }
}
