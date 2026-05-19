pub fn http_get(url: &str) -> Result<String, String> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| e.to_string())?;
    
    resp.into_string().map_err(|e| e.to_string())
}

pub fn http_post(url: &str, body: &str) -> Result<String, String> {
    let resp = ureq::post(url)
        .send_string(body)
        .map_err(|e| e.to_string())?;
    
    resp.into_string().map_err(|e| e.to_string())
}
