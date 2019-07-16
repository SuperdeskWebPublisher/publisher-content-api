pub fn generate_asset_url(asset_id: &String, file_extension: &String) -> String {
    let mut cdn_url = std::env::var("PUBLISHER_CDN_URL").expect("PUBLISHER_CDN_URL must be set");

    if cdn_url.ends_with('/') { 
        cdn_url.pop(); 
    }

    let mut url: String = cdn_url.to_owned();
    url.push_str("/");
    url.push_str(asset_id);
    url.push_str(".");
    url.push_str(file_extension);

    url
}
