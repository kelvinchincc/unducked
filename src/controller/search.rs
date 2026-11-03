/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::{api_errors, controller::ApiTag};
use color_eyre::eyre::Result;
use poem_openapi::{OpenApi, payload::PlainText};

pub struct SearchController;

#[OpenApi(tag = "ApiTag::SearchController", prefix_path = "/search")]
impl SearchController {
    #[oai(path = "/", method = "get")]
    pub async fn search(&self) -> Result<PlainText<String>, SearchQueryError> {
        Ok(PlainText("Search results".to_string()))
    }
}

api_errors!(pub SearchQueryError { BadRequest = 400 => "Invalid search query" });
