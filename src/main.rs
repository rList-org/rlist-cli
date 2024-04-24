mod config;
mod routes;
mod state;

use std::sync::Arc;
use crate::routes::routes;

#[tokio::main]
async fn main() {
    // load config file
    let config = config::loader::load_config();
    let config::Config { site_profile, drives, address, port} = config;
    let bind_address = format!("{}:{}", address, port);

    // setup logging
    tracing_subscriber::fmt()
        .init();

    // set site profile as a shared immutable state
    let state = Arc::new(state::AppState {
        site_profile,
    });

    // load driver instances
    let driver_instance_future = drives
        .into_iter()
        .map(|driver_index| tokio::spawn(driver_index.create_instance()))
        .collect::<Vec<_>>();
    let mut driver_instances = Vec::new();
    for driver_instance in driver_instance_future {
        driver_instances.push(driver_instance.await.unwrap());
    }

    // create wheel
    let wheel = rlist_vfs::Wheel::new(driver_instances).await;

    let app = routes(state, wheel);

    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}