use crate::{
    errors::CliError,
    models::filter::Filter,
    services::filter_service,
    utils::{command, filesystem, paths},
};

use std::collections::HashSet;

fn import_filters(filters: &[Filter], live: bool) -> Result<(), CliError> {
    log::info!("Starting import of {} filters", filters.len());

    let mut success_count = 0;
    let mut failure_count = 0;

    for filter in filters {
        match command::import_filter(filter.id, live) {
            Ok(_) => {
                log::info!("Successfully imported filter {}", filter.id);
                success_count += 1;
            }
            Err(e) => {
                log::error!("Failed to import filter {}: {}", filter.id, e);
                failure_count += 1;
            }
        }
    }

    log::info!(
        "Import completed. Success: {}, Failures: {}",
        success_count,
        failure_count
    );

    Ok(())
}

pub fn run(
    filename: String,
    existing: &Option<String>,
    count: usize,
    open: bool,
    live: bool,
    offset: usize,
    autoimport: bool,
    verbose: bool,
) -> Result<(), CliError> {
    // Determine existing filters file path
    let existing_path = existing
        .clone()
        .unwrap_or_else(|| paths::get_existing_path(live).to_string());

    // Load and filter data
    let raw_data: Vec<Filter> = filesystem::load_data(filename)?;
    log::info!("Loaded {} filters from source file", raw_data.len());

    // Remove filters that already exist
    let filtered_data = filter_service::remove_existing_filters(raw_data, &existing_path)?;
    log::info!(
        "Found {} new filters after removing existing ones",
        filtered_data.len()
    );

    let unique_set: HashSet<Filter> = filtered_data.into_iter().collect();
    let unique_data: Vec<Filter> = unique_set.into_iter().collect();
    log::info!(
        "Found {} unique filters after deduplication",
        unique_data.len()
    );

    // Filter by validity criteria
    let valid_filters = filter_service::filter_valid_entries(unique_data);
    log::info!("Found {} valid filters", valid_filters.len());

    let mut sorted_filters = filter_service::sort_by_score(valid_filters);

    // Apply offset if specified
    if offset > 0 {
        sorted_filters = sorted_filters.into_iter().skip(offset).collect();
        log::info!("Applied offset of {}", offset);
    }

    // Limit to requested count
    sorted_filters.truncate(count);
    log::info!("Selected top {} filters", sorted_filters.len());

    if autoimport {
        import_filters(&sorted_filters, live)?;
    } else {
        filter_service::display_filters(&sorted_filters, open, live, verbose)?;
    }

    Ok(())
}
