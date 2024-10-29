### **Day 6: Rust TUI Chat Application - Mastering Terminal User Interfaces**

#### Overview
Today, we're going to build a chat application using Rust's terminal user interface (TUI) libraries. This application will allow two instances to communicate over a network, simulating a simple chat. We'll use `tui` for the interface and `tokio` for asynchronous networking.

#### Difficulty
🌲 **Advanced**

#### Prerequisites
- Intermediate Rust knowledge
- Familiarity with asynchronous programming
- Basic understanding of networking in Rust

#### Project Structure

First, let's set up our project:

```sh
mkdir rust-tui-chat
cd rust-tui-chat
cargo init --bin
```

Now, let’s define our folder structure:

```
rust-tui-chat/
│
├── src/
│   ├── main.rs
│   ├── ui.rs
│   ├── network.rs
│   └── message.rs
│
├── Cargo.toml
└── README.md
```

#### Step 1: Setting up `Cargo.toml`

```toml
[package]
name = "rust_tui_chat"
version = "0.1.0"
edition = "2018"

[dependencies]
crossterm = "0.20"
tokio = { version = "1", features = ["full"] }
tokio-util = { version = "0.6", features = ["codec"] }
tui = { version = "0.16", default-features = false, features = ['crossterm'] }
anyhow = "1.0"
```

#### Step 2: `ui.rs` - Terminal User Interface

```rust
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self, anyhow::Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, input: &str, messages: &[String]) -> Result<(), anyhow::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let messages_block = Block::default().title("Chat").borders(Borders::ALL);
            let input_block = Block::default().title("Input").borders(Borders::ALL);
            f.render_widget(messages_block, chunks[0]);
            f.render_widget(Paragraph::new(input), chunks[1]);
        })?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), anyhow::Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
```

#### Step 3: `network.rs` - Network Communication

```rust
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct Network {
    stream: Option<TcpStream>,
}

impl Network {
    pub async fn new(addr: &str) -> Result<Self, anyhow::Error> {
        let stream = if addr.contains(":") {
            TcpStream::connect(addr).await?
        } else {
            let listener = TcpListener::bind(addr).await?;
            let (stream, _) = listener.accept().await?;
            stream
        };
        Ok(Self { stream: Some(stream) })
    }

    pub async fn send_message(&mut self, msg: &str) -> Result<(), anyhow::Error> {
        if let Some(stream) = self.stream.as_mut() {
            stream.write_all(&[msg.len() as u8]).await?;
            stream.write_all(msg.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<String, anyhow::Error> {
        if let Some(stream) = self.stream.as_mut() {
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).await?;
            let mut buffer = vec![0; len[0] as usize];
            stream.read_exact(&mut buffer).await?;
            return String::from_utf8(buffer).map_err(|e| e.into());
        }
        Err(anyhow::anyhow!("No stream available"))
    }
}
```

#### Step 4: `message.rs` - Message Handling

```rust
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(text: String) -> Self {
        Message { text }
    }
}
```

#### Step 5: `main.rs` - Main Application Logic

```rust
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
```

#### Step 6: Running Your Chat App

To run the application:

```sh
cargo run
```

#### Explanation

- **UI**: We've used `tui` for creating a clean, interactive terminal interface where users can see messages and type their own.
- **Networking**: Async networking with `tokio` allows for sending and receiving messages over TCP. Here, you can run one instance as the server and another as the client locally or over a network.
- **Message Handling**: Simple struct to manage messages in our application.

#### Conclusion

This project is an advanced step into Rust programming, combining TUI, networking, and asynchronous programming. It opens the door to further enhancements like:

- Adding user authentication.
- Implementing chat rooms or private messaging.
- Enhancing the UI with more widgets or color.

This chat application serves as a solid foundation for experimenting with terminal applications and network communication in Rust.