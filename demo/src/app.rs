use crate::clap::syntax;
use anyhow::Result;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//use egui_console::console::{ConsoleBuilder, ConsoleEvent, ConsoleWindow};
use egui_console::{ConsoleBuilder, ConsoleEvent, ConsoleWindow};
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
//#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct ConsoleDemo {
    // Example stuff:
    label: String,
    #[cfg_attr(feature = "persistence", serde(skip))]
    // This how you opt-out of serialization of a field
    value: f32,
    console_win: ConsoleWindow,
}

impl Default for ConsoleDemo {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            console_win: ConsoleBuilder::new()
                .prompt(">> ")
                .history_size(20)
                .tab_quote_character('\"')
                .build(),
        }
    }
}

impl ConsoleDemo {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };
        for cmd in syntax().get_subcommands() {
            app.console_win
                .command_table_mut()
                .push(cmd.get_name().to_string());
        }

        app
    }
}

impl eframe::App for ConsoleDemo {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        #[cfg(feature = "persistence")]
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
                let is_web = cfg!(target_arch = "wasm32") | true;
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        // egui::SidePanel::left("left_panel")
        //     .resizable(true)
        //     .default_width(150.0)
        //     .width_range(80.0..=200.0)
        //     .show(ctx, |ui| {
        //         ui.vertical_centered(|ui| {
        //             ui.heading("Left Panel");
        //         });
        //     });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut console_response: ConsoleEvent = ConsoleEvent::None;
            egui::Window::new("Console Window")
                .default_height(500.0)
                .resizable(true)
                .show(ctx, |ui| {
                    console_response = self.console_win.draw(ui);
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
                if !resp.is_empty() {
                    self.console_win.write(&resp);
                }
                self.console_win.prompt();
            }

            if ui.button("click for console output").clicked() {
                self.console_win.write("clicked");
                self.console_win.prompt();
            }
        });
    }
}
impl ConsoleDemo {
    pub fn dispatch(&mut self, line: &str, ctx: &egui::Context) -> Result<String> {
        // let args = line.split_whitespace();
        let args = shlex::split(line).ok_or(anyhow::anyhow!("cannot parse"))?;
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
            Some(("clear_screen", _)) => {
                self.console_win.clear();
                Ok("".to_string())
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
            Some(("history", _)) => {
                let history = self.console_win.get_history();
                let mut result = String::new();
                for (i, line) in history.iter().enumerate() {
                    result.push_str(&format!("{}: {}\n", i, line));
                }
                Ok(result)
            }
            Some(("clear_history", _)) => {
                self.console_win.clear_history();
                Ok("".to_string())
            }
            _ => Ok("Unknown command".to_string()),
        }
    }
}
