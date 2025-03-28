use crate::models::filter_traits::{FilterScoring, FilterValidation};
use std::collections::{HashMap, HashSet};

use crate::{
    errors::CliError,
    models::filter::Filter,
    utils::{config, filesystem},
};

pub fn find_duplicates(filters: &[Filter]) -> HashMap<Filter, Vec<i32>> {
    let mut seen: HashMap<Filter, Vec<i32>> = HashMap::new();

    for filter in filters {
        seen.entry(filter.clone())
            .or_insert_with(Vec::new)
            .push(filter.id);
    }

    seen.into_iter().filter(|(_, ids)| ids.len() > 1).collect()
}

pub fn remove_existing_filters(
    data: Vec<Filter>,
    existing_path: &str,
) -> Result<Vec<Filter>, CliError> {
    let existing_data = filesystem::load_data(existing_path.to_string())?;
    let existing_set: HashSet<_> = existing_data.iter().collect();

    Ok(data
        .into_iter()
        .filter(|filter| !existing_set.contains(filter))
        .collect())
}

pub fn filter_valid_entries(filters: Vec<Filter>) -> Vec<Filter> {
    filters
        .into_iter()
        .filter(|entry| entry.is_valid())
        .collect()
}

pub fn filter_low_performing(filters: Vec<Filter>) -> Vec<Filter> {
    filters
        .into_iter()
        .filter(|filter| filter.is_low_performing())
        .collect()
}

pub fn sort_by_score(mut filters: Vec<Filter>) -> Vec<Filter> {
    filters.sort_by(|a, b| {
        b.get_score()
            .partial_cmp(&a.get_score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    filters
}

pub fn display_filters(
    filters: &[Filter],
    open: bool,
    live: bool,
    verbose: bool,
) -> Result<(), CliError> {
    let base_url = config::get_web_base_url(live);

    for (i, item) in filters.iter().enumerate() {
        let url = format!("{}/{}/history", base_url, item.id);

        if open {
            if let Err(err) = open::that(&url) {
                return Err(CliError::BrowserError(format!(
                    "Failed to open URL: {}. Error: {}",
                    url, err
                )));
            }
            continue;
        }

        if verbose {
            log::info!(
                "ROI: {:.2}%\nTotal Picks: {}\nSuccess Rate: {:.2}%\nScore is {:.2}\nURL: {}",
                item.roi,
                item.total_picks,
                item.success_rate,
                item.get_score(),
                url,
            );

            if i < filters.len() - 1 {
                log::info!("\n");
            }
        } else {
            log::info!("{}", url);
        }
    }

    Ok(())
}
