// --- implementations.rs
pub mod List {
    // ---- need walkdir for directory traversal
    extern crate walkdir;
    use std::fs::File;
    use std::io::Write;
    use walkdir::WalkDir;
    // ----- iteration management
    use itertools::Itertools;
    use std::path::Path;

    // -- other stuff
    use crate::error_handling::AppErrors::*;
    use crate::error_handling::*;
    use crate::structs::List::*;
    use crate::traits::List::*;

    impl AppCfgWG for AppCfg {
        fn new() -> Self {
            Self {
                content_path: "~".into(),
                output_file: "dir2draft.manuscript".into(),
            }
        }
    }

    impl Title {
        pub fn new() -> Self {
            Self {
                sort_by: "001 - The Title".into(),
                display_by: "The Title".into(),
            }
        }
    }

    impl Book {
        pub fn new() -> Self {
            Self {
                title: Title::new(),
                part_list: [].to_vec(),
            }
        }
    }

    impl Part {
        pub fn new() -> Self
        where
            Self: Sized,
        {
            Self {
                title: Title::new(),
                chapter_list: [].to_vec(),
            }
        }
    }

    impl Chapter {
        pub fn new() -> Self {
            Self {
                title: Title::new(),
                scene_list: [].to_vec(),
            }
        }
    }

    impl Scene {
        pub fn new() -> Self {
            Self {
                title: Title::new(),
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
                    nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
                    reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
                    pariatur. Excepteur sint occaecat cupidatat non proident, sunt in
                    culpa qui officia deserunt mollit anim id est laborum."
                    .into(),
            }
        }
    }

    impl AppCfgImpls for AppCfg {
        fn get_path_elements<'a>(content_path: &'a String) -> Vec<String> {
            use walkdir::DirEntry;
            // ---
            fn do_strip_prefix(this_content_path: String, this_path: DirEntry) -> String {
                this_path
                    .path()
                    .strip_prefix(this_content_path)
                    .expect(&format!("{}", VaildPath))
                    .to_str()
                    .expect(&format!("{}", PlainTextString))
                    .to_string()
            }

            // ----
            let mut path_elements: Vec<String> = vec![];
            for entry in WalkDir::new(content_path.clone())
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let txt_buffer: String = do_strip_prefix(content_path.clone(), entry);

                let vec_buffer: Vec<&str> = txt_buffer.split("/").collect();

                for di in vec_buffer {
                    let new_path_string = di.to_string();
                    if !path_elements.contains(&new_path_string) && new_path_string.len() > 0 {
                        path_elements.push(new_path_string);
                    }
                }
            }
            path_elements
        }
    }

    impl BookImpls for Book {
        fn sort_part_list(book: Self) -> Book {
            let mut to_sort: Book = book;
            to_sort.part_list.sort_by(|a, b| {
                a.title
                    .sort_by
                    .partial_cmp(&b.title.sort_by)
                    .expect(&format!("{}", ValidPartList))
            });
            to_sort
        }

        fn find_part(&mut self, search_title: &str) -> Option<usize> {
            let found_part_index = self
                .part_list
                .iter()
                .position(|i| i.title.sort_by == search_title);
            if Some(found_part_index).is_some() {
                found_part_index
            } else {
                Some(0)
            }
        }

        fn part_exists(&mut self, dir_entry: &str) -> bool {
            self.part_list.iter().any(|i| i.title.sort_by == dir_entry)
        }

        fn add_content(&mut self, app: &AppCfg, dir_entry: &str) {
            let mut part_index: usize = 0;
            let mut chapter_index: usize = 0;
            use std::env;
            use std::fs;
            use walkdir::DirEntry;

            if self.part_list.len() > 0 {
                part_index = self.part_list.len() - 1;
                if self.part_list[part_index].chapter_list.len() > 0 {
                    chapter_index = self.part_list[part_index].chapter_list.len() - 1
                };
            };

            if dir_entry.contains(".md") {
                let mut this_scene = Scene::new();
                this_scene.title.sort_by = dir_entry.to_string();
                this_scene.title.display_by = Scene::smart_title(&this_scene.title.sort_by).into();
                this_scene.content = Scene::get_content_for(app.content_path.clone(), &dir_entry)
                    .expect(&format!("{}", PlainTextString));
                self.part_list[part_index].chapter_list[chapter_index]
                    .scene_list
                    .push(this_scene);
                println!("Added Scene: {:#?}", &dir_entry);
                return ();
            }

            let dir_contents = WalkDir::new(app.content_path.clone().clone());
            for entry in dir_contents
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_dir()
                    && entry.depth() == 1
                    && entry.file_name() == dir_entry
                {
                    let mut to_add = Part::new();
                    to_add.title.sort_by = dir_entry.to_string();
                    to_add.title.display_by = Part::smart_title(&to_add.title.sort_by).into();
                    self.part_list.push(to_add);
                    println!("Added Part: {:#?}", &dir_entry);
                }

                if entry.file_type().is_dir()
                    && entry.depth() == 2
                    && entry.file_name() == dir_entry
                {
                    let mut to_add = Chapter::new();
                    to_add.title.sort_by = dir_entry.to_string();
                    to_add.title.display_by = Chapter::smart_title(&to_add.title.sort_by).into();
                    self.part_list[part_index].chapter_list.push(to_add);
                    println!("Added Chapter: {:#?}", &dir_entry);
                }
            }

            return ();
        } // fn add_content
    } // impl BookImpls for Book

    impl PartImpls for Part {
        fn smart_title(sortable_title: &str) -> String {
            let split_meta: &str = "==";
            let smart_title = sortable_title
                .split(split_meta)
                .last()
                .expect(&format!("{}", PlainTextString));
            smart_title
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .join(" ")
        }

        fn sort_chapter_list(part: Self) -> Self {
            let mut to_sort: Part = part;
            to_sort.chapter_list.sort_by(|a, b| {
                a.title
                    .sort_by
                    .partial_cmp(&b.title.sort_by)
                    .expect(&format!("{}", ValidChapterList))
            });
            to_sort
        }
    }

    impl ChapterImpls for Chapter {
        fn smart_title(sortable_title: &str) -> String {
            let split_meta: &str = "==";
            let smart_title = sortable_title
                .split(split_meta)
                .last()
                .expect(&format!("{}", PlainTextString));
            smart_title
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .join(" ")
        }
    }

    impl SceneImpls for Scene {
        fn smart_title(sortable_title: &str) -> String {
            let split_meta: &str = "==";
            let smart_title = sortable_title
                .split(split_meta)
                .last()
                .expect(&format!("{}", PlainTextString));
            smart_title
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .join(" ")
        }
        fn get_content_for(content_path: String, dir_entry: &str) -> Option<String> {
            use std::env;
            use std::fs;
            use walkdir::DirEntry;
            let dir_contents = WalkDir::new(content_path.clone());
            for entry in dir_contents
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() && entry.file_name() == dir_entry {
                    return Some(
                        fs::read_to_string(entry.path()).expect(&format!("{}", ReadableFile)),
                    );
                }
            }
            return None;
        }
    }
}

// --- implementations.rs
