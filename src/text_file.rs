// Import necessary modules from the standard library and external crates
use std::fs::{File, write};
use serde::{Serialize, Deserialize};

/// Represents a note item with an ID and content
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteItem {
    pub id: i64,
    pub content: String,
}

/// Converts a NoteItem to a file
///
/// # Arguments
///
/// * `note` - The NoteItem to be converted
///
/// # Returns
///
/// * `File` - The created file
pub fn convert(note: NoteItem) -> File {
    // Create a new file named "note.txt"
    let file = File::create("note.txt").unwrap();
    
    // Commented out JSON serialization
    // serde_json::to_writer(file, ¬e.content).unwrap();
    
    // Write the note content to the file
    write("note.txt", note.content).unwrap();

    file
}