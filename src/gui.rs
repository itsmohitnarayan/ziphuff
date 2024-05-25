// src/gui.rs

use eframe::egui::{self, ProgressBar};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;
use rfd::FileDialog;
use std::time::{Duration, Instant};

use crate::{compression, freqs};

pub struct HuffmanApp {
    input: String,
    output: String,
    action: Action,
    mode: Mode,
    log: Arc<Mutex<String>>,
    progress: Arc<Mutex<f32>>,
    elapsed_time: Arc<Mutex<Duration>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Compress,
    Extract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Words,
    Chars,
}

impl Default for HuffmanApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            action: Action::Compress,
            mode: Mode::Words,
            log: Arc::new(Mutex::new(String::new())),
            progress: Arc::new(Mutex::new(0.0)),
            elapsed_time: Arc::new(Mutex::new(Duration::new(0, 0))),
        }
    }
}

impl eframe::App for HuffmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zip Huff");

            ui.horizontal(|ui| {
                ui.label("Input file:");
                if ui.button("Browse..").clicked(){
                    if let Some(path) = FileDialog::new().pick_file(){
                    self.input = path.display().to_string();
                    }
                }
                ui.text_edit_singleline(&mut self.input);
            });

            ui.horizontal(|ui| {
                ui.label("Output file:");
                ui.text_edit_singleline(&mut self.output);
            });

            ui.horizontal(|ui| {
                ui.label("Action:");
                ui.radio_value(&mut self.action, Action::Compress, "Compress");
                ui.radio_value(&mut self.action, Action::Extract, "Extract");
            });

            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.radio_value(&mut self.mode, Mode::Words, "Words");
                ui.radio_value(&mut self.mode, Mode::Chars, "Chars");
            });

            if ui.button("Run").clicked() {
                let input_path = self.input.clone();
                let output_path = self.output.clone();
                let action = self.action;
                let mode = self.mode;
                let log = self.log.clone();
                let progress = self.progress.clone();
                let elapsed_time = self.elapsed_time.clone();
                let _ = std::thread::spawn(move || {
                    let start_time = Instant::now();
                    let result = run_huffman(input_path, output_path, action, mode, &progress);
                    let mut log = log.lock().unwrap();
                    *log = result;
                    *elapsed_time.lock().unwrap() = start_time.elapsed();
                });
            }
            ui.add(egui::ProgressBar::new(*self.progress.lock().unwrap()));

            if *self.progress.lock().unwrap() >= 1.0 {
                ui.label(format!(
                    "Elapsed time: {:.2?}",
                    *self.elapsed_time.lock().unwrap()
                ));
            }

            ui.separator();

            ui.label("Log:");
            let log = self.log.lock().unwrap();
            ui.label(log.as_str());
        });
    }
}

fn run_huffman(input_path: String, output_path: String, action: Action, mode: Mode, progress: &Arc<Mutex<f32>>,) -> String {
    let result = match action {
        Action::Compress => {
            let text = std::fs::read_to_string(&input_path);
            if let Err(err) = text {
                return format!("Failed to read input file: {}", err);
            }
            let lines: Vec<_> = text.unwrap().split('\n').map(|x| x.to_string()).collect();

            let compressed = match mode {
                Mode::Words => compression::compress(&lines, freqs::word_frequencies, |line| {
                    line.split_ascii_whitespace().map(|token| token.to_string())
                }),
                Mode::Chars => {
                    compression::compress(&lines, freqs::char_frequencies, |line| line.chars())
                }
            };
            if let Err(err) = compressed {
                return format!("Compression failed: {}", err);
            }
            let compressed = compressed.unwrap();

            let mut out_f = File::create(&output_path);
            if let Err(err) = out_f {
                return format!("Failed to create output file: {}", err);
            }
            if let Err(err) = out_f.unwrap().write(&compressed) {
                return format!("Failed to write to output file: {}", err);
            }
            *progress.lock().unwrap() = 1.0; // Indicate completion
            "Extraction succeeded".to_string()
        }
        Action::Extract => {
            let data = std::fs::read(&input_path);
            if let Err(err) = data {
                return format!("Failed to read input file: {}", err);
            }
            let data = data.unwrap();

            let content = match mode {
                Mode::Words => compression::extract(&data, |tokens: Vec<String>| tokens.join(" ")),
                Mode::Chars => compression::extract(&data, |tokens: Vec<char>| tokens.into_iter().collect()),
            };
            if let Err(err) = content {
                return format!("Extraction failed: {}", err);
            }
            let content = content.unwrap();

            if let Err(err) = std::fs::write(&output_path, content.join("\n")) {
                return format!("Failed to write to output file: {}", err);
            }
            *progress.lock().unwrap() = 1.0; // Indicate completion
            "Extraction succeeded".to_string()
        }
    };
    result
}
