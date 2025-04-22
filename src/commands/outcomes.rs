use std::collections::HashMap;

use crate::{
    errors::CliError,
    models::filter::Filter,
    utils::{command, filesystem, paths},
};

fn create_outcome_mapping() -> HashMap<String, String> {
    let mut mapping = HashMap::new();

    mapping.insert("Favorite wins".to_string(), "(Fav)".to_string());
    mapping.insert("Favorite wins at home".to_string(), "(Fav)".to_string());
    mapping.insert("Underdog wins".to_string(), "(Underdog)".to_string());
    mapping.insert(
        "Underdog wins at home".to_string(),
        "(Underdog)".to_string(),
    );
    mapping.insert("X".to_string(), "(X)".to_string());
    mapping.insert("1".to_string(), "(1)".to_string());
    mapping.insert("2".to_string(), "(2)".to_string());
    mapping.insert("12".to_string(), "(12)".to_string());
    mapping.insert("1X".to_string(), "(1X)".to_string());
    mapping.insert("X2".to_string(), "(X2)".to_string());
    mapping.insert("GG".to_string(), "(GG)".to_string());
    mapping.insert("NG".to_string(), "(NG)".to_string());
    mapping.insert("O15".to_string(), "(+1.5)".to_string());
    mapping.insert("O25".to_string(), "(+2.5)".to_string());
    mapping.insert("O35".to_string(), "(+3.5)".to_string());
    mapping.insert("U15".to_string(), "(-1.5)".to_string());
    mapping.insert("U45".to_string(), "(-4.5)".to_string());
    mapping.insert("+0.5".to_string(), "(+0.5)".to_string());
    mapping.insert("+0.5HT".to_string(), "(+0.5HT)".to_string());
    mapping.insert("+1.5HT".to_string(), "(+1.5HT)".to_string());
    mapping.insert("-0.5".to_string(), "(-0.5)".to_string());
    mapping.insert("-1.5HT".to_string(), "(-1.5HT)".to_string());
    mapping.insert("Home_O15".to_string(), "(Home +1.5)".to_string());
    mapping.insert("Home_U15".to_string(), "(Home -1.5)".to_string());
    mapping.insert("1 HT".to_string(), "(1HT)".to_string());
    mapping.insert("2 HT".to_string(), "(2HT)".to_string());
    mapping.insert("X HT".to_string(), "(XHT)".to_string());
    mapping.insert("GG HT".to_string(), "(+1.5HT)".to_string());
    mapping.insert("Over 0.5 Since Picked".to_string(), "(+1G)".to_string());
    mapping.insert("Over 1.5 Since Picked".to_string(), "(+2G)".to_string());

    mapping
}

pub fn run(live: bool, dry_run: bool) -> Result<(), CliError> {
    // Get path to the existing file
    let filename = paths::get_existing_path(live);

    // Load data from file
    let filters: Vec<Filter> = filesystem::load_data(filename.to_string())?;
    log::info!("Loaded {} filters for renaming", filters.len());

    // Create mapping of outcomes to service name
    let mapping = create_outcome_mapping();

    // Base API URL for updating filters
    let api_base = crate::utils::config::get_api_base_url(live);

    let mut updated = 0;
    let mut skipped = 0;
    let mut errors = 0;

    for filter in filters {
        if let Some(outcome) = &filter.desired_outcome {
            if let Some(new_name) = mapping.get(outcome) {
                if filter.name != *new_name {
                    log::info!(
                        "Filter {} - Updating '{}' to '{}' based on outcome '{}'",
                        filter.id,
                        filter.name,
                        new_name,
                        outcome
                    );

                    if !dry_run {
                        match command::execute_script(
                            "requests/update_filter.sh",
                            &[api_base, &filter.id.to_string(), new_name, &filename],
                        ) {
                            Ok(_) => {
                                log::info!("Successfully updated filter {}", filter.id);
                                updated += 1;
                            }
                            Err(e) => {
                                log::error!("Failed to update filter {}: {}", filter.id, e);
                                errors += 1;
                            }
                        }
                    } else {
                        updated += 1;
                    }
                } else {
                    log::debug!(
                        "Filter {} - Name already correct ('{}')",
                        filter.id,
                        filter.name,
                    );
                    skipped += 1;
                }
            } else {
                log::warn!(
                    "No mapping found for outcome '{}' on filter {}",
                    outcome,
                    filter.id,
                );
                skipped += 1;
            }
        } else {
            log::warn!("Filter {} has no desired outcome", filter.id);
            skipped += 1;
        }
    }

    let mode = if dry_run { "Dry run" } else { "Renaming" };

    log::info!(
        "{} completed. Updated: {}, Skipped: {}, Errors: {}",
        mode,
        updated,
        skipped,
        errors
    );

    Ok(())
}
