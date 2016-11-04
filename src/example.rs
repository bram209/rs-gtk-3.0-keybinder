extern crate libkeybinder;
extern crate gtk;

use libkeybinder::keybinder;

use gtk::prelude::*;

fn main() {
    //init
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    keybinder::init();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);
    window.show_all();

    keybinder::bind("<Ctrl>A", || {
        println!("Works!");
//      println!("Visible = {}", window.is_visible());
        gtk::main_quit();
    });

    gtk::main();
}