use serde::{Deserialize, Serialize};
use std::{
    fs::{self, metadata, Metadata},
    path::Path,
};

#[warn(private_interfaces)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    name: String,
    size: u64,
    ext: Option<String>,
    is_file: bool,
    is_dir: bool,
    file_type: String,
    formatted_size: String,
    processing_result: Option<String>,
    modified: u64,
    created: u64,
    accessed: u64,
}

impl FileInfo {
    fn new(path: String, metadata: Metadata) -> Self {
        let path_obj = Path::new(&path);
        let name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let ext = path_obj
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string());

        let size = metadata.len();
        let is_file = metadata.is_file();
        let is_dir = metadata.is_dir();

        let formatted_size = Self::calculate_formatted_size(size);

        let file_type = Self::calculate_file_type(is_dir, &ext);

        let modified = metadata
            .modified()
            .map(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        let created = metadata
            .created()
            .map(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        let accessed = metadata
            .accessed()
            .map(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        Self {
            path,
            name,
            size,
            ext,
            is_file,
            is_dir,
            file_type,
            formatted_size,
            processing_result: None,
            modified,
            created,
            accessed,
        }
    }

    // Setter for processing result
    fn set_processing_result(&mut self, result: String) {
        self.processing_result = Some(result);
    }

    // calculate formatted size (static method)
    fn calculate_formatted_size(size: u64) -> String {
        let size = size as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.2} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }

    // calculate file type based on extension and whether it's a directory
    fn calculate_file_type(is_dir: bool, ext: &Option<String>) -> String {
        if is_dir {
            "Directory".to_string()
        } else if let Some(ext) = ext {
            match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => "Image".to_string(),
                "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" => "Video".to_string(),
                "mp3" | "wav" | "flac" | "aac" | "ogg" => "Audio".to_string(),
                "txt" | "md" | "log" => "Text".to_string(),
                "pdf" => "PDF Document".to_string(),
                "doc" | "docx" => "Word Document".to_string(),
                "xls" | "xlsx" => "Excel Document".to_string(),
                "zip" | "rar" | "7z" | "tar" | "gz" => "Archive".to_string(),
                _ => format!("{} File", ext.to_uppercase()),
            }
        } else {
            "Unknown".to_string()
        }
    }

    // check if the file is an image
    fn is_image(&self) -> bool {
        if let Some(ext) = &self.ext {
            matches!(
                ext.to_lowercase().as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"
            )
        } else {
            false
        }
    }

    // check if the file is a text file
    fn is_text(&self) -> bool {
        if let Some(ext) = &self.ext {
            matches!(
                ext.to_lowercase().as_str(),
                "txt"
                    | "md"
                    | "json"
                    | "xml"
                    | "html"
                    | "css"
                    | "js"
                    | "ts"
                    | "rs"
                    | "py"
                    | "java"
                    | "c"
                    | "cpp"
                    | "h"
                    | "go"
            )
        } else {
            false
        }
    }
}

#[tauri::command]
pub fn handle_dropfile(path: String) -> Result<FileInfo, String> {
    // Check if a file exists
    if !Path::new(&path).exists() {
        return Err(format!("File does not exist: {}", path));
    }

    // Get file metadata
    let metadata =
        metadata(&path).map_err(|e| format!("Failed to get metadata for file: {}", e))?;

    // Create FileInfo instance
    let mut info = FileInfo::new(path.clone(), metadata);

    // Different processing according to file type
    let processing_result = if info.is_image() {
        process_image_file(&info)
    } else if info.is_text() {
        process_text_file(&info)
    } else {
        process_generic_file(&info)
    };

    // Set the processing result
    match processing_result {
        Ok(msg) => {
            info.set_processing_result(msg);
            Ok(info)
        }
        Err(e) => Err(format!("Failed to process file: {}", e)),
    }
}

// handler for image files
fn process_image_file(info: &FileInfo) -> Result<String, String> {
    println!("Processing image file: {}", info.name);

    // You can add image processing logic here, for example:
    // - Read image size
    // - Compress image
    // - Convert format, etc.
    Ok(format!("Image file '{}' analyzed", info.name))
}

// handler for text files
fn process_text_file(info: &FileInfo) -> Result<String, String> {
    println!("Processing text file: {}", info.name);

    // Read the content of the text file
    let content =
        fs::read_to_string(&info.path).map_err(|e| format!("Failed to read text file: {}", e))?;

    let line_count = content.lines().count();
    let char_count = content.chars().count();

    Ok(format!(
        "Text file analyzed - Lines: {}, Characters: {}",
        line_count, char_count
    ))
}

// handler for generic files
fn process_generic_file(info: &FileInfo) -> Result<String, String> {
    println!("Processing generic file: {}", info.name);

    // For common files, only basic information is read
    if info.is_file {
        Ok(format!("File '{}' information retrieved", info.name))
    } else if info.is_dir {
        // If it's a directory, list the contents
        let entries =
            fs::read_dir(&info.path).map_err(|e| format!("Failed to read directory: {}", e))?;

        let count = entries.count();
        Ok(format!("Directory contains {} items", count))
    } else {
        Ok("Unknown file type processed".to_string())
    }
}
