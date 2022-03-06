mod client;
mod db;
mod gemini;
mod header;
mod response;
mod tls;

use std::rc::Rc;

use url::Url;

use crate::client::GeminiClient;
use crate::db::Db;
use crate::tls::verification::TofuVerifier;

fn main() -> anyhow::Result<()> {
    let db = Db::new("dioscuri.sqlite")?;
    db.prepare()?;

    let verifier = Rc::new(TofuVerifier::new(db));
    let client = GeminiClient::new(verifier)?;

    let url = Url::parse("gemini://geminiquickst.art/").unwrap();
    let rsp = client.get(&url)?;

    let body = rsp.body().unwrap();
    let document = gemini::build_document(body, &url);

    dbg!(&document.lines());
    dbg!(&rsp.header().status());
    dbg!(&rsp.header().inner());
    dbg!(&rsp.url());

    Ok(())
}
