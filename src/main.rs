extern crate walkdir;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

fn main() {
    // ---- will be incoming ARGs
    let content_path = "./content";
    let output_file = "my_book_title";

    // ---- structures
    struct Title {
        sort_by: String,
        display: String,
    }

    struct Scene {
        title: Title,
        content: String,
    }

    struct Chapter {
        title: Title,
        scene_list: Vec<Scene>,
    }

    struct Part {
        title: Title,
        chapter_list: Vec<Chapter>,
    }

    struct Book {
        title: Title,
        part_list: Vec<Part>,
    }

    // ---- function set
    trait PartFns {
        fn is_a_new_part(self, unsorted_title: &str) -> bool;
    }

    // ---- implements
    impl Title {
        fn new() -> Title {
            Title {
                sort_by: "01A",
                display: "First Title",
            }
        }
    }

    impl Scene {
        fn new() -> Scene {
            Scene {
                title: Title::new(),
                content: "New Scene",
            }
        }
    }

    impl Chapter {
        fn new() -> Chapter {
            Chapter {
                title: Title::new(),
                scene_list: [],
            }
        }
    }

    impl Part {
        fn new() -> Part {
            Part {
                title: Title::new(),
                chapter_list: [],
            }
        }
    }

    impl Book {
        fn new() -> Book {
            Book {
                title: Title::new(),
                part_list: [],
            }
        }
    }

    impl PartFns for Book {
        fn is_a_new_part(self, unsorted_title: &str) -> bool {
            true;
        }
    };

    // ---- main code
    let mut project = Book::new();
    let mut write_ptr =
        File::create(format!("{}.md", output_file)).expect("ER01 - Expect valid path");

    for entry in WalkDir::new(content_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let full_path = entry
            .path()
            .strip_prefix(content_path)
            .expect("ER01 - Expect valid path")
            .to_string_lossy();

        let path_elemets: Vec<_> = full_path.split("/").collect();

        if path_elemets.len() == 3 {
            let found_part = path_elemets[0];
            let found_chapter = path_elemets[1];

            if project.is_a_new_part(found_part) {
                project.part_list.add(found_part)
            }

            if project.for_part(found_part).is_a_new_chapter(found_chapter) {
                project.part_list.add(found_chapter)
            }
        }
    }
}
