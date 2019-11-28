pub fn generate_asset_url(asset_id: &String, file_extension: &String) -> String {
    let cdn_url = std::env::var("PUBLISHER_CDN_URL").expect("PUBLISHER_CDN_URL must be set");

    generate_url(cdn_url, asset_id, file_extension)
}

pub fn generate_avatar_url(asset_id: &String, file_extension: &String) -> String {
    let cdn_url = std::env::var("PUBLISHER_AVATAR_CDN_URL").expect("PUBLISHER_AVATAR_CDN_URL must be set");

    generate_url(cdn_url, asset_id, file_extension)
}

fn generate_url(mut cdn_url: String, asset_id: &String, file_extension: &String) -> String {
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
