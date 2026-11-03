/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use color_eyre::Result;
use poem_openapi::OpenApiService;

mod controller;
mod dto;
mod error;
mod service;
mod types;

const API_VERSION: &str = "1.0";
const API_TITLE: &str = "Unducked, a search engine redirector";
const API_IP: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    init_env()?;

    let api_service =
        OpenApiService::new(controller::search::SearchController, API_TITLE, API_VERSION)
            .server("/");
    let ui = api_service.swagger_ui();
    let app = poem::Route::new().nest("/docs", ui).nest("/", api_service);
    poem::Server::new(poem::listener::TcpListener::bind(API_IP))
        .run(app)
        .await?;

    Ok(())
}

fn init_env() -> Result<()> {
    color_eyre::install()?;

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "debug") };
    }

    tracing_subscriber::fmt::init();

    Ok(())
}
