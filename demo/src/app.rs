/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use egui_console::console::{ConsoleEvent, ConsoleWindow};

use crate::clap::syntax;
use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    //  #[serde(skip)] // This how you opt-out of serialization of a field
    console: ConsoleWindow,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            console: ConsoleWindow::new(">> "),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                //  egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut console_window_open = true;
            let mut console_response: ConsoleEvent = ConsoleEvent::None;
            egui::Window::new("Console Window")
                //.id(Id::new(self.name()))
                .open(&mut console_window_open)
                .vscroll(false)
                .default_height(500.0)
                .resizable(true)
                //  .auto_sized()
                .show(ctx, |ui| {
                    console_response = self.console.draw(ui);
                });
            if let ConsoleEvent::Command(command) = console_response {
                let resp = match self.dispatch(&command, ctx) {
                    Err(e) => {
                        if let Some(original_error) = e.downcast_ref::<clap::error::Error>() {
                            format!("{}", original_error)
                        } else if e.backtrace().status()
                            == std::backtrace::BacktraceStatus::Captured
                        {
                            format!("{} {}", e, e.backtrace())
                        } else {
                            format!("{}", e)
                        }
                    }

                    Ok(string) => string, // continue
                };
                self.console.sync_response(&resp);
            }

            if ui.button("click").clicked() {
                self.console.async_message("clicked");
            }
            // let console_response =
            // // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/main/",
            //     "Source code."
            // ));

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     powered_by_egui_and_eframe(ui);
            //     egui::warn_if_debug_build(ui);
            //});
        });
    }
}
impl TemplateApp {
    pub fn dispatch(&mut self, line: &str, ctx: &egui::Context) -> Result<String> {
        let args = line.split_whitespace();
        // parse with clap
        let matches = syntax().try_get_matches_from(args)?;
        // execute the command
        match matches.subcommand() {
            Some(("cd", args)) => {
                let dir = args.get_one::<String>("directory").unwrap();
                std::env::set_current_dir(dir)?;
                let cwd = std::env::current_dir()?;
                Ok(format!("Current working directory: {}", cwd.display()))
            }
            Some(("dark", _)) => {
                //  let ctx = egui::Context::default();
                ctx.set_visuals(egui::Visuals::dark());
                Ok("Dark mode enabled".to_string())
            }
            Some(("light", _)) => {
                //   let ctx = egui::Context::default();
                ctx.set_visuals(egui::Visuals::light());
                Ok("Light mode enabled".to_string())
            }
            Some(("quit", _)) => {
                //   let ctx = egui::Context::default();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                Ok("Bye".to_string())
            }
            Some(("dir", args)) => {
                let filter = if let Some(filter) = args.get_one::<String>("filter") {
                    filter.clone()
                } else {
                    "".to_string()
                };
                let entries = std::fs::read_dir(".")?;
                let mut result = String::new();
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.display().to_string().contains(filter.as_str()) {
                        result.push_str(&format!("{}\n", path.display()));
                    }
                }
                Ok(result)
            }

            _ => Ok("huh?".to_string()),
        }
    }
}
