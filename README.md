# A console window for egui
Provides a console window for egui. This is not a shell to the OS its simply a command shell window. Its very useful for providing a command line interface inside a GUI app.

## features
- host in any container
- persisted (optional) searchable history

## demo

Run it with `cargo run -p demo`. Type 'help' at the command prompt


## use

- create a ConsoleBuilder, at least specify a prompt
- call create to obtain an instance of ConsoleWindow. This needs to be persisteed between frames.
- In each update call ConsoleWindow::draw
- If the user entered a command then this call returns what they entered, otherwise None
- output to console with ConsoleWindow::write
- reprompt with ConsoleWindow::prompt
