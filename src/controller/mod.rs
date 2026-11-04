/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub mod opensearch_metadata;
pub mod search;
pub mod static_files;

use poem_openapi::Tags;

#[derive(Tags)]
enum ApiTag {
    #[oai(rename = "Search Controller")]
    SearchController,
}
