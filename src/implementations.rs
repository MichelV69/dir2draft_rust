use std::fs::File;

// --- implementations.rs
use crate::error_handling::AppErrors::{self, *};
use crate::structs::{AppCfg, Book, Chapter, Part, Scene, Title};
use crate::traits::{AppCfgImpls, AppCfgWG, BookImpls, ChapterImpls, PartImpls, SceneImpls};
use itertools::Itertools;
use std::io::Write;
use walkdir::WalkDir;

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
    fn sort_part_list(book: &Self) -> Book {
        let mut to_sort: Book = book.clone();
        to_sort.part_list.sort_by(|a, b| {
            a.title
                .sort_by
                .partial_cmp(&b.title.sort_by)
                .expect(&format!("{}", ValidPartList))
        });
        to_sort
    }

    fn write_toc(book: Self, output_file: &mut File) {
        fn three_dashes(output_file: &mut File) {
            writeln!(output_file, "{}", "\r\n---\r\n")
                .expect(&format!("{}", AppErrors::CannotWriteToFile));
        }

        let mut this_book: Book = book.clone();
        writeln!(output_file, "\r\n# {}\r\n", "Table of Contents")
            .expect(&format!("{}", AppErrors::CannotWriteToFile));

        this_book = Book::sort_part_list(&this_book);

        for mut part in this_book.part_list {
            part = Part::sort_chapter_list(part);
            three_dashes(output_file);
            writeln!(output_file, "## {}\r\n", part.title.display_by)
                .expect(&format!("{}", AppErrors::CannotWriteToFile));
            for mut chapter in part.chapter_list {
                chapter = Chapter::sort_scene_list(chapter);
                writeln!(output_file, "### {}\r\n", chapter.title.display_by)
                    .expect(&format!("{}", AppErrors::CannotWriteToFile));
                for scene in chapter.scene_list {
                    writeln!(output_file, "#### {}\r\n", scene.title.display_by)
                        .expect(&format!("{}", AppErrors::CannotWriteToFile));
                } // for scene
            } //for chapter
        } //for part

        three_dashes(output_file);
    }

    fn write_content(book: Self, output_file: &mut File) {
        println!("Assembling novel for you ...");
        print!("[");
        fn three_dashes(output_file: &mut File) {
            writeln!(output_file, "{}", "\r\n---\r\n")
                .expect(&format!("{}", AppErrors::CannotWriteToFile));
        }

        let mut this_book: Book = book.clone();
        writeln!(output_file, "\r\n# {}\r\n", book.title.display_by)
            .expect(&format!("{}", AppErrors::CannotWriteToFile));

        this_book = Book::sort_part_list(&this_book);

        for mut part in this_book.part_list {
            part = Part::sort_chapter_list(part);
            three_dashes(output_file);
            writeln!(output_file, "## {}\r\n", part.title.display_by)
                .expect(&format!("{}", AppErrors::CannotWriteToFile));
            print!("P");

            for mut chapter in part.chapter_list {
                chapter = Chapter::sort_scene_list(chapter);
                writeln!(output_file, "### {}\r\n", chapter.title.display_by)
                    .expect(&format!("{}", AppErrors::CannotWriteToFile));
                print!("C");

                for scene in chapter.scene_list {
                    writeln!(output_file, "#### {}\r\n", scene.title.display_by)
                        .expect(&format!("{}", AppErrors::CannotWriteToFile));
                    print!("S");

                    writeln!(output_file, "#### {}\r\n", scene.content)
                        .expect(&format!("{}", AppErrors::CannotWriteToFile));
                    print!("c");
                } // for scene
            } //for chapter
        } //for part

        three_dashes(output_file);
        println!("]!");
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

    fn part_exists(&mut self, check_for: &str) -> bool {
        self.part_list.iter().any(|i| i.title.sort_by == check_for)
    }

    fn chapter_exists(&mut self, part_index: usize, check_for: &str) -> bool {
        self.part_list[part_index]
            .chapter_list
            .iter()
            .any(|i| i.title.sort_by == check_for)
    }

    fn find_chapter(&mut self, part_index: usize, search_title: &str) -> Option<usize> {
        let found_chap_index = self.part_list[part_index]
            .chapter_list
            .iter()
            .position(|i| i.title.sort_by == search_title);
        if Some(found_chap_index).is_some() {
            found_chap_index
        } else {
            Some(0)
        }
    }

    fn scene_exists(&mut self, part_index: usize, chap_index: usize, check_for: &str) -> bool {
        self.part_list[part_index].chapter_list[chap_index]
            .scene_list
            .iter()
            .any(|i| i.title.sort_by == check_for)
    }

    fn find_scene(
        &mut self,
        part_index: usize,
        chap_index: usize,
        search_title: &str,
    ) -> Option<usize> {
        let found_scene_index = self.part_list[part_index].chapter_list[chap_index]
            .scene_list
            .iter()
            .position(|i| i.title.sort_by == search_title);
        if Some(found_scene_index).is_some() {
            found_scene_index
        } else {
            Some(0)
        }
    }

    // ----------------

    fn add_content(&mut self, app: &AppCfg, book_piece: &str) {
        // gets 1 item, a DirPath that may or may not be a file,
        // and determines "what part" of the book and returns that

        let mut part_index: usize = 0;
        let mut chapter_index: usize = 0;

        if book_piece.contains("DNI ==") {
            println!("Skipping Do-Not-Include: {:#?}", &book_piece);
            return ();
        }

        if !book_piece.contains(".md") {
            return ();
        }

        let search_for = format!("{}\\{}", app.content_path.clone(), &book_piece);
        println!("\n----\n was given book_piece: {:#?}", &search_for);

        let dir_listing = WalkDir::new(search_for);
        for dir_item in dir_listing
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("Asked for [{:#?}]", dir_item.path());
            if dir_item.file_type().is_file() {
                let temp_heap = dir_item
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .replace(&app.content_path, "");

                let mut path_pieces = temp_heap.split("\\").into_iter().collect_vec();

                path_pieces.retain(|p| !p.is_empty());

                if path_pieces.len() == 3 {
                    // Part + Chapter + Scene
                    let mut new_part = Part::new();
                    new_part.title.sort_by = path_pieces[0].to_string();
                    new_part.title.display_by = Part::smart_title(&new_part.title.sort_by).into();
                    if !self.part_exists(&new_part.title.sort_by) {
                        self.part_list.push(new_part.clone());
                        println!("Added Part: {:#?}", new_part.title.display_by);
                    }
                    let needle1 = path_pieces[0].to_string().clone();
                    part_index = self.find_part(&needle1).expect("expected usized value");

                    let mut new_chap = Chapter::new();
                    new_chap.title.sort_by = path_pieces[1].to_string();
                    new_chap.title.display_by =
                        Chapter::smart_title(&new_chap.title.sort_by).into();
                    if !self.chapter_exists(part_index, &new_chap.title.sort_by) {
                        self.part_list[part_index]
                            .chapter_list
                            .push(new_chap.clone());
                        println!("Added Chapter: {:#?}", new_chap.title.display_by);
                    }
                    let needle2 = path_pieces[1].to_string().clone();
                    chapter_index = self
                        .find_chapter(part_index, &needle2)
                        .expect("expected usized value");

                    let mut this_scene = Scene::new();
                    this_scene.title.sort_by = path_pieces[2].to_string();
                    this_scene.title.display_by =
                        Scene::smart_title(&this_scene.title.sort_by).into();

                    if !self.scene_exists(part_index, chapter_index, &this_scene.title.sort_by) {
                        self.part_list[part_index].chapter_list[chapter_index]
                            .scene_list
                            .push(this_scene.clone());
                        println!("Added Scene: {:#?}", this_scene.title.display_by);
                    }
                    let needle3 = path_pieces[2].to_string().clone();
                    let mut new_scene_index = self.find_scene(part_index, chapter_index, &needle3).expect("expected usized value");;

                    self.part_list[part_index]
                        .chapter_list[chapter_index]
                    .scene_list[new_scene_index]=  Scene::get_content_for(dir_item );

                } // path_pieces.len() == 3
            };
        } // dir_item.file_type().is_file()
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

    fn sort_scene_list(chapter: Self) -> Self {
        let mut to_sort: Chapter = chapter;
        to_sort.scene_list.sort_by(|a, b| {
            a.title
                .sort_by
                .partial_cmp(&b.title.sort_by)
                .expect(&format!("{}", ValidSceneList))
        });
        to_sort
    }
}

impl SceneImpls for Scene {
    fn smart_title(sortable_title: &str) -> String {
        let split_meta: &str = "==";
        let ftype: &str = ".md";
        let buffer = sortable_title
            .split(split_meta)
            .last()
            .expect(&format!("{}", PlainTextString));
        let smart_title: String = buffer.split(' ').filter(|s| !s.is_empty()).join(" ");
        smart_title.replace(ftype, "")
    }
    fn get_content_for(content_file: &str) -> Option<String> {
        use std::fs;
        let dir_contents = WalkDir::new(content_file.clone());
        for entry in dir_contents
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                println!("is_file** [{:#?}] [{:#?}]", entry.file_name(), content_file);
                if content_file.ends_with(entry.file_name().to_str().unwrap()) {
                    println!("Made it this far. Ish. Looking for {:#?}", entry.path());
                    return Some(
                        fs::read_to_string(entry.path()).expect(&format!("{}", ReadableFile)),
                    );
                }
            }
        }
        return None;
    }
}

// --- implementations.rs
