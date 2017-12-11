extern crate gtk;
use gtk::prelude::*;

mod gui;

fn main() 
{
    gtk::init();

    gui::gui_main();    
    
    gtk::main();
}
