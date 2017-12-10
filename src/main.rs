extern crate gtk;
use gtk::prelude::*;

fn main() 
{
    gtk::init();

    let glade_src = include_str!("../res/file_explorer.glade");

    let builder = gtk::Builder::new_from_string(glade_src);

    let window : gtk::Window = builder.get_object("windowMain").unwrap();

    window.connect_delete_event
    (|_, _|
        {
            gtk::main_quit();
            gtk::Inhibit(true)
        }
    );

    window.show_all();
        
    gtk::main();
}
