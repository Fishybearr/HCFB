// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::path::Path;

//serializing library for file struct
use serde::Serialize;



#[derive(Serialize)]
pub struct FileObject
{
    path: String,
    file_type: bool,
    file_name: String
    
}

#[tauri::command]
async fn read_home_dir() -> Result<String,String>
{
    match dirs::home_dir() 
    {
        Some(d) => Ok(d.to_string_lossy().to_string()),
        None => Err("Could not open dir".to_string())
    }
}


#[tauri::command]
async fn get_parent_dir(path:String) -> Result<String,String>
{
    let p = Path::new(&path);

    match p.parent()
    {
        Some(parent) => Ok(parent.to_string_lossy().to_string()),
        None => Err("Could not find parent".to_string())
    }
}


#[tauri::command]
async fn read_files_from_path(path:String) -> Result<Vec<FileObject>,String>
{
    let mut entries = tokio::fs::read_dir(path).await.map_err(|e| e.to_string())?;

    let mut dir_list:Vec<FileObject> = Vec::new();

    
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())?
    {
        //create an file object for each file in the directory
        // and add it to the list

        //attempt to get the the file type of the current file
        let ft = entry.file_type().await.map_err(|e| e.to_string())?; 
        let f_name = entry.file_name().display().to_string();


        let file_o = FileObject
        {
            path: entry.path().display().to_string(),
            file_type: ft.is_dir(),
            file_name: f_name
        };

        dir_list.push(file_o)
    }
    

    Ok(dir_list)
}
    

/* 
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_files_from_path,read_home_dir,get_parent_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
