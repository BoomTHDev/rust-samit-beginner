mod config;
mod models;
mod services;

fn main() {
    println!("Application starting.");
    config::init();
    models::user::show_user();
    models::product::show_product();
    services::auth::login();
    services::payment::process_payment(100.0);
    services::auth::logout();
    println!("Application finished successfully.");
}
