use autoclicker_lib::{AutoClicker, ClickType};
use eframe::egui;
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Default)]
struct AutoClickerApp {
    delay_ms: u64,
    click_type: ClickType,
    is_running: bool,
    autoclicker: Option<AutoClicker>,
    error_message: Option<String>,
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AutoClicker");

            ui.horizontal(|ui| {
                ui.label("Delay (ms):");
                ui.add(egui::Slider::new(&mut self.delay_ms, 1..=5000).text("milliseconds"));
                ui.label(format!("{}", self.delay_ms));
            });

            ui.horizontal(|ui| {
                ui.label("Click type:");
                ui.radio_value(&mut self.click_type, ClickType::Left, "Left");
                ui.radio_value(&mut self.click_type, ClickType::Right, "Right");
                ui.radio_value(&mut self.click_type, ClickType::Middle, "Middle");
            });

            ui.separator();

            if let Some(ref msg) = self.error_message {
                ui.colored_label(egui::Color32::RED, msg);
                if ui.button("Clear Error").clicked() {
                    self.error_message = None;
                }
            }

            ui.horizontal(|ui| {
                if ui
                    .button(if self.is_running { "Stop" } else { "Start" })
                    .clicked()
                {
                    if self.is_running {
                        // Stop the autoclicker
                        if let Some(ref mut autoclicker) = self.autoclicker {
                            autoclicker.stop();
                        }
                        self.is_running = false;
                        self.error_message = None;
                    } else {
                        // Start the autoclicker
                        let mut ac = AutoClicker::new(self.delay_ms, self.click_type.clone());
                        match ac.start() {
                            Ok(_) => {
                                self.autoclicker = Some(ac);
                                self.is_running = true;
                                self.error_message = None;
                            }
                            Err(e) => {
                                self.error_message = Some(e);
                                self.is_running = false;
                            }
                        }
                    }
                }

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.separator();

            ui.label(format!(
                "Status: {}",
                if self.is_running {
                    "RUNNING"
                } else {
                    "STOPPED"
                }
            ));

            if self.is_running {
                ui.colored_label(egui::Color32::RED, "AUTOCLICKER IS ACTIVE - BE CAREFUL!");
                // Only request repaint if running to show the active status
                ctx.request_repaint_after(Duration::from_millis(500));
            }

            ui.separator();
            ui.label("Note: Actual mouse clicks require system dependencies to be installed.");
            ui.label("See README.md for installation instructions.");
        });
    }
}

fn run_cli() -> io::Result<()> {
    println!("AutoClicker CLI Mode");
    println!("-------------------");
    println!("Controls:");
    println!("  s - Start");
    println!("  t - Stop");
    println!("  q - Quit");
    println!("  + - Increase delay (100ms)");
    println!("  - - Decrease delay (100ms)");
    println!("-------------------");

    let mut delay_ms = 1000;
    let click_type = ClickType::Left;
    let mut autoclicker: Option<AutoClicker> = None;
    let mut is_running = false;

    enable_raw_mode()?;
    let mut stdout = io::stdout();

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        if !is_running {
                            let mut ac = AutoClicker::new(delay_ms, click_type.clone());
                            match ac.start() {
                                Ok(_) => {
                                    autoclicker = Some(ac);
                                    is_running = true;
                                    println!("\r\n[STARTED] Delay: {}ms", delay_ms);
                                }
                                Err(e) => {
                                    println!("\r\n[ERROR] {}", e);
                                }
                            }
                        }
                    }
                    KeyCode::Char('t') => {
                        if is_running {
                            if let Some(ref mut ac) = autoclicker {
                                ac.stop();
                            }
                            is_running = false;
                            println!("\r\n[STOPPED]");
                        }
                    }
                    KeyCode::Char('+') => {
                        delay_ms = (delay_ms + 100).min(10000);
                        println!("\r\nDelay set to: {}ms", delay_ms);
                    }
                    KeyCode::Char('-') => {
                        delay_ms = delay_ms.saturating_sub(100).max(10);
                        println!("\r\nDelay set to: {}ms", delay_ms);
                    }
                    _ => {}
                }
            }
        }
        
        // Print status indicator
        if is_running {
            print!(".");
            stdout.flush()?;
        }
    }

    if let Some(ref mut ac) = autoclicker {
        ac.stop();
    }
    disable_raw_mode()?;
    println!("\r\nExiting CLI Mode.");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.iter().any(|arg| arg == "--cli") {
        run_cli()?;
        return Ok(());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 350.0])
            .with_min_inner_size([400.0, 350.0])
            .with_title("AutoClicker"),
        ..Default::default()
    };

    eframe::run_native(
        "AutoClicker",
        options,
        Box::new(|_cc| Box::new(AutoClickerApp::default())),
    ).map_err(|e| e.into())
}
