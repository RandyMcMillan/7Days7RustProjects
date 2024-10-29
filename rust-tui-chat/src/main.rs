mod ui;
mod network;
mod message;

use anyhow::Result;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ui = ui::Ui::new()?;
    let (tx, rx) = mpsc::channel();
    let mut network = network::Network::new("127.0.0.1:8080").await?;

    let handle = thread::spawn(move || {
        let mut input = String::new();
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Enter => {
                        let _ = tx.send(input.clone());
                        input.clear();
                    },
                    KeyCode::Backspace => { input.pop(); },
                    KeyCode::Esc => break,
                    _ => {},
                }
            }
        }
    });

    let mut messages = Vec::new();
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                network.send_message(&msg).await?;
                messages.push(msg);
            },
            Err(_) => {
                if let Ok(msg) = network.receive_message().await {
                    messages.push(msg);
                }
            },
        }
        ui.draw(&input, &messages)?;
        thread::sleep(Duration::from_millis(100));
    }

    ui.cleanup()?;
    handle.join().unwrap();
    Ok(())
}