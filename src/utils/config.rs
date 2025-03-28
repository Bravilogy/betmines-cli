pub struct UrlConfig {
    pub web_base_live: &'static str,
    pub web_base_pre_match: &'static str,
    pub api_base_live: &'static str,
    pub api_base_pre_match: &'static str,
}

pub const URLS: UrlConfig = UrlConfig {
    web_base_live: "https://betmines.com/vip/live-filters",
    web_base_pre_match: "https://betmines.com/vip/pre-match-scanner-for-football",
    api_base_live: "https://api.betmines.com/betmines/v1/livefilters",
    api_base_pre_match: "https://api.betmines.com/betmines/v1/preMatchfilters",
};

pub fn get_web_base_url(live: bool) -> &'static str {
    if live {
        URLS.web_base_live
    } else {
        URLS.web_base_pre_match
    }
}

pub fn get_api_base_url(live: bool) -> &'static str {
    if live {
        URLS.api_base_live
    } else {
        URLS.api_base_pre_match
    }
}

pub fn get_copy_endpoint(live: bool) -> String {
    format!("{}/copyFilter", get_api_base_url(live))
}

pub fn get_delete_endpoint(live: bool) -> String {
    format!("{}/", get_api_base_url(live))
}
