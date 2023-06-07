// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod myguest;
mod sending;
mod wispot_api;

use application::application::BumbershootApp;
use myguest::myguest::MyGuest;
use sending::sending::Sending;
use wispot_api::wispot_api::WispotApi;

fn main() {
    let application = BumbershootApp::new(
        MyGuest::new(),
        Sending::new(),
        WispotApi::new()
    );

    application.run();
}
