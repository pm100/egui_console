# A console window for egui
Provides a console window for egui. This is not a shell to the OS its simply a command shell window. Its very useful for providing a command line interface inside a GUI app.

## features
- host in any container
- persisted (optional) searchable history
- tab completion for filesystem paths and arbitrary commands

## demo

Run it with `cargo run -p demo`. Type 'help' at the command prompt. Shows integration with https://docs.rs/clap/latest/clap/ 

![image](https://github.com/user-attachments/assets/de2df396-68ac-4723-ae62-2811fb81ba05)

To see command completeion type 'l<tab>'.
To see filesystem completion try 'cd s<tab>'

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

And then use the builder to instantiate a ConsoleWindow

```
        ConsoleBuilder::new()
                .prompt(">> ")
                .history_size(20)
                .tab_quote_character('\"')
                .build();
```

On each ui update cycle call the draw method, passing in the Ui instance that should host the console window. Draw returns a ConsoleEvent enum, at the moment this is either None or the text of the command the user entered.
```
    let console_response = self.console.draw(ui);
    if let ConsoleEvent::Command(command) = console_response {
        self.console.write(&command);
        self.console.prompt();
    }
```
The prompt method repromts the user. The sample above simply echoes the command the user entered and then reprompts.

### command completion

Tab completion for 'commands' works if the user types part of a command at the prompt; ie it must be the first thing on the line.

You must supply a table of commands for tab completion to work. The console window maintains a `Vec<String>` of commands. you can modify this table by calling the `command_table_mut` method. THis returns a mutable reference to the command table.

The demo app loads this from the clap subcommands
