use crate::errors;
use crate::models::filter::Filter;
use crate::services::filter_service;
use crate::utils::{config, filesystem, paths};

pub fn run(filename: String, live: bool) -> Result<(), errors::CliError> {
    let file_path = if filename.is_empty() {
        paths::get_existing_path(live).to_string()
    } else {
        filename.clone()
    };

    let data: Vec<Filter> = filesystem::load_data(file_path)?;
    log::info!("Loaded {} filters to duplicate analysis", data.len());

    let duplicates = filter_service::find_duplicates(&data);
    log::info!("Found {} sets of duplicate filters", duplicates.len());

    // If no duplicates found, log and return
    if duplicates.is_empty() {
        log::info!("No duplicate filters found");
        return Ok(());
    }

    let base_url = config::get_web_base_url(live);

    for (filter, ids) in duplicates {
        log::info!("\nIdentical filters found ({} duplicates):", ids.len());
        if let Some(outcome) = &filter.desired_outcome {
            log::info!("Desired outcome: {}", outcome);
        }

        log::info!(
            "ROI: {:.2}%, Success Rate: {:.2}%",
            filter.roi,
            filter.success_rate
        );

        for id in ids {
            let url = format!("{}/{}/history", base_url, id);
            log::info!("Filter ID {}: {}", id, url);
        }
    }

    Ok(())
}
