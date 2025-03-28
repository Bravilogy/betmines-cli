use crate::{
    errors,
    models::filter::Filter,
    services::filter_service,
    utils::{command, filesystem, paths},
};

pub fn run(live: bool) -> Result<(), errors::CliError> {
    // Get path to the existing file
    let filename = paths::get_existing_path(live);

    // Load data from file
    let data: Vec<Filter> = filesystem::load_data(filename.to_string())?;

    // Get only low performing filters
    let filters_to_delete = filter_service::filter_low_performing(data);

    log::info!("Found {} filters to delete", filters_to_delete.len());

    for filter in filters_to_delete {
        match command::delete_filter(filter.id, live) {
            Ok(_) => log::info!("Successfully deleted filter {}", filter.id),
            Err(err) => log::error!("Failed to delete filter {}: {}", filter.id, err),
        }
    }

    Ok(())
}
