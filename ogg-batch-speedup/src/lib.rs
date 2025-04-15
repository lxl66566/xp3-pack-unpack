use log::{debug, error, info};
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

/// Process all audio files in the specified folder recursively with the given speed multiplier.
///
/// # Arguments
///
/// * `folder` - Path to the folder containing audio files
/// * `speed` - Speed multiplier (e.g., 1.5 for 1.5x speed)
///
/// # Returns
///
/// * `Result<()>` - Ok(()) if successful, or an error if processing fails
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use ogg_batch_speedup::process_audio_files;
///
/// let folder = Path::new("path/to/audio/files");
/// let speed = 1.5;
/// process_audio_files(folder, speed).unwrap();
/// ```
pub fn process_audio_files(folder: impl AsRef<Path>, speed: f32) -> std::io::Result<()> {
    let folder = folder.as_ref();

    // Collect all files that need to be processed
    let files: Vec<_> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    // Process all files in parallel
    files.par_iter().try_for_each(|entry| {
        let path = entry.path();
        if !path.is_file() {
            return Ok(());
        }
        // Check if file header indicates OGG file
        let mut file = File::open(path)?;
        let mut header = [0u8; 4];
        if let Err(e) = file.read_exact(&mut header) {
            error!("Error reading file header: {}", e);
            return Err(e);
        }
        if &header != b"OggS" {
            debug!("Skipping non-ogg file: {}", path.display());
            return Ok(());
        }

        let output_file = path.with_file_name(format!(
            "temp_{}",
            path.file_name().unwrap().to_str().unwrap()
        ));

        info!("Processing {}...", path.display());

        let status = Command::new("ffmpeg")
            .args([
                "-i",
                path.to_str().unwrap(),
                "-filter:a",
                &format!("atempo={}", speed),
                "-vn",
                output_file.to_str().unwrap(),
                "-y",
                "-loglevel",
                "error",
            ])
            .status();

        if let Err(e) = status {
            error!("Error processing {}: {}", path.display(), e);
            return Err(e);
        }

        if status.unwrap().success() {
            std::fs::rename(&output_file, path)?;
        } else {
            if output_file.exists() {
                std::fs::remove_file(output_file)?;
            }
            error!("Error processing {}", path.display());
        }
        Ok(())
    })
}
