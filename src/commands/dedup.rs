use crate::errors;
use crate::models::filter::Filter;
use crate::utils::url;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn run(filename: &String, live: bool) -> Result<(), errors::CliError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let data: Vec<Filter> = serde_json::from_reader(reader)?;

    let mut seen: HashMap<Filter, Vec<i32>> = HashMap::new();

    for filter in data.iter() {
        seen.entry(filter.clone())
            .or_insert_with(Vec::new)
            .push(filter.id);
    }

    let base_url = url::get_base_url(filename, live);

    for (_, ids) in seen.iter() {
        if ids.len() == 1 {
            continue;
        }

        println!("\nIdentical filters:");
        for id in ids {
            println!("URL: {}", format!("{}/{}/history", base_url, id));
        }
    }

    Ok(())
}
