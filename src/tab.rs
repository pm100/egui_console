use std::{fs::DirEntry, path::PathBuf};

pub(crate) fn tab_complete(search: &str, nth: usize) -> Option<DirEntry> {
    let mut search_path = PathBuf::from(search);
    let mut nth = nth;
    let base_search = loop {
        if search_path.is_dir() {
            break search_path;
        }
        let y = search_path.parent();
        if y.is_none() {
            return None;
        } else {
            search_path = y.unwrap().to_path_buf();
        }
    };
    //   let orig_canon = root_path.canonicalize().ok()?;
    // let search_str = search.to_string_lossy();
    // let root_path = if search.is_dir() {
    //     search
    // } else {
    //     search.parent().unwrap().to_path_buf()
    // };
    // if !root_path.is_dir() {
    //     return None;
    // };
    // let search_str = search.to_string_lossy();
    // let root_path = root_path.canonicalize().ok()?;
    println!("root_path: {}", base_search.display());
    let x = std::fs::read_dir(&base_search);

    if let Ok(entries) = x {
        for e in entries {
            if let Ok(ent) = e {
                println!("ent: {} {}", ent.path().display(), nth);
                if ent.path().display().to_string().starts_with(search) {
                    if nth == 0 {
                        return Some(ent);
                    } else {
                        nth -= 1;
                    }
                }
            }
        }
        //     let ent = entries
        //         .filter(|ent| {
        //             println!("ent: {:?}", ent);
        //             if let Ok(ent) = ent {
        //                 let canon = ent.path().canonicalize();
        //                 if let Ok(c) = canon {
        //                     let cx = c.to_string_lossy();
        //                     println!("cx: {} xs {}", cx, orig_canon);
        //                     return cx.starts_with(orig_canon.as_ref());
        //                 } else {
        //                     return false;
        //                 }
        //             } else {
        //                 return false;
        //             }
        //         })
        //         .nth(nth);

        //     if let Some(entry) = ent {
        //         return Some(entry.unwrap());
        //     }
    }
    None
}
