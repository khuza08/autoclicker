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

// Mock AutoClicker implementation for demonstration purposes
// In a real implementation, this would use enigo for actual mouse clicks
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

    pub fn start(&mut self) {
        // Stop any existing autoclicker
        self.stop();

        let delay = Duration::from_millis(self.delay_ms);
        let click_type = self.click_type.clone();

        // Create a channel for stopping the thread
        let (stop_tx, stop_rx) = mpsc::channel();
        self.stop_tx = Some(stop_tx);

        self.handle = Some(thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
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
                match click_type {
                    ClickType::Left => enigo.button(Button::Left, Direction::Click).unwrap(),
                    ClickType::Right => enigo.button(Button::Right, Direction::Click).unwrap(),
                    ClickType::Middle => enigo.button(Button::Middle, Direction::Click).unwrap(),
                }

                thread::sleep(delay);
            }
        }));
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
