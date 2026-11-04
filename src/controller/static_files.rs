/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use poem::endpoint::StaticFilesEndpoint;

pub struct StaticFilesController;

impl StaticFilesController {
    pub fn endpoint() -> StaticFilesEndpoint {
        StaticFilesEndpoint::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))
            .index_file("index.html")
    }
}
