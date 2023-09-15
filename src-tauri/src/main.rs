#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod myguest;
mod sending;
mod wispot_api;
mod wispot_integration;

use application::application::BumbershootApp;
use myguest::myguest::MyGuest;
use sending::sending::Sending;
use wispot_api::wispot_api::WispotApi;
use wispot_integration::wispot_integration::WispotIntegration;

fn main() {
    let application = BumbershootApp::new(
        MyGuest::new(),
        Sending::new(),
        WispotApi::new(),
        WispotIntegration::new()
    );

    application.run();
}
