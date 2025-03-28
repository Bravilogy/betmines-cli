pub struct FilePaths {
    pub live_data: &'static str,
    pub pre_match_data: &'static str,
    pub existing_live: &'static str,
    pub existing_pre_match: &'static str,
}

pub const PATHS: FilePaths = FilePaths {
    live_data: "live.json",
    pre_match_data: "pre.json",
    existing_live: "existing_live.json",
    existing_pre_match: "existing_pre.json",
};

pub fn get_data_path(live: bool) -> &'static str {
    if live {
        PATHS.live_data
    } else {
        PATHS.pre_match_data
    }
}

pub fn get_existing_path(live: bool) -> &'static str {
    if live {
        PATHS.existing_live
    } else {
        PATHS.existing_pre_match
    }
}
