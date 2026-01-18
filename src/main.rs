use fancy_regex::Regex;
use image::ImageFormat;
use rayon::prelude::*;
use serde::Serialize;
use serde_json;
use std::env;
use std::io::Cursor;
use std::process::Command;

fn main() {
    // :ook through all arguments to see if any match "--shrink-thumbnails" for image thumbnail downscaling
    let args: Vec<String> = env::args().collect();
    let shrink_thumbnails = args.contains(&String::from("--shrink-thumbnails"));

    // Execute 'clipvault list' and capture its output
    let output = Command::new("clipvault")
        .arg("list")
        .output()
        .expect("Failed to execute 'clipvault list'");

    // Convert the stdout (in bytes) to a string
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Construct and pre-compile regex, which is thread-safe
    let pattern = r"^\d*\s*\[\[ binary data.*\]\]$";
    let re = Regex::new(pattern).unwrap();

    // Take the first 100 lines and collect them into a Vector so that Rayon can split work across threads
    // It does require an iterator (which requires a collection) for that, however
    let lines: Vec<&str> = stdout.lines().take(100).collect();

    // Use rayon to parallelize processing of lines retrieved from clipvault get
    let data: Vec<PipeData> = lines
        .into_par_iter()
        .map(|line| {
            let mut title = None;
            let mut binary: Option<Vec<u8>> = None;
            let mut result = None;

            if let Some((id, t)) = line.split_once("\t") {
                title = Some(t.to_string());
                result = Some(id.to_string());

                if re.is_match(line).unwrap_or(false) {
                    // This command now runs in a separate thread thanks to rayon
                    let decoded_output = Command::new("clipvault")
                        .arg("get")
                        .arg(id)
                        .output()
                        .expect("Failed to execute 'clipvault get'");

                    if shrink_thumbnails {
                        if let Ok(img) = image::load_from_memory(&decoded_output.stdout) {
                            // Resize to 100x100 (thumbnail)
                            let thumbnail = img.thumbnail(100, 100);

                            let mut thumb_bytes: Vec<u8> = Vec::new();
                            let mut cursor = Cursor::new(&mut thumb_bytes);

                            // Write the smaller image back to bytes in PNG format
                            thumbnail.write_to(&mut cursor, ImageFormat::Png).ok();

                            binary = Some(thumb_bytes);
                        } else {
                            // Fallback to original decoded output if image fails to load
                            binary = Some(decoded_output.stdout);
                        }
                    } else {
                        // Use original decoded output, do not shrink image thumbnail
                        binary = Some(decoded_output.stdout);
                    }
                }
            }

            PipeData {
                title,
                result,
                binary,
                icon_size: Some(100),
            }
        })
        .collect(); // rayon collects the results back in the correct order

    // Serialize to json and print
    let res = PipeResult::new(data);

    let json = serde_json::to_string(&res).expect("failed to serialize");
    print!("{}", json);
}

#[derive(Debug, Serialize, Clone)]
pub struct PipeData {
    pub title: Option<String>,
    pub result: Option<String>,
    pub binary: Option<Vec<u8>>,
    pub icon_size: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PipeResult {
    settings: Vec<String>,
    elements: Vec<PipeData>,
}

impl PipeResult {
    fn new(data: Vec<PipeData>) -> Self {
        Self {
            settings: Vec::new(),
            elements: data,
        }
    }
}
