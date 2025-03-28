use crate::errors::CliError;
use std::process::Command;

pub fn execute_script(script: &str, args: &[&str]) -> Result<String, CliError> {
    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .output()
        .map_err(|e| CliError::CommandFailed(format!("Failed to execute {}: {}", script, e)))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CliError::CommandFailed(format!(
            "Command failed: {}",
            stderr
        )))
    }
}

pub fn import_filter(filter_id: i32, live: bool) -> Result<(), CliError> {
    let base_url = crate::utils::config::get_copy_endpoint(live);
    let import_url = format!("{}/{}", base_url, filter_id);

    match execute_script("requests/import_filter.sh", &[&import_url]) {
        Ok(_) => {
            log::info!("{} imported successfully", filter_id);
            Ok(())
        }
        Err(e) => Err(CliError::ImportError(format!(
            "Failed to import filter {}: {}",
            filter_id, e
        ))),
    }
}

pub fn delete_filter(filter_id: i32, live: bool) -> Result<(), CliError> {
    let base_url = crate::utils::config::get_delete_endpoint(live);
    let delete_url = format!("{}{}", base_url, filter_id);

    match execute_script("requests/delete_filter.sh", &[&delete_url]) {
        Ok(_) => {
            log::info!("Deleted filter {}", filter_id);
            Ok(())
        }
        Err(e) => Err(CliError::DeletionError(format!(
            "Failed to delete filter {}: {}",
            filter_id, e
        ))),
    }
}

pub fn fetch_filters() -> Result<(), CliError> {
    execute_script("requests/fetch_filters.sh", &[])
        .map(|_| ())
        .map_err(|e| CliError::CommandFailed(format!("Failed to fetch filters: {}", e)))
}
