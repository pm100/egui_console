use std::{fs::DirEntry, path::PathBuf};

pub(crate) fn tab_complete(search: &str, nth: usize) -> Option<PathBuf> {
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
            if search_path.display().to_string().len() == 0 {
                search_path = PathBuf::from(use_backslash);
                added_dot = true;
                break search_path;
            } else {
                if search_path.display().to_string() == "." {
                    search_path = PathBuf::from(use_backslash);
                    break search_path;
                }
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
                println!(
                    "ent: {} {} {} {} {}",
                    ent.path().display(),
                    nth,
                    ret_path.display(),
                    search,
                    added_dot
                );

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
