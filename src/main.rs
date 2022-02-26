mod client;
mod header;
mod response;
mod tls;

fn main() -> anyhow::Result<()> {
    let client = crate::client::GeminiClient::new();
    let url = url::Url::parse("gemini://geminiquickst.art/").unwrap();

    let rsp = client.get(&url)?;
    dbg!(&rsp.header());

    let body = std::str::from_utf8(rsp.body().unwrap())?;
    dbg!(&body);

    Ok(())
}
