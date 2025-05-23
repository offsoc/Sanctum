use crate::ipc::IpcClient;
use serde_json::{to_value, Value};
use shared_std::driver_manager::DriverState;

#[derive(serde::Serialize, serde::Deserialize)]
enum Response {
    Ok(String),
    Err(String),
}

/// Install the driver on the host machine
#[tauri::command]
pub async fn driver_install_driver() -> Result<String, ()> {
    let state = match IpcClient::send_ipc::<DriverState, Option<Value>>(
        "driver_install_driver",
        None,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[-] Error with IPC for install driver: {e}");
            DriverState::Uninstalled("An error occurred talking to the engine.".to_string())
        }
    };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}

/// Uninstall the driver on the host machine
#[tauri::command]
pub async fn driver_uninstall_driver() -> Result<String, ()> {
    let state =
        match IpcClient::send_ipc::<DriverState, Option<Value>>("driver_uninstall_driver", None)
            .await
        {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[-] Error with IPC for uninstall driver: {e}");
                DriverState::Uninstalled("An error occurred talking to the engine.".to_string())
            }
        };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}

#[tauri::command]
pub async fn driver_start_driver() -> Result<String, ()> {
    let state = match IpcClient::send_ipc::<DriverState, Option<Value>>("driver_start_driver", None)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[-] Error with IPC for start driver: {e}");
            DriverState::Uninstalled("An error occurred talking to the engine.".to_string())
        }
    };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}

#[tauri::command]
pub async fn driver_stop_driver() -> Result<String, ()> {
    let state =
        match IpcClient::send_ipc::<DriverState, Option<Value>>("driver_stop_driver", None).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[-] Error with IPC for stop driver: {e}");
                DriverState::Uninstalled("An error occurred talking to the engine.".to_string())
            }
        };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}

#[tauri::command]
pub async fn driver_check_state() -> Result<String, ()> {
    let state =
        match IpcClient::send_ipc::<DriverState, Option<Value>>("driver_get_state", None).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[-] Error with IPC for get driver state: {e}");
                DriverState::Uninstalled("An error occurred talking to the engine.".to_string())
            }
        };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}

#[tauri::command]
pub async fn ioctl_ping_driver() -> Result<String, ()> {
    let response =
        match IpcClient::send_ipc::<String, Option<Value>>("ioctl_ping_driver", None).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[-] Error with IPC for get driver state: {e}");
                "An error occurred when communicating  with the driver.".to_string()
            }
        };

    Ok(response)
}

/// Poll the usermode engine for any new messages from the kernel which need to be processed by the GUI; this is only
/// for the driver controller page
#[tauri::command]
pub async fn driver_get_kernel_debug_messages() -> Result<String, ()> {
    let state =
        match IpcClient::send_ipc::<Value, Option<Value>>("driver_collect_knl_dbg_msg", None).await
        {
            Ok(s) => {
                println!("[i] Received kernel msg: {}", s);
                s
            }
            Err(e) => {
                eprintln!("[-] Error with IPC for get driver state: {e}");
                to_value("").unwrap()
            }
        };

    let state_string = serde_json::to_string(&state).unwrap();

    Ok(state_string)
}
