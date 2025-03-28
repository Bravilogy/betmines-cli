pub fn get_base_url(filename: String, live: bool) -> &'static str {
    let live_url = "https://betmines.com/vip/live-filters";
    let pre_match_url = "https://betmines.com/vip/pre-match-scanner-for-football";

    if live || filename.contains("live") {
        return live_url;
    }

    pre_match_url
}
