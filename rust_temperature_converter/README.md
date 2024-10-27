## Introduction

Welcome to Day 2 of our Rust project journey! Today, we're stepping into the world of GUI applications with Rust. We'll create a Temperature Converter that allows users to convert between Celsius and Fahrenheit. We'll use `egui`, a simple yet powerful GUI library for Rust, for this project.

## Prerequisites

- Rust installed on your system (`rustup` and `cargo`)
- Basic knowledge of Rust syntax

## Project Setup

### Step 1: Create a New Rust Project

Open your terminal and run:

```sh
cargo new rust_temperature_converter --bin
cd rust_temperature_converter
```

### Step 2: Add Dependencies

Edit your `Cargo.toml` to include:

```toml
[dependencies]
eframe = "0.20.0"
egui = "0.20.0"
```

### Step 3: Project Structure

Your project should now look like this:

```
rust_temperature_converter/
├── Cargo.lock
├── Cargo.toml
└── src/
    └── main.rs
```

### Step 4: Implementing the Converter

Now, replace the content in `src/main.rs` with:

```rust
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Temperature Converter",
        options,
        Box::new(|_cc| Box::new(TempConverter::default())),
    )
}

struct TempConverter {
    celsius: f32,
    fahrenheit: f32,
}

impl Default for TempConverter {
    fn default() -> Self {
        Self {
            celsius: 0.0,
            fahrenheit: 32.0,
        }
    }
}

impl eframe::App for TempConverter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Temperature Converter");
            ui.separator();

            ui.label("Celsius:");
            ui.text_edit_singleline(&mut self.celsius.to_string());

            if ui.button("Convert to Fahrenheit").clicked() {
                self.fahrenheit = self.celsius * 9.0 / 5.0 + 32.0;
            }

            ui.label("Fahrenheit:");
            ui.text_edit_singleline(&mut self.fahrenheit.to_string());

            if ui.button("Convert to Celsius").clicked() {
                self.celsius = (self.fahrenheit - 32.0) * 5.0 / 9.0;
            }
        });
    }
}
```

## Explanation of the Code

- **Main Function**: Here we set up the application window using `eframe::NativeOptions`. The `run_native` function starts the application with our `TempConverter` struct as the app state.

- **TempConverter Struct**: This holds our state, namely `celsius` and `fahrenheit` temperatures. We implement `Default` for easy initialization.

- **App Implementation**: We implement the `eframe::App` trait for `TempConverter`. The `update` method is where the UI logic resides:
  - We use `CentralPanel` for our main window content.
  - `text_edit_singleline` allows users to input or see temperature values.
  - Buttons trigger temperature conversions using the formulas: 
    - Celsius to Fahrenheit: `(°C × 9/5) + 32`
    - Fahrenheit to Celsius: `(°F - 32) × 5/9`

## Running the Project

- Save your `main.rs` file.
- From the terminal, in the project directory, run:

```sh
cargo run
```

You should now see a window appear with input fields for Celsius and Fahrenheit temperatures and buttons to convert between them.

## Conclusion

Congratulations! You've now created a functional GUI in Rust. This project not only teaches GUI interaction but also basic temperature unit conversion. As you continue with Rust, consider exploring different GUI libraries or adding more features like Kelvin conversion or saving conversion history.

Keep coding, and see you tomorrow for Day 3! 🚀

---
