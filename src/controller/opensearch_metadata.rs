/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use poem::{Request, Response, RouteMethod, get, handler, http::header};

pub struct OpenSearchMetaController;

impl OpenSearchMetaController {
    pub fn endpoint() -> RouteMethod {
        get(get_opensearch_metadata)
    }
}

#[handler]
fn get_opensearch_metadata(req: &Request) -> Response {
    let host = req.header("host").unwrap_or("localhost:3000");
    let base = format!("{}://{}", req.scheme().as_str(), host);

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
    <ShortName>Unducked</ShortName>
    <Description>Search engine redirector</Description>
    <InputEncoding>UTF-8</InputEncoding>
    <Url type="text/html" method="get" template="{base}/search?q={{searchTerms}}" />
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
