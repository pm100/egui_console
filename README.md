# A console window for egui
Provides a console window for egui. This is not a shell to the OS its simply a command shell window. Its very useful for providing a command line interface inside a GUI app.

## features
- host in any container
- persisted (optional) searchable history

## demo

Run it with `cargo run -p demo`. Type 'help' at the command prompt. Shows integration with https://docs.rs/clap/latest/clap/ 

![image](https://github.com/user-attachments/assets/de2df396-68ac-4723-ae62-2811fb81ba05)

## use

You need a ConsoleWindow instance in your egui App 
```
pub struct ConsoleDemo {
    // Example stuff:
    label: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    console: ConsoleWindow,
}
```
