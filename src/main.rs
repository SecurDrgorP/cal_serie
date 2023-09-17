use std::io;
//use gtk::prelude::*;


mod methodes;
pub use crate::methodes::*;//you can use crate::...;

fn main() {
/*
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("My App");
    window.set_default_size(300, 200);

    let label = gtk::Label::new(Some("Hello, world!"));
    window.add(&label);

    window.show_all();
*/

    println!("{} and {}", 14 as char, 65 as char);
    
    println!("Input: ");
    let mut serie = String::new();
    io::stdin().read_line(&mut serie)
        .expect("Failed to read line");

    let vect_serie = vector_of(&serie);

    println!("{:?}", vect_serie);
    
    let nvector = ver_into(&vect_serie);

    println!("{:?}", nvector);

    println!("{:?}", nvector.parentheses_cal().run_cal());  
}








