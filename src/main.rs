extern crate gtk;
use gtk::prelude::*;

fn main() 
{
    gtk::init();

    let glade_src = include_str!("../res/file_explorer.glade");

    let builder = gtk::Builder::new_from_string(glade_src);

    let window : gtk::Window = builder.get_object("windowMain").unwrap();
    let btn_run : gtk::Button = builder.get_object("btnRun").unwrap();
    let btn_quit : gtk::Button = builder.get_object("btnQuit").unwrap();
    let entry_command : gtk::Entry = builder.get_object("entryCommand").unwrap();
    let cmb_option : gtk::ComboBoxText = builder.get_object("cmbOption").unwrap();

    window.connect_delete_event
    (|_, _|
        {
            gtk::main_quit();
            gtk::Inhibit(true)
        }
    );

    btn_run.connect_clicked
    (
        move|_|
        {
            let text = &entry_command.get_text().unwrap(); 
            let option = &cmb_option.get_active_text().unwrap();
            println!("{}", text);
            println!("{}", option);
        }
    );

    btn_quit.connect_clicked
    (
        move|_|
        {
            gtk::main_quit();
        }
    );

    window.show_all();
        
    gtk::main();
}
