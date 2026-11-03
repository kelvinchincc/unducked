/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::constants;

pub enum SearchEngineProvidier {
    /// DuckDuckGo
    #[allow(dead_code)]
    DDG,
    // Google
    GOOGLE,
}

impl Default for SearchEngineProvidier {
    fn default() -> Self {
        SearchEngineProvidier::GOOGLE
    }
}

pub fn get_search_engine_url(search_engine: SearchEngineProvidier, query: &str) -> String {
    let has_bang = constants::values::create_bang_regex().captures(query);

    if let Some(captures) = has_bang {
        // if the query has a bang and is '!d', we will redirect to DuckDuckGo, without the bang
        if let Some(bang) = captures.get(1) {
            if bang.as_str() == "ddg" {
                let query_without_bang = query
                    .replace(&format!("!{}", bang.as_str()), "")
                    .trim()
                    .to_string();
                log::debug!(
                    "Redirecting to DuckDuckGo with query: {}",
                    query_without_bang
                );
                return format!("https://duckduckgo.com/?q={}", query_without_bang);
            }
        }

        // if the query has a bang, we will redirect to DuckDuckGo
        return format!("https://duckduckgo.com/?q={}", query);
    }

    match search_engine {
        SearchEngineProvidier::DDG => format!("https://duckduckgo.com/?q={}", query),
        SearchEngineProvidier::GOOGLE => format!("https://www.google.com/search?q={}", query),
    }
}
