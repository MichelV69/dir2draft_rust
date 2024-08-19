// --- implementations.rs
pub mod List {
    use crate::error_handling::AppErrors::*;
    use crate::error_handling::*;
    use crate::structs::List::*;
    use crate::traits::List::*;
    use itertools::Itertools;

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
                //          scene_list: [].to_vec(),
            }
        }
    }

    //   impl Scene {
    //       pub fn new() -> Self {
    //           Self {
    //               title: Title::new(),
    //               content: "New Scene".into(),
    //           }
    //       }
    //   }

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
