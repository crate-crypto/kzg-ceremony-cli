use crate::rest_api::RestAPI;
use log::info;

pub fn cmd(api: &RestAPI) {
    let url = api.link_to_current_transcript();
    info!("Opening : {}", url);
    open::with(&url, "firefox").unwrap();
}
