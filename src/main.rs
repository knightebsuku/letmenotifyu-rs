mod background;
mod database;
mod yify;

fn main() {
    println!("Hello, world!");
    match database::migration() {
        Ok(()) => println!("Migration complete"),
        Err(err) => println!("{}", err),
    }
    println!("OK");
    let handle = background::update();
    handle.join().unwrap();
}
