// --- implementations.rs
pub mod List {
    // ---- need walkdir for directory traversal
    extern crate walkdir;
    use std::fs::File;
    use std::io::Write;
    use walkdir::WalkDir;
    // ----- iteration management
    use std::path::Path;
    use itertools::Itertools;

    // -- other stuff
    use crate::error_handling::AppErrors::*;
    use crate::error_handling::*;
    use crate::structs::List::*;
    use crate::traits::List::*;

    impl App {
        pub fn new() -> Self {
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
        pub fn new() -> Self {
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

    impl AppImpls for App {
        fn get_path_elements<'a>(content_path: &'a String) -> Vec<String>{
            use walkdir::DirEntry;
            // ---
            fn do_strip_prefix(this_content_path: String, this_path: DirEntry) -> String {
                this_path.path()
                    .strip_prefix(this_content_path)
                    .expect(&getExpected(VaildPath))
                    .to_str()
                    .expect(&getExpected(PlainTextString))
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
                    if ! path_elements.contains(&new_path_string) && new_path_string.len() > 0 {
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
                    .expect(&getExpected(ValidPartList))
            });
            to_sort
        }
    }

    impl PartImpls for Part {
        fn smart_title(sortable_title: &str) -> String {
            let split_meta: &str = "==";
            let smart_title = sortable_title
                .split(split_meta)
                .last()
                .expect(&getExpected(PlainTextString));
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
                    .expect(&getExpected(ValidChapterList))
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
                .expect(&getExpected(PlainTextString));
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
                .expect(&getExpected(PlainTextString));
            smart_title
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .join(" ")
        }
    }

    //        fn is_a_new_part(&mut self, unsorted_title: &str) -> bool {
    //            !self.part_list.iter().any(|&i| i.title.sort_by == unsorted_title)
    //        }
    //
    //        fn for_part(&mut self, unsorted_title: &str) -> Result<Option<&Part>, appErrors> {
    //            let found_part = self.part_list
    //                .iter()
    //                .find(|&i| i.title.sort_by == unsorted_title);
    //
    //            if Some(found_part).is_some() {Ok(found_part)}
    //            else {Err(noSuchPart)}
    //        }
}

// --- implementations.rs
