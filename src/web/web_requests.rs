use reqwest::Client;

pub async fn make_call(path: &String) -> Result<String, reqwest::Error> {
    let response = Client::new()
        .get(path)
        .send().await?;
    let body = response.text().await?;
    Ok(body)
}
