mod api;
use api::scan;

fn main() {
    println!("Hello, world!");

    match scan(r"C:\Users\gusta\Downloads\60c4199364474569561cba359d486e6c69ae8cba.jpeg".to_owned())
    {
        Ok(_) => println!("Scanned succesfully"),
        Err(e) => println!("Failed with error: {:?}", e),
    }
}
