mod database;

fn main() {
    println!("Hello, world!");
    match database::migration() {
        Ok(()) => println!("Migration complete"),
        Err(err) => println!("{}", err),
    }
}
