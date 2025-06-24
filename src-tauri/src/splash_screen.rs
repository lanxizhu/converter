use super::window_state;
use std::sync::Mutex;
use tauri::{async_runtime::spawn, App, AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

// Create a struct we'll use to track the completion of
// setup related tasks
pub struct SetupState {
    pub frontend_task: bool,
    pub backend_task: bool,
}

// A custom task for setting the state of a setup task
#[tauri::command]
pub async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<String, ()> {
    // Lock the state without write access
    let mut state_lock = state.lock().unwrap();

    if state_lock.frontend_task && state_lock.backend_task {
        return Ok("Both tasks are already completed!".to_string());
    }

    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // Check if both tasks are completed
    if state_lock.backend_task && state_lock.frontend_task {
        window_state::restore(app.clone());
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok("Task completed!".to_string())
}

// An async function that does some heavy setup task
async fn setup(app: AppHandle) -> Result<(), ()> {
    // Fake performing some heavy action for 3 seconds
    println!("Performing really heavy backend setup task...");
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // Set the backend task as being completed
    // Commands can be ran as regular functions as long as you take
    // care of the input arguments yourself
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

pub fn init(app: &mut App) -> Result<(), ()> {
    spawn(setup(app.handle().clone()));

    Ok(())
}
