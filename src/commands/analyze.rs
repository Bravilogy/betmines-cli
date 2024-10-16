use crate::{errors, models::filter::Filter};

use std::{fs, io::BufReader};

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

fn calculate_score(filter: &Filter) -> f64 {
    let roi_weight = 0.5;
    let sr_weight = 0.3;
    let ps_weight = 0.2;

    if filter.roi < 0.0 {
        return 0.0;
    }

    roi_weight * filter.roi as f64
        + sr_weight * filter.success_rate as f64
        + ps_weight * filter.total_picks as f64
}

pub fn run(filename: &str, count: usize, open: bool, live: bool) -> Result<(), errors::CliError> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    let data: Vec<Filter> = serde_json::from_reader(reader)?;

    let mut filtered_data: Vec<Filter> = data
        .into_iter()
        .filter(|entry| is_valid_roi(entry.roi) && is_valid_desired_outcome(&entry.desired_outcome))
        .map(|mut entry| {
            entry.score = calculate_score(&entry);
            entry
        })
        .collect();

    filtered_data.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    filtered_data.truncate(count);

    let base_url = get_base_url(live, &filename.to_string());

    for (i, item) in filtered_data.iter().enumerate() {
        let url = format!("{}/{}/history", base_url, item.id);

        if open {
            if let Err(err) = open::that(&url) {
                eprintln!("Failed to open URL: {}. Error: {}", url, err);
            }
            continue;
        }

        println!(
            "ROI: {:.2}%\nTotal Picks: {}\nSuccess Rate: {:.2}%\nScore is {:.2}\nURL: {}",
            item.roi, item.total_picks, item.success_rate, item.score, url,
        );

        if i < filtered_data.len() - 1 {
            println!("\n")
        }
    }

    Ok(())
}
