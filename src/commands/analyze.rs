use crate::{errors, models::filter::Filter};

use std::{collections::HashSet, fs, io::BufReader};

fn is_valid_roi(roi: f32) -> bool {
    roi >= 20.0
}

fn is_valid_desired_outcome(outcome: &Option<String>) -> bool {
    outcome.as_ref().map_or(false, |value| {
        !value.starts_with("CO") && !value.starts_with("CU")
    })
}

fn get_base_url(live: bool, filename: &String) -> &'static str {
    let live_url = "https://betmines.com/vip/live-filters";
    let pre_match_url = "https://betmines.com/vip/pre-match-scanner-for-football";

    if live || filename.contains("live") {
        return live_url;
    }

    pre_match_url
}

fn load_data(filename: &str) -> Result<Vec<Filter>, errors::CliError> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let data: Vec<Filter> = serde_json::from_reader(reader)?;
    Ok(data)
}

fn calculate_score(filter: &Filter) -> f64 {
    // roi
    let roi_weight = 0.8;

    // success rate
    let sr_weight = 0.3;

    // picks
    let ps_weight = 0.1;

    if filter.roi < 0.0 {
        return 0.0;
    }

    roi_weight * filter.roi as f64
        + sr_weight * filter.success_rate as f64
        + ps_weight * filter.total_picks as f64
}

fn remove_existing_filters(
    data: Vec<Filter>,
    existing: &str,
) -> Result<Vec<Filter>, errors::CliError> {
    let existing_data = load_data(existing)?;
    let existing_set: HashSet<_> = existing_data.iter().collect();

    Ok(data
        .into_iter()
        .filter(|filter| !existing_set.contains(filter))
        .collect())
}

fn display_data(data: &[Filter], open: bool, live: bool, verbose: bool, filename: &str) {
    let base_url = get_base_url(live, &filename.to_string());

    for (i, item) in data.iter().enumerate() {
        let url = format!("{}/{}/history", base_url, item.id);

        if open {
            if let Err(err) = open::that(&url) {
                eprintln!("Failed to open URL: {}. Error: {}", url, err);
            }
            continue;
        }

        if verbose {
            println!(
                "ROI: {:.2}%\nTotal Picks: {}\nSuccess Rate: {:.2}%\nScore is {:.2}\nURL: {}",
                item.roi, item.total_picks, item.success_rate, item.score, url,
            );

            if i < data.len() - 1 {
                println!("\n")
            }
        } else {
            println!("{}", url);
        }
    }
}

pub fn run(
    filename: &str,
    existing: &Option<String>,
    count: usize,
    open: bool,
    live: bool,
    offset: usize,
    verbose: bool,
) -> Result<(), errors::CliError> {
    let data = load_data(filename)?;
    let data = if let Some(existing) = existing {
        remove_existing_filters(data, existing)?
    } else {
        data
    };

    // remove identical filters
    let set: HashSet<Filter> = data.into_iter().collect();
    let data: Vec<Filter> = set.into_iter().collect();

    // filter data based on ROI and desired outcome
    let mut filtered_data: Vec<Filter> = data
        .into_iter()
        .filter(|entry| is_valid_roi(entry.roi) && is_valid_desired_outcome(&entry.desired_outcome))
        .map(|mut entry| {
            entry.score = calculate_score(&entry);
            entry
        })
        .collect();

    // sort by score to get the best filters
    filtered_data.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if offset > 0 {
        filtered_data = filtered_data.into_iter().skip(offset).collect();
    }

    filtered_data.truncate(count);

    display_data(&filtered_data, open, live, verbose, filename);

    Ok(())
}
