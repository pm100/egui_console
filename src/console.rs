use std::{cmp::Reverse, collections::VecDeque, str::Lines};

use egui::{
    text::CCursorRange, Align, Context, Event, EventFilter, Id, Key, Modifiers, TextEdit, Ui,
};
static SEARCH_PROMPT: &str = "(reverse-i-search) :";
const SEARCH_PROMPT_SLOT_OFF: usize = 18;

pub enum ConsoleEvent {
    Command(String),
    CtrlC,
    None,
}
pub struct ConsoleWindow {
    text: String,
    new_line: bool,
    history_size: usize,
    scrollback_size: usize,
    command_history: VecDeque<String>,
    history_cursor: Option<usize>,
    prompt: String,
    prompt_len: usize,
    id: Id,
    save_prompt: String,
    search_partial: Option<String>,
}

impl ConsoleWindow {
    pub fn new(prompt: &str) -> Self {
        Self {
            text: prompt.to_string(),
            new_line: false,
            command_history: VecDeque::new(),
            history_cursor: None,
            history_size: 5,
            scrollback_size: 1000,
            prompt: prompt.to_string(),
            prompt_len: prompt.chars().count(),

            id: Id::new("console_text"),
            save_prompt: prompt.to_string(),
            search_partial: None,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui) -> ConsoleEvent {
        // do we need to handle keyboard events?
        let msg = if ui.ctx().memory(|mem| mem.has_focus(self.id)) {
            self.handle_kb(ui.ctx())
        } else {
            ConsoleEvent::None
        };
        {
            let text_len = self.text.len();
            self.ui(ui);

            // did somebody type?
            if self.text.len() != text_len {
                if self.search_partial.is_some() {
                    self.search_partial = Some(self.get_search_text().to_string());
                    println!("update searching for {:?}", self.search_partial);
                    self.prompt = SEARCH_PROMPT.to_string();
                    self.prompt.insert_str(
                        SEARCH_PROMPT_SLOT_OFF + 1,
                        &self.search_partial.as_ref().unwrap(),
                    );
                    println!("prompt is now {}", self.prompt);
                    if self.search_partial.as_ref().unwrap().is_empty() {
                        self.history_cursor = None;
                    } //else {
                    self.history_back();
                    // }
                }
            }
        }
        // this is all so that we get the escape key (to exit search)
        //  if self.search_partial.is_some() {
        let event_filter = EventFilter {
            escape: true,
            horizontal_arrows: true,
            vertical_arrows: true,
            tab: false,
        };
        if ui.ctx().memory(|mem| mem.has_focus(self.id)) {
            ui.ctx()
                .memory_mut(|mem| mem.set_focus_lock_filter(self.id, event_filter));
        }
        //    }
        msg
    }

    pub fn sync_response(&mut self, data: &str) {
        self.text.push_str(&format!("\n{}\n{}", data, self.prompt));
        self.new_line = true;
    }

    pub fn async_message(&mut self, data: &str) {
        self.text.push_str(&format!("\n{}\n{}", data, self.prompt));
        self.new_line = true;
    }

    pub fn load_history(&mut self, history: Lines<'_>) {
        self.command_history = history.into_iter().map(|s| s.to_string()).collect();
        self.history_cursor = None; //self.command_history.len() as isize - 1;
    }

    pub fn get_history(&self) -> VecDeque<String> {
        self.command_history.clone()
    }

    fn cursor_at_end(&self) -> CCursorRange {
        egui::text::CCursorRange::one(egui::text::CCursor::new(self.text.chars().count()))
    }
    fn cursor_at(&self, loc: usize) -> CCursorRange {
        if loc >= self.text.chars().count() {
            return self.cursor_at_end();
        }
        egui::text::CCursorRange::one(egui::text::CCursor::new(loc))
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                let widget = egui::TextEdit::multiline(&mut self.text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .lock_focus(true)
                    .desired_width(0.0f32)
                    // .desired_width(f32::INFINITY)
                    .id(self.id);
                let output = widget.show(ui);
                let mut new_cursor = None;

                // fix up cursor position
                // different logic depending on normal vs search mode
                // scroll, mouse move etc
                // cursor might not be in a good location

                match self.search_partial {
                    Some(_) => {
                        if let Some(cursor) = output.state.cursor.char_range() {
                            let last_off = self.last_line_offset();
                            if cursor.primary.index < (last_off + SEARCH_PROMPT_SLOT_OFF + 1) {
                                new_cursor =
                                    Some(self.cursor_at(last_off + SEARCH_PROMPT_SLOT_OFF + 1));
                            } else {
                                let search_text = self.get_search_text();
                                if cursor.primary.index
                                    > ((last_off + SEARCH_PROMPT.len() + search_text.len())
                                        as usize)
                                {
                                    new_cursor = Some(self.cursor_at(
                                        last_off + SEARCH_PROMPT_SLOT_OFF + search_text.len() + 1,
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        if let Some(cursor) = output.state.cursor.char_range() {
                            let last_off = self.last_line_offset();
                            if cursor.primary.index < last_off + self.prompt_len - 1 {
                                new_cursor = Some(self.cursor_at_end());
                            }
                        }

                        // we need a new line (user pressed enter)
                        if self.new_line {
                            new_cursor = Some(self.cursor_at_end());
                            self.new_line = false;
                        }
                    }
                };

                if new_cursor.is_some() {
                    let text_edit_id = output.response.id;

                    if let Some(mut state) = TextEdit::load_state(ui.ctx(), text_edit_id) {
                        state.cursor.set_char_range(new_cursor);
                        state.store(ui.ctx(), text_edit_id);
                    }
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }
                output.response
            });
        });
    }

    fn get_last_line(&self) -> &str {
        self.text
            .lines()
            .last()
            .unwrap_or("")
            .strip_prefix(&self.prompt)
            .unwrap_or("")
    }

    fn get_search_text(&self) -> &str {
        let last = self.text.lines().last().unwrap_or("");
        let mut iter = last.char_indices();
        let (start, _) = iter.nth(SEARCH_PROMPT_SLOT_OFF + 1).unwrap_or((0, ' '));
        for (end, ch) in iter {
            // TODO - this will fail if the search text contains ':'
            if ch == ':' {
                return &last[start..end];
            }
        }
        ""
    }
    fn consume_key(ctx: &Context, modifiers: Modifiers, logical_key: Key) {
        ctx.input_mut(|inp| inp.consume_key(modifiers, logical_key));
    }

    fn handle_key(
        &mut self,
        key: &Key,
        modifiers: Modifiers,
        cursor: usize,
    ) -> (bool, Option<String>) {
        // return value is (consume_key, command)

        let return_value = match (modifiers, key) {
            (Modifiers::NONE, Key::ArrowDown) => {
                // down arrow only means something if we are in search mode
                if self.search_partial.is_some() {
                    self.exit_search_mode()
                };
                println!("history_fwd hc = {:?}", self.history_cursor);
                if let Some(mut hc) = self.history_cursor {
                    let last = self.get_last_line();
                    self.text = self.text.strip_suffix(last).unwrap_or("").to_string();
                    if hc == self.command_history.len() - 1 {
                        self.history_cursor = None;
                    } else {
                        if hc < self.command_history.len() - 1 {
                            hc += 1;
                            self.text.push_str(self.command_history[hc].as_str());
                        }
                        self.history_cursor = Some(hc);
                    }
                }
                (true, None)
            }
            (Modifiers::NONE, Key::ArrowUp) => {
                if self.command_history.is_empty() {
                    return (true, None);
                }
                if self.search_partial.is_some() {
                    self.exit_search_mode()
                };

                self.history_back();
                (true, None)
            }
            (Modifiers::NONE, Key::Enter) => {
                let last = self.get_last_line().to_string();
                if self.search_partial.is_some() {
                    self.exit_search_mode()
                };
                if self.command_history.len() >= self.history_size {
                    self.command_history.pop_front();
                }
                self.command_history.push_back(last.clone());

                self.new_line = true;
                self.history_cursor = None;
                (true, Some(last))
            }

            // in search mode the cursor is constrained to the inside of the
            // search prompt. In mormal mode the cursor is constrained to the
            // right of the prompt
            (Modifiers::NONE, Key::Delete) => {
                if let Some(search_partial) = &self.search_partial {
                    let last_off = self.last_line_offset();
                    if cursor > (last_off + SEARCH_PROMPT.len() - 2 + search_partial.len()) {
                        return (true, None);
                    }
                }
                (false, None)
            }
            (Modifiers::NONE, Key::ArrowRight) => {
                // nothing to do in normal mode. In search mode we need to
                // constrain the cursor to the search prompt
                if let Some(search_partial) = &self.search_partial {
                    let last_off = self.last_line_offset();

                    if cursor > (last_off + SEARCH_PROMPT.len() - 2 + search_partial.len()) {
                        return (true, None);
                    }
                }
                (false, None)
            }
            (Modifiers::NONE, Key::ArrowLeft) | (Modifiers::NONE, Key::Backspace) => {
                // in either mode dont allow motion (or deleting) into prompt

                let last_off = self.last_line_offset();
                println!("back");
                match self.search_partial {
                    Some(_) => {
                        if cursor < (last_off + SEARCH_PROMPT_SLOT_OFF + 2) {
                            return (true, None);
                        }
                    }
                    None => {
                        if cursor < (last_off + self.prompt.len() + 1) {
                            return (true, None);
                        }
                    }
                }

                (false, None)
            }
            (Modifiers::NONE, Key::Escape) => {
                if self.search_partial.is_some() {
                    self.exit_search_mode()
                };
                self.history_cursor = None;
                (true, None)
            }

            // ctrl-r reverse search history
            (
                Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: true,
                },
                Key::R,
            ) => {
                if self.search_partial.is_none() {
                    self.search_partial = Some(String::new());
                    self.enter_search_mode();
                } else {
                    self.history_back();
                }
                (true, None)
            }
            _ => (false, None),
        };

        return_value
    }

    fn history_back(&mut self) {
        println!("history_back hc = {:?}", self.history_cursor);
        let hc = match self.history_cursor {
            Some(hc) => hc,
            None => self.command_history.len(),
        };

        let mut hist_line = String::new();
        for i in (0..hc).rev() {
            match &self.search_partial {
                Some(search) => {
                    if search.is_empty() {
                        self.history_cursor = None;
                        break;
                    }
                    println!("searching for {} {}", search, &self.command_history[i]);
                    if self.command_history[i].contains(search) {
                        hist_line = self.command_history[i].clone();
                        self.history_cursor = Some(i);
                        break;
                    }
                }
                None => {
                    hist_line = self.command_history[i].clone();
                    self.history_cursor = Some(i);
                    break;
                }
            }
        }
        let last = self.get_last_line();
        println!("history_back found {:?} last='{}'", hist_line, last);

        self.text = self.text.strip_suffix(last).unwrap_or("").to_string();
        if !hist_line.is_empty() {
            self.text.push_str(&hist_line);
        }
    }

    fn last_line_offset(&self) -> usize {
        // offset in buffer of start of last line
        self.text.rfind('\n').map_or(0, |off| off + 1)
    }
    fn enter_search_mode(&mut self) {
        self.prompt = SEARCH_PROMPT.to_string();
        self.search_partial = Some(String::new());
        println!("prompt is now {}", self.prompt);
        let last_off = self.last_line_offset();
        self.text.truncate(last_off);
        self.draw_prompt();
        self.new_line = true;
    }
    fn exit_search_mode(&mut self) {
        self.prompt = self.save_prompt.clone();
        let last_off = self.last_line_offset();
        self.text.truncate(last_off);
        self.draw_prompt();
        self.search_partial = None;
        self.new_line = true;
    }
    fn draw_prompt(&mut self) {
        if self.text.len() > 0 && !self.text.ends_with('\n') {
            self.text.push('\n');
        }
        self.text.push_str(&self.prompt);
    }

    fn handle_kb(&mut self, ctx: &egui::Context) -> ConsoleEvent {
        // process all the key events in the queue
        // if they are meaningful to the console then use them and consume them
        // otherwise pass along to the textedit widget

        // current cursor position

        let cursor = if let Some(state) = egui::TextEdit::load_state(ctx, self.id) {
            state.cursor.char_range().unwrap().primary.index
        } else {
            0
        };

        // a list of keys to consume

        let mut kill_list = vec![];
        let mut command = None;
        ctx.input(|input| {
            for event in &input.events {
                if let Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    modifiers,
                    repeat: _,
                } = event
                {
                    if *pressed {
                        let (kill, msg) = self.handle_key(key, *modifiers, cursor);
                        if kill {
                            kill_list.push((*modifiers, *key));
                        }
                        command = msg;
                        // if the user pressed enter we are done
                        if command.is_some() {
                            break;
                        }
                    }
                }
            }
        });

        // consume the keys we used
        for (modifiers, key) in kill_list {
            Self::consume_key(ctx, modifiers, key);
        }

        if let Some(command) = command {
            return ConsoleEvent::Command(command);
        }
        ConsoleEvent::None
    }
}
