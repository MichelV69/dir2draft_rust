// --- traits.rs
use crate::File;
use crate::structs::AppCfg;

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
    fn write_toc(book: Self, output_file: &mut File);

    fn write_content(book: Self, output_file: &mut File);
    fn add_content(&mut self, app: &AppCfg, dir_entry: &str);

    fn sort_part_list(book: &Self) -> Self;
    fn find_part(&mut self, search_title: &str) -> Option<usize>;
    fn part_exists(&mut self, dir_entry: &str) -> bool;

    fn chapter_exists(&mut self, part_index: usize, check_for: &str) -> bool;
    fn find_chapter(&mut self, part_index: usize, search_title: &str) -> Option<usize>;

    fn scene_exists(&mut self, part_index: usize, chap_index: usize, check_for: &str) -> bool;
    fn find_scene(
        &mut self,
        part_index: usize,
        chap_index: usize,
        search_title: &str,
    ) -> Option<usize>;
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
    fn get_content_for(dir_entry: &str) -> Option<String>;
}

// --- traits.rs
