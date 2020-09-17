extern crate gtk;

use gtk::{GtkWindowExt, WidgetExt, FileChooserDialog, FileChooserAction, DialogExt, ApplicationWindow, ResponseType, Builder, CssProvider, CssProviderExt, StyleContext, Button, ButtonExt, FileChooserExt};
use gtk::prelude::{BuilderExtManual};
use std::u32::MAX;
use std::path::{PathBuf, Path};

macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }

fn main() {

    assert_gtk_is_available();

    let builder = Builder::from_string(include_str!("../assets/ui.glade"));
    let window: ApplicationWindow = builder.get_object("main_window").unwrap();
    let provider = CssProvider::new();
    let select_dir_button: Button = builder.get_object("select_dir").unwrap();

    select_dir_button.connect_clicked(clone!(window => move |_| {
        download_and_patch_if_applicable(&window)
    }));

    provider.load_from_path("assets/style.css").unwrap();

    StyleContext::add_provider_for_screen(&window.get_screen().unwrap(), &provider, MAX);
    window.show_all();

    gtk::main();
}

fn assert_gtk_is_available() {
    if gtk::init().is_err() {
        panic!("GTK cant be executed!")
    }
}

fn download_and_patch_if_applicable(window: &ApplicationWindow) {
    let result = open_file_explorer(&window, FileChooserAction::SelectFolder, "Select directory of Guild Wars 2");
    let path: &Path = result.as_path();

    if path.exists() {
        println!("Der Scheiß existiert tatsächlich!");
        return;
    }

   // println!(":{}", result.into_os_string().into_string().unwrap_or_default())
}

fn open_file_explorer(window: &ApplicationWindow, action: FileChooserAction, title: &str) -> PathBuf {
    let file: PathBuf;

    let dialog = FileChooserDialog::new(
        Some(title),
        Some(window),
        action,
    );

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Accept", ResponseType::Accept);

    let result = dialog.run();

    if result == ResponseType::Cancel {
        dialog.close()
    }

    file = dialog.get_filename().unwrap_or_default();
    dialog.close();
    file
}