//! Antivirus.rs contains all functions associated with the antivirus UI in Tauri.
//! This module will handle state, requests, async, and events.

use std::sync::Arc;
use tauri::{Emitter, State};
use std::path::PathBuf;
use um_engine::UmEngine;

#[tauri::command]
pub fn check_page_state(
    engine: State<'_, Arc<UmEngine>>,
) -> Result<String, ()> {

    let engine = Arc::clone(&engine);

    // todo from here, regular poll of status of the scan - maybe every second
    // this should also fetch data on files scanned, time taken, etc.
    let state = engine.scanner_get_state();
    println!("[i] State: {:?}", state);

    // app_handle.emit("state_update", state).unwrap();

    let state = serde_json::to_string(&state).unwrap_or(String::new());

    Ok(state)
}

#[tauri::command]
pub async fn stop_scan(
    engine: State<'_, Arc<UmEngine>>,
) -> Result<(), ()> {  

    let engine = Arc::clone(&engine);

    engine.scanner_cancel_scan();

    Ok(())
}


#[tauri::command]
pub async fn start_folder_scan(
    file_path: String,
    engine: State<'_, Arc<UmEngine>>,
	app_handle: tauri::AppHandle,
) -> Result<String, ()> {

	let engine = Arc::clone(&engine);
    let path = PathBuf::from(file_path);

	tokio::spawn(async move {
        // The result is wrapped inside of an enum from the filescanner module, so we need to first match on that
        // as DirectoryResult (since we are scanning a dir). The result should never be anything else for this scan
        // so if it is something has gone wrong with the internal wiring.
		match engine.scanner_start_scan(path) {

            um_engine::State::Finished(v) => {
                if v.scan_results.is_empty() {
                    app_handle.emit("folder_scan_no_results", "No malicious files found.").unwrap();
                } else {
                    app_handle.emit("folder_scan_malware_found", &v).unwrap();
                }
            },
            um_engine::State::FinishedWithError(v) => {
                app_handle.emit("folder_scan_error", &v).unwrap();
            },
            um_engine::State::Scanning(_) => {
                app_handle.emit("folder_scan_error", format!("A scan is already in progress.")).unwrap()
            },
            _ => (),
		}
	});

	// // todo some kind of feedback like 1/1 file scanned; but then same for the mass scanner, be good to show x files scanned, and time taken so far. Then completed time and 
	// // total files after.

	// todo this shouldn't show in every case..
	Ok(format!("Scan started..."))
}