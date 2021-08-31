#![allow(non_snake_case)]
use rand::seq::SliceRandom;

fn pause() {
    extern "C" {
        fn _getch() -> std::os::raw::c_int;
    }

    println!("<< Press Any Key >>");
    let _ = unsafe { _getch() };
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut inputtext = String::new();  

    println!("Enter text to Whatinize.");
    std::io::stdin().read_line(&mut inputtext).expect("Did not enter a correct string");
    
    let mut inputtextbytes = inputtext.into_bytes();
    inputtextbytes.shuffle(&mut rng);
    let outputtext = String::from_utf8(inputtextbytes).unwrap();

    println!("\nWhatinized text: "); // On a different line or else it will scramble this as well for some reason
    println!("{}", outputtext);
    println!("");
    pause();
}