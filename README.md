# 🧮 Calculator Project (Slint + Rust)
Introduction
This is a basic calculator application project developed using the Rust language and the Slint User Interface framework. This project illustrates the integration between the Slint UI code (in the .slint file) and the business logic implemented in Rust.

## 🛠️ How to Build and Run the Application
To run this application, you need to install Rust and use the Cargo project management tool.

1. Prerequisites
   - Rust: Install Rust following the [getting-started guide].
   - Operating System: Supports Windows, macOS, Linux (or platforms supported by Slint).
   
2. Execution Steps
cargo build => Compiles the entire project, including both Rust code and Slint UI code, creating the executable binary in the target/debug/ directory.
cargo run => Compiles (if necessary) and runs the application directly. This is the fastest way to start the application.

## 🏗️ Architecture Overview

The project uses a clear separation of concerns between the user interface and the business logic.
1. User Interface (Frontend - .slint):
   - The main interface file is ui/app-window.slint (or similar).
   - Uses the Slint Markup Language syntax to define the layout, widgets (buttons, display screen), and callbacks (functions invoked upon events, e.g., button press).
     
2. Business Logic (Backend - .rs):
   - The Rust code in src/main.rs contains the core calculation logic.
   - Rust is responsible for initializing the Slint interface, binding the callbacks (such as handle_button_press) defined in the .slint file to the corresponding Rust functions, and updating the properties on the UI (e.g., displaying the result) after calculation.

Don't forget to edit this readme to replace it by yours, and edit the `name =` field in `Cargo.toml` to match the name of yourproject.
## 🖼️ Screenshot of the app
<img width="317" height="482" alt="image" src="https://github.com/user-attachments/assets/f7a92540-ddaa-4fe9-bb2b-5d122c9493e1" />



