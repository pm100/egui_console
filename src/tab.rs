use std::path::PathBuf;

use crate::ConsoleWindow;
impl ConsoleWindow {
    pub(crate) fn tab_complete(&mut self) -> (bool, Option<String>) {
        if self.tab_string.is_empty() {
            self.tab_quoted = false;
            let last = self.get_last_line().to_string();
            // let args = last.split_ascii_whitespace().collect::<Vec<&str>>();
            let args = self.digest_line(&last);
            let last_arg = &args[args.len() - 1];
            if last_arg.is_empty() {
                return (true, None);
            }
            println!("{} {} {}", last_arg, self.tab_string, last);
            self.tab_string = last_arg.to_string();
            self.tab_nth = 0;
            self.tab_offset = self.text.len() - last_arg.len();
        } else {
            self.tab_nth += 1; //self.tab_nth.wrapping_add(1);
        }
        loop {
            if let Some(mut path) = fs_tab_complete(&self.tab_string, self.tab_nth) {
                let mut added_quotes = false;
                let mut remove_quotes = self.tab_quoted;
                if path.display().to_string().contains(' ') {
                    path = PathBuf::from(format!(
                        "{}{}{}",
                        self.tab_quote,
                        path.display(),
                        self.tab_quote
                    ));
                    added_quotes = true;
                    remove_quotes = false;
                }
                println!(
                    "{} {} {} {}",
                    added_quotes, remove_quotes, self.tab_quoted, self.tab_offset
                );

                let tab_off = if self.tab_quoted || remove_quotes {
                    self.tab_offset
                } else {
                    self.tab_offset
                };
                self.text.truncate(tab_off);
                self.force_cursor_to_end = true;
                self.text.push_str(path.to_str().unwrap());

                self.tab_quoted = added_quotes;
                break;
            } else {
                if self.tab_nth == 0 {
                    break;
                }
                // force wrap around to first match
                self.tab_nth = 0;
            }
        }
        (true, None)
    }

    fn digest_line(&mut self, line: &str) -> Vec<String> {
        let chunks: Vec<&str> = line.split_ascii_whitespace().collect();

        let mut result: Vec<String> = Vec::new();
        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.ends_with(self.tab_quote) && !chunk.starts_with(self.tab_quote) && i > 0 {
                if chunks[i - 1].starts_with(self.tab_quote)
                    && !chunks[i - 1].ends_with(self.tab_quote)
                {
                    result[i - 1].push(' ');
                    result[i - 1].push_str(chunk);
                    continue;
                }
            }
            result.push(chunk.to_string());
        }
        result
    }
}
pub(crate) fn fs_tab_complete(search: &str, nth: usize) -> Option<PathBuf> {
    let use_backslash = if cfg!(target_os = "windows") && search.find('\\').is_some() {
        ".\\"
    } else {
        "./"
    };
    let mut search_path = PathBuf::from(search);

    let mut nth = nth;
    let mut added_dot = false;

    let mut base_search = loop {
        if search_path.is_dir() {
            break search_path;
        }
        let parent = search_path.parent();
        if parent.is_none() {
            return None;
        } else {
            search_path = parent.unwrap().to_path_buf();
            if search_path.display().to_string().is_empty() {
                search_path = PathBuf::from(use_backslash);
                added_dot = true;
                break search_path;
            } else if search_path.display().to_string() == "." {
                search_path = PathBuf::from(use_backslash);
                break search_path;
            }
        }
    };

    if base_search.display().to_string() == ".." {
        base_search = PathBuf::from(format!(".{}", use_backslash));
    }

    println!("root_path: {}", base_search.display());
    let x = std::fs::read_dir(&base_search);

    if let Ok(entries) = x {
        for e in entries {
            if let Ok(ent) = e {
                let mut ret_path = ent.path();
                if added_dot {
                    ret_path = ret_path.strip_prefix(use_backslash).ok()?.to_path_buf();
                }
                // println!(
                //     "ent: {} {} {} {} {}",
                //     ent.path().display(),
                //     nth,
                //     ret_path.display(),
                //     search,
                //     added_dot
                // );

                if ret_path.display().to_string().starts_with(search) {
                    if nth == 0 {
                        return Some(ret_path);
                    } else {
                        nth -= 1;
                    }
                }
            }
        }
    }
    None
}
#[test]
fn test_digest_line() {
    let mut console = ConsoleWindow::new(">> ");
    let result = console.digest_line("cd foo");
    assert_eq!(result, vec!["cd", "foo"]);
    let result = console.digest_line("cd \"foo bar\"");
    assert_eq!(result, vec!["cd", "\"foo bar\""]);
    let result = console.digest_line("cd \"foo bar");
    assert_eq!(result, vec!["cd", "\"foo", "bar"]);
    let result = console.digest_line("cd foo bar\"");
    assert_eq!(result, vec!["cd", "foo", "bar\""]);
    let result = console.digest_line("\"cd foo bar\"");
    assert_eq!(result, vec!["\"cd", "foo", "bar\""]);
    let result = console.digest_line("cd\" foo bar\"");
    assert_eq!(result, vec!["cd\"", "foo", "bar\""]);
}
