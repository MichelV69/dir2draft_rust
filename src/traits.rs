// --- traits.rs
pub mod List {
    use crate::error_handling::*;
    use crate::structs::List::*;
    use std::fs::File;

    pub trait AppCfgWG {
        fn new() -> Self
        where
            Self: Sized;
    }

    trait Title {
        fn new() -> Self;
    }

    trait Book {
        fn new() -> Self
        where
            Self: Sized;
    }

    trait Part {
        fn new() -> Self
        where
            Self: Sized;
    }

    trait Chapter {
        fn new() -> Self;
    }

    trait Scene {
        fn new() -> Self;
    }

    pub trait BookImpls {
        fn sort_part_list(book: &Self) -> Self;
        fn write_toc(book: Self, output_file: &mut File);
        fn write_content(book: Self, output_file: &mut File);
        fn find_part(&mut self, unsorted_title: &str) -> Option<usize>;
        fn add_content(&mut self, app: &AppCfg, dir_entry: &str);
        fn part_exists(&mut self, dir_entry: &str) -> bool;
    }

    pub trait AppCfgImpls {
        fn get_path_elements<'a>(content_path: &'a String) -> Vec<String>;
    }

    pub trait PartImpls {
        fn smart_title(sortable_title: &str) -> String;
        fn sort_chapter_list(part: Self) -> Self;
    }

    pub trait ChapterImpls {
        fn smart_title(sortable_title: &str) -> String;
        fn sort_scene_list(chapter: Self) -> Self;
    }

    pub trait SceneImpls {
        fn smart_title(sortable_title: &str) -> String;
        fn get_content_for(content_path: String, dir_entry: &str) -> Option<String>;
    }

    // fn is_a_new_part(&mut self, unsorted_title: &str) -> bool;
    // fn for_part(&mut self, new_title: &str) -> Result<Option<&Part>, appErrors>;

    //    //  pub trait ChapterImpls {
    //      fn is_a_new_chapter(&mut self, unsorted_title: &str) -> bool;
    //      fn for_chapter(&mut self, new_title: &str) -> Result<Option<&Chapter>, appErrors>;
    //  }
}

// --- traits.rs
