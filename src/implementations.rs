// --- implementations.rs
pub mod List {
    use crate::structs::List::*;
    use crate::traits::List::*;

    impl Title {
        fn new() -> Self {
            Self {
                sort_by: "01A".into(),
                display: "First Title".into(),
            }
        }
    }

    impl Scene {
        fn new() -> Self {
            Self {
                title: Title::new(),
                content: "New Scene".into(),
            }
        }
    }

    impl Chapter {
        fn new() -> Self {
            Self {
                title: Title::new(),
                scene_list: [].to_vec(),
            }
        }
    }

    impl Part {
        fn new() -> Self {
            Self {
                title: Title::new(),
                chapter_list: [].to_vec(),
            }
        }
    }

    impl Book {
        fn new() -> Self {
            Self {
                title: Title::new(),
                part_list: [].to_vec(),
            }
        }
    }

    impl PartFns for Book {
        fn is_a_new_part(&mut self, unsorted_title: &str) -> bool {
            !self.part_list.any(|&i| i == &unsorted_title)
        }

        fn for_part(&mut self, new_title: &str, new_content: &str) -> Result<(), ()> {}
    }
}

// --- implementations.rs
