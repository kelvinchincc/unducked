/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use color_eyre::Result;
use poem::{Response, endpoint::StaticFilesEndpoint, get, handler, http::header};
use poem_openapi::OpenApiService;

mod constants;
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
            .server("/search");
    let ui = api_service.swagger_ui();
    let app = poem::Route::new()
        .nest("/docs", ui)
        .nest("/search", api_service)
        .at("/opensearch.xml", get(opensearch_xml))
        .nest(
            "/",
            StaticFilesEndpoint::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))
                .index_file("index.html"),
        );
    poem::Server::new(poem::listener::TcpListener::bind(API_IP))
        .run(app)
        .await?;

    Ok(())
}

#[handler]
fn opensearch_xml() -> Response {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
    <ShortName>Unducked</ShortName>
    <Description>Search engine redirector</Description>
    <InputEncoding>UTF-8</InputEncoding>
    <!-- Replace the host with your deployed root address. OpenSearch requires an absolute URL. -->
    <Url type="text/html" method="get" template="http://localhost:3000/search?q={{searchTerms}}" />
    <Url
        type="application/x-suggestions+json"
        method="get"
        template="https://duckduckgo.com/ac/?q={{searchTerms}}&amp;type=list"
    />

</OpenSearchDescription>"#
    );

    Response::builder()
        .header(
            header::CONTENT_TYPE,
            "application/opensearchdescription+xml",
        )
        .body(xml)
}

fn init_env() -> Result<()> {
    color_eyre::install()?;

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "debug") };
    }

    tracing_subscriber::fmt::init();

    Ok(())
}
