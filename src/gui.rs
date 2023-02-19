use gdk::prelude::*;
use gtk::prelude::{ContainerExt, GridExt, GtkWindowExt, WidgetExt};
use gtk::Entry;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Listen to keyboard events with from GTK");
    window.set_default_size(200, 120);

    let margin = 5;

    let grid = gtk::Grid::builder()
        .margin(margin)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    window.set_child(Some(&grid));

    // GTK keyboard events listener
    window.connect("key_press_event", false, |values| {
        let raw_event = &values[1].get::<gdk::Event>().unwrap();
        match raw_event.downcast_ref::<gdk::EventKey>() {
            Some(event) => {
                println!("Key code: {:?}", event.keycode());
                println!("Key val: {:?}", event.keyval());
                println!("Modifier: {:?}", event.state());
            }
            None => {}
        }

        let result = glib::value::Value::from_type(glib::types::Type::BOOL);
        Some(result)
    });

    let entry = Entry::builder().margin(margin).build();
    grid.attach(&entry, 0, 0, 3, 1);

    window.show_all();
}
