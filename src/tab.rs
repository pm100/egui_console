use std::path::PathBuf;

use crate::ConsoleWindow;
impl ConsoleWindow {
    pub(crate) fn tab_complete(&mut self) {
        if self.tab_string.is_empty() {
            // means we are entering tab search mode

            // the main fiddling here is for path with spaces in them on mac and windows
            // the user can enter a partial path with quotes, which we have to strip before passing to the
            // fs tabber and reinstate after an answer comes back
            // else if we get a path with spaces in it from the fs tabber we have to add quotes

            self.tab_quoted = false;
            let last = self.get_last_line().to_string();

            let args = self.digest_line(&last);
            let last_arg = &args[args.len() - 1];
            if last_arg.is_empty() {
                return;
            }
            if last_arg.starts_with(self.tab_quote) {
                self.tab_string = last_arg.strip_prefix(self.tab_quote).unwrap().to_string();
            }
            //println!("{} {} {}", last_arg, self.tab_string, last);
            else {
                self.tab_string = last_arg.to_string()
            };
            self.tab_nth = 0;
            self.tab_offset = self.text.len() - last_arg.len();
        } else {
            // otherwise move to the next match
            self.tab_nth += 1;
        }
        // the loop gets us back to the first match once fs tabber returns no match
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

                self.text.truncate(self.tab_offset);
                self.force_cursor_to_end = true;
                self.text.push_str(path.to_str().unwrap());

                self.tab_quoted = added_quotes;
                break;
            } else {
                // exit if there were no matches at all
                if self.tab_nth == 0 {
                    break;
                }
                // force wrap around to first match
                self.tab_nth = 0;
            }
        }
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

// return the nth matching path, or None if there isnt one
pub(crate) fn fs_tab_complete(search: &str, nth: usize) -> Option<PathBuf> {
    let dot_slash = if cfg!(target_os = "windows") && search.find('\\').is_some() {
        ".\\"
    } else {
        "./"
    };
    let search_path = PathBuf::from(search);

    let mut nth = nth;
    let mut added_dot = false;

    // were we given a real path to start with?

    let mut base_search = if search_path.is_dir() {
        search_path
    } else {
        // no - look at the parent (ie we got "cd dir/f")
        let parent = search_path.parent();
        if parent.is_none() {
            return None;
        } else {
            let p = parent.unwrap().to_path_buf();
            // if empty parent then search "." (remember we added the dot so remove it later)
            if p.display().to_string().is_empty() {
                added_dot = true;
                PathBuf::from(dot_slash)
            } else if p.display().to_string() == "." {
                // we were given . as a dir
                PathBuf::from(dot_slash)
            } else {
                p
            }
        }
    };
    // convert .. to ../ or ..\
    if base_search.display().to_string() == ".." {
        base_search = PathBuf::from(format!(".{}", dot_slash));
    }

    println!("root_path: {}", base_search.display());
    let x = std::fs::read_dir(&base_search);

    if let Ok(entries) = x {
        for e in entries {
            if let Ok(ent) = e {
                let mut ret_path = ent.path();
                if added_dot {
                    ret_path = ret_path.strip_prefix(dot_slash).ok()?.to_path_buf();
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
