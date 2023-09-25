#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bumbershoot::{
    application::application::BumbershootApp,
    factories::factories::factory_app_list
};

fn main() {
    let application = BumbershootApp::new(factory_app_list());

    application.run();
}
