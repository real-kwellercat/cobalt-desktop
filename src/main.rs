use eframe::{egui, *};
use egui::{CentralPanel, RichText, FontId};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::prelude::*;

struct MyApp {}

lazy_static! {
    static ref DOWNLOAD_ME: Mutex<String> = Mutex::new(String::from("https://youtu.be/klqi_h9FElc"));
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |_ui| {
            _ui.label(RichText::new("Download a video using the Cobalt API!").font(FontId::proportional(28.0)));
            _ui.label(RichText::new("App may freeze when downloading.").font(FontId::proportional(18.0)));
            
            {
                let mut downloadme = DOWNLOAD_ME.lock().unwrap();
                _ui.add(egui::TextEdit::singleline(&mut *downloadme));
            }
            
            if _ui.button("Download video").on_hover_text("Download the video from the provided URL").clicked {
                let downloadme = DOWNLOAD_ME.lock().unwrap();
                if downloadme.is_empty() {
                    eprintln!("URL is empty. Please enter a URL.");
                } else {
                    get_vid(&*downloadme);
                }
            };
        });
    }
}

fn get_vid(_url: &str) {

    let client = Client::new();
    let endpoint = "https://co.wuk.sh/api/json";

    // Construct the JSON object with the URL
    let json_body = json!({ "url": _url });

    // Serialize the JSON object to a string
    let json_string = serde_json::to_string(&json_body).expect("Failed to serialize JSON");

    // Make a POST request to the API endpoint
    let response = client.post(endpoint)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(json_string)
        .send()
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        // Extract JSON data from the response
        let json: Value = response.json().unwrap();

        // Assume you have a request body with a filenamePattern field
        let _filename_pattern = json["filenamePattern"].as_str().unwrap_or("basic");

        // Get the URL from the JSON response
        if let Some(url) = json.get("url").and_then(Value::as_str) {
            // Download the content from the provided URL
            match reqwest::blocking::get(url) {
                Ok(mut downloaded_content) => {
                    let mut rng = rand::thread_rng();
                    let mut nums: Vec<u32> = (1..10000).collect();
                    nums.shuffle(&mut rng);
                    let random_number = nums[0]; // Get the first number from the shuffled vector

                    let filename = format!("video-{}.mp4", random_number);
                    match File::create(&filename) {
                        Ok(mut file) => {
                            let mut buffer = Vec::new();
                            match downloaded_content.copy_to(&mut buffer) {
                                Ok(_) => {
                                    match file.write_all(&buffer) {
                                        Ok(_) => {
                                            println!("Downloaded video successfully");
                                        },
                                        Err(e) => {
                                            println!("Failed to write content to file: {}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to read content: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to create file '{}': {}", filename, e);
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to download video content: {}", e);
                }
            }
        } else {
            println!("No 'url' field found in the response JSON");
        }
    } else {
        // Handle the case where the request was not successful
        println!("Request was not successful: {}", response.status());
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    run_native("Cobalt-Desktop", native_options, Box::new(|_| Box::new(MyApp {})))
}