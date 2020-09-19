#![windows_subsystem = "windows"]
#![feature(async_closure)]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate reqwest;

use nwd::NwgUi;
use nwg::{NativeUi};
use std::path::Path;
use std::fs::{File};
use std::fs::remove_file;

#[derive(Default, NwgUi)]
pub struct Application {
    #[nwg_control(size: (600, 500), position: (0, 0), title: "Arc DPS Updater", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()],)]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 45, weight: 600)]
    font: nwg::Font,

    #[nwg_control(text: "ARC DPS UPDATER", h_align: HTextAlign::Center, font: Some(& data.font))]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 2)]
    title: nwg::Label,

    #[nwg_resource(source_file: Some("./assets/logo.png"))]
    logo: nwg::Bitmap,

    #[nwg_control(bitmap: Some(& data.logo))]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 3, col_span: 2)]
    img: nwg::ImageFrame,

    #[nwg_control(text: "No directory selected", flags: "VISIBLE|DISABLED")]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, row_span: 1, col_span: 2)]
    selected_dir: nwg::TextInput,

    #[nwg_control(text: "Select Guild Wars 2 directory")]
    #[nwg_layout_item(layout: grid, col: 0, row: 5, row_span: 1)]
    #[nwg_events(OnButtonClick: [Application::select_directory])]
    select_dir_button: nwg::Button,

    #[nwg_control(text: "Patch Arc DPS", enabled: false)]
    #[nwg_layout_item(layout: grid, col: 1, row: 5, row_span: 1)]
    #[nwg_events(OnButtonClick: [Application::replace_file_with_new_downloaded_version])]
    patch_button: nwg::Button,

    #[nwg_resource(title: "Select Guild Wars 2 directory", action: nwg::FileDialogAction::OpenDirectory)]
    select_install_dir_dialog: nwg::FileDialog,

    #[nwg_control]
    #[nwg_events(OnNotice: [Application::after_download_is_done_callback])]
    download_notice: nwg::Notice,

    compute: std::cell::RefCell<Option<std::thread::JoinHandle<()>>>,
}

impl Application {
    const DESTINATION_FILE_NAME: &'static str = "d3d9.dll";
    const DOWNLOAD_URL: &'static str = "https://www.deltaconnected.com/arcdps/x64/d3d9.dll";

    fn select_directory(&self) {
        self.selected_dir.set_text("No directory selected");
        self.patch_button.set_enabled(false);

        if self.select_install_dir_dialog.run(Some(&self.window)) {
            let result = self.select_install_dir_dialog.get_selected_item().unwrap_or_default();

            if result == "" {
                return;
            }

            if !Path::new(result.as_str()).exists() {
                return;
            }

            self.selected_dir.set_text(result.as_str());
            self.patch_button.set_enabled(true);
        }
    }

    fn remove_old_file(&self) {
        let olf_file_path = self.get_destination_file_path();
        let old_file_path = Path::new( olf_file_path.as_str());

        if !old_file_path.exists() {
            return;
        }

        remove_file(old_file_path).expect("Can't delete old arc dps file");
    }

    fn replace_file_with_new_downloaded_version(&self) {
        self.patch_button.set_enabled(false);
        self.select_dir_button.set_enabled(false);

        self.remove_old_file();

        let sender = self.download_notice.sender();
        let file_path_as_string = self.get_destination_file_path();

        *self.compute.borrow_mut() = Some(std::thread::spawn(move || {
            let mut file = File::create(file_path_as_string).unwrap();

            let mut response = reqwest::blocking::get(Application::DOWNLOAD_URL).expect("Request failed");

            response.copy_to(&mut file).expect("Couldn't patch arc dps");

            sender.notice();
        }));
    }

    fn after_download_is_done_callback(&self){
        self.patch_button.set_enabled(true);
        self.select_dir_button.set_enabled(true);
        nwg::simple_message("Patched!", "Arc DPS successfully patched. Happy Playing!");
    }

    fn get_destination_file_path(&self) -> String {
        let mut destination_file_path_as_string = self.selected_dir.text().clone();
        destination_file_path_as_string.push_str("\\bin64\\");
        destination_file_path_as_string.push_str(Application::DESTINATION_FILE_NAME);
        destination_file_path_as_string
    }
}

fn main() {
    nwg::init().expect("Failed to init GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = Application::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}