// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{ HashMap, LinkedList };

mod bumbershoot_app;
mod runners;
mod event_handler;

use bumbershoot_app::application::BumbershootApplication;
use runners::runners::{ Runner, AppRunner, SystemRunner };

fn main() {
    // criar factory
    let myguest = AppRunner::new(
        "MyGuest".to_string(),
        "7.2.33".to_string(),
        "c:/wamp64/www/wispot/myguest-new/artisan".to_string(),
        "dev-painel.myguest.com.br".to_string(),
        8002
    );

    // criar factory
    let sending = AppRunner::new(
        "Sending".to_string(),
        "7.4.9".to_string(),
        "c:/wamp64/www/wispot/sending/artisan".to_string(),
        "dev-painel.sending.com.br".to_string(),
        8082
    );

    // criar factory
    let wispot_api = AppRunner::new(
        "Wispot API".to_string(),
        "7.4.9".to_string(),
        "c:/wamp64/www/wispot/api.wispot.com.br/artisan".to_string(),
        "dev-api.wispot.com.br".to_string(),
        8003
    );

    let mut runners: LinkedList<&AppRunner> = LinkedList::new();
    runners.push_back(&myguest);
    runners.push_back(&sending);
    runners.push_back(&wispot_api);

    let system_runner = SystemRunner::new(runners);

    let mut app_runners: HashMap<String, &(dyn Runner + Sync + Send)> = HashMap::new();
    app_runners.insert("run-myguest".to_string(), &myguest);
    app_runners.insert("run-sending".to_string(), &sending);
    app_runners.insert("run-wispot_api".to_string(), &wispot_api);
    app_runners.insert("close".to_string(), &system_runner);

    let application = BumbershootApplication::new(app_runners);

    application.run();
}
