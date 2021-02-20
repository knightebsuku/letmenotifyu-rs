mod background;
mod database;
mod movie;
mod yify;

fn main() {
    println!("Hello, world!");
    match database::migration() {
        Ok(()) => println!("Migration complete"),
        Err(err) => panic!("Unable to complete migrations {}", err),
    }
    match yify::images_path() {
        Ok(()) => println!("image directory ok"),
        Err(err) => panic!("Unable to create an image directory"),
    }
    let handle = background::update();
    handle.join().unwrap();
}
