extern crate rgtk;

use rgtk::*;

pub fn new_project(state: &mut ::utils::State) {
    let chooser = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::enums::file_chooser_action::CreateFolder).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        state.projects.insert(filename.unwrap());
    }
    chooser.destroy();
}

pub fn import_project(state: &mut ::utils::State) {
    let chooser = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::enums::file_chooser_action::SelectFolder).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        state.projects.insert(filename.unwrap());
    }
    chooser.destroy();
}

pub fn rename_project(state: &mut ::utils::State) {
    
}

pub fn remove_project(state: &mut ::utils::State) {
    
}