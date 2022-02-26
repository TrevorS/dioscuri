mod client;
mod header;
mod response;
mod tls;

fn main() -> anyhow::Result<()> {
    let client = crate::client::GeminiClient::new();
    let url = url::Url::parse("gemini://geminiquickst.art/").unwrap();

    let rsp = client.get(&url)?;

    let body = std::str::from_utf8(rsp.body().unwrap())?;
    dbg!(&body);

    dbg!(&rsp.header().status());
    dbg!(&rsp.header().inner());
    dbg!(&rsp.url());

    Ok(())
}
