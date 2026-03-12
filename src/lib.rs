use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ClickType {
    #[default]
    Left,
    Right,
    Middle,
}

// Core AutoClicker implementation using the enigo crate.
pub struct AutoClicker {
    pub delay_ms: u64,
    pub click_type: ClickType,
    handle: Option<thread::JoinHandle<()>>,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl AutoClicker {
    pub fn new(delay_ms: u64, click_type: ClickType) -> Self {
        Self {
            delay_ms,
            click_type,
            handle: None,
            stop_tx: None,
        }
    }

    /// Starts the autoclicker thread.
    /// Returns an error if the enigo instance fails to initialize.
    pub fn start(&mut self) -> Result<(), String> {
        // Stop any existing autoclicker
        self.stop();

        let delay = Duration::from_millis(self.delay_ms);
        let click_type = self.click_type.clone();

        // Create a channel for stopping the thread
        let (stop_tx, stop_rx) = mpsc::channel();
        self.stop_tx = Some(stop_tx);

        // Pre-check enigo initialization to catch errors early
        if let Err(e) = Enigo::new(&Settings::default()) {
            return Err(format!("Failed to initialize Enigo: {}", e));
        }

        self.handle = Some(thread::spawn(move || {
            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("AutoClicker thread failed to initialize Enigo: {}", e);
                    return;
                }
            };

            println!(
                "AutoClicker started with delay {}ms, click type {:?}",
                delay.as_millis(),
                click_type
            );

            loop {
                // Check if we should stop
                if stop_rx.try_recv().is_ok() {
                    println!("AutoClicker stopped");
                    break;
                }

                // Perform the actual mouse click
                let result = match click_type {
                    ClickType::Left => enigo.button(Button::Left, Direction::Click),
                    ClickType::Right => enigo.button(Button::Right, Direction::Click),
                    ClickType::Middle => enigo.button(Button::Middle, Direction::Click),
                };

                if let Err(e) = result {
                    eprintln!("AutoClicker failed to click: {}", e);
                    break;
                }

                thread::sleep(delay);
            }
        }));

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(()); // Send stop signal
        }

        if let Some(handle) = self.handle.take() {
            let _ = handle.join(); // Wait for thread to finish
        }
    }
}

impl Drop for AutoClicker {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autoclicker_creation() {
        let autoclicker = AutoClicker::new(1000, ClickType::Left);
        assert_eq!(autoclicker.delay_ms, 1000);
        assert_eq!(autoclicker.click_type, ClickType::Left);
    }

    #[test]
    fn test_click_types() {
        assert_eq!(ClickType::Left, ClickType::Left);
        assert_eq!(ClickType::Right, ClickType::Right);
        assert_eq!(ClickType::Middle, ClickType::Middle);

        // Test default
        let default_click: ClickType = Default::default();
        assert_eq!(default_click, ClickType::Left);
    }
}
