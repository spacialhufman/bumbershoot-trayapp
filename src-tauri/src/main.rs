#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;

use application::application::BumbershootApp;
use bumbershoot::lib::factory_app_list;

fn main() {
    let application = BumbershootApp::new(factory_app_list());

    application.run();
}
