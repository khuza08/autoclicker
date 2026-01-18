use autoclicker_lib::{AutoClicker, ClickType};
use eframe::egui;

#[derive(Default)]
struct AutoClickerApp {
    delay_ms: u64,
    click_type: ClickType,
    is_running: bool,
    autoclicker: Option<AutoClicker>,
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
                    } else {
                        // Start the autoclicker
                        self.autoclicker =
                            Some(AutoClicker::new(self.delay_ms, self.click_type.clone()));
                        if let Some(ref mut autoclicker) = self.autoclicker {
                            autoclicker.start();
                        }
                        self.is_running = true;
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
            }

            ui.separator();
            ui.label("Note: Actual mouse clicks require system dependencies to be installed.");
            ui.label("See README.md for installation instructions.");
        });

        // Refresh UI periodically
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
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
    )
}

