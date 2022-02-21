mod client;
mod header;
mod response;
mod tls;

fn main() {
    let client = crate::client::GeminiClient::new();

    let url = url::Url::parse("gemini://geminiquickst.art/").unwrap();
    let rsp = client.get(&url);

    dbg!(&rsp);
}
