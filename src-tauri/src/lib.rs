// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


use tokio::fs;


#[tauri::command]
async fn read_files_from_path() -> Result<Vec<String>,String>
{
    //TODO: Update this to read in a path from user
    let mut entries = tokio::fs::read_dir("./").await.map_err(|e| e.to_string())?;

    let mut dirList:Vec<String> = Vec::new();

    
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())?
    {
        let path = entry.path();
        dirList.push(path.display().to_string())
    }
    

    Ok(dirList)
}
    

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,read_files_from_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
