/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::{
    api_errors,
    controller::ApiTag,
    service::search_engine_provider_service::{self, SearchEngineProvidier},
};
use color_eyre::eyre::Result;
use poem_openapi::{ApiResponse, OpenApi, param::Query};

pub struct SearchController;

#[OpenApi(tag = "ApiTag::SearchController")]
impl SearchController {
    #[oai(path = "/", method = "get")]
    pub async fn search(&self, q: Query<String>) -> Result<SearchQueryResponse, SearchQueryError> {
        let search_engine = SearchEngineProvidier::default();
        let url = search_engine_provider_service::get_search_engine_url(search_engine, &q.0);

        // Redirect to the search engine URL
        Ok(SearchQueryResponse::MovedPermanently(url))
    }
}

api_errors!(pub SearchQueryError { BadRequest = 400 => "Invalid search query" });

#[derive(ApiResponse)]
pub enum SearchQueryResponse {
    #[oai(status = 301)]
    MovedPermanently(#[oai(header = "Location")] String),
}
