extern crate serde;
extern crate serde_json;

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};

const CONFIG_FILE: &str = "config.tskbr";

#[derive(Serialize, Deserialize)]
struct AppList {
  programs: Vec<String>,
}

impl AppList {
  fn load_from_file() -> Self {
    let data = fs::read_to_string(CONFIG_FILE).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or(AppList { programs: Vec::new() })
  }

  fn save_to_file(&self) {
    let data = serde_json::to_string_pretty(self).expect("Failed to serialize data :'(");
    fs::write(CONFIG_FILE, data).expect("Failed to write to configuration file! :(");
  }

  pub fn add_program(&mut self, program: String) {
    self.programs.push(program);
    self.save_to_file();
  }
}

struct Tskbr {
  app_list: Arc<Mutex<AppList>>,
  program_name: String,
}

impl Default for Tskbr {
  fn default() -> Self {
    let app_list = Arc::new(Mutex::new(AppList::load_from_file()));
    let program_name = String::new();
    if app_list.lock().unwrap().programs.is_empty() {
      let mut app_list = app_list.lock().unwrap();
      app_list.programs.push("Google Chrome".to_string());
      app_list.programs.push("Steam".to_string());
      app_list.programs.push("Discord".to_string());
      app_list.save_to_file();
    }
    Tskbr { app_list, program_name }
  }
}

impl eframe::App for Tskbr {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Tskbr");

      ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut self.program_name);
        if ui.button("Add").clicked() {
          let mut app_list = self.app_list.lock().unwrap();
          app_list.add_program(self.program_name.clone());
          self.program_name.clear();
        }
      });

      ui.separator();

      let programs = self.app_list.lock().unwrap().programs.clone();
      for program in programs {
        if ui.button(&program).clicked() {
          launch_program(&program);
        }
      }
    });
  }
}

fn main() -> std::io::Result<()> {
  let options = eframe::NativeOptions::default();
  eframe::run_native(
    "Tskbr",
    options,
    Box::new(|_cc| Ok(Box::new(Tskbr::default()))),
  ).expect("PANIK! Failed to run TskBr");
  Ok(())
}

fn launch_program(program: &str) {
  #[cfg(target_os = "macos")]
  std::process::Command::new("open")
    .arg("-a")
    .arg(program)
    .spawn()
    .expect("Failed to launch :(");

  #[cfg(target_os = "linux")]
  std::process::Command::new(program)
    .spawn()
    .expect("Failed to launch program");

  #[cfg(target_os = "windows")]
  std::process::Command::new("cmd")
    .args(&["/C", "start", program])
    .spawn()
    .expect("Failed to launch program");
}