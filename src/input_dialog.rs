use gtk::prelude::*;
use gtk::{Entry, Window, WindowType, Button, Box as GtkBox, Orientation};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub struct TextInputDialog {
    title: String,
    placeholder: String,
}

impl TextInputDialog {
    pub fn new(title: &str, placeholder: &str) -> Self {
        Self {
            title: title.into(),
            placeholder: placeholder.into(),
        }
    }

    pub fn get_input(&self) -> Receiver<String> {
        let (s, r): (Sender<String>, Receiver<String>) = channel();
        let title = self.title.clone();
        let placeholder = self.placeholder.clone();

        thread::spawn(move || {
            if gtk::init().is_err() {
                println!("Failed to initialize GTK.");
                return;
            }

            let window = Window::new(WindowType::Toplevel);
            window.set_title(&title);
            window.set_default_size(350, 70);

            let vbox = GtkBox::new(Orientation::Vertical, 5);
            let entry = Entry::new();
            entry.set_placeholder_text(Some(&placeholder));
            let button = Button::with_label("Submit");

            vbox.pack_start(&entry, true, true, 0);
            vbox.pack_start(&button, true, true, 0);
            window.add(&vbox);

            let window_clone = window.clone(); // Clone the window for the button's clicked closure
            button.connect_clicked(move |_| {
                let text = entry.get_text().to_string();
                s.send(text).expect("Failed to send input through channel");
                window_clone.close(); // Use the cloned window reference here
            });

            window.connect_delete_event(move |_, _| {
                gtk::main_quit();
                Inhibit(false)
            });

            window.show_all();
            gtk::main();
        });

        r
    }
}