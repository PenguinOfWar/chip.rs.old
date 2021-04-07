// https://riptutorial.com/rust/example/23984/simple-gtkplus-window-with-text

extern crate gtk;

use chrono::Local;
use env_logger::Builder;
use gtk::prelude::*; // Import all the basic things
use gtk::{CssProvider, CssProviderExt, Label, StyleContext, Window, WindowType};
use log::LevelFilter;
use std::io::Write;

const CSS: &str = include_str!("styles/main.css");

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    if gtk::init().is_err() {
        //Initialize Gtk before doing anything with it
        panic!("Can't init GTK");
    }

    let window = Window::new(WindowType::Toplevel);

    //Destroy window on exit
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.set_title("CHIP");
    window.set_default_size(320, 240);
    gtk::WidgetExt::set_widget_name(&window, "window");

    // css
    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();
    let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
    StyleContext::add_provider_for_screen(&screen, &style, 800);

    let label = Label::new(Some("CHIP"));
    label.set_halign(gtk::Align::Center);
    label.set_valign(gtk::Align::Center);
    // We need to name it in order to be able to use its name as a CSS label to
    // apply CSS on it.
    gtk::WidgetExt::set_widget_name(&label, "title");

    window.add(&label);
    window.show_all();

    log::info!("Label: {:?}", label);

    gtk::main();
}
