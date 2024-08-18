// --- implementations.rs
pub mod List {
    use crate::structs::List::*;
    use crate::traits::List::*;
    use crate::error_handling::*;

    impl Title {
        pub fn new() -> Self {
            Self {
                sort_by: "001 - The Title".into(),
                display: "The Title".into(),
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

 //   impl Chapter {
 //       pub fn new() -> Self {
 //           Self {
 //               title: Title::new(),
 //               scene_list: [].to_vec(),
 //           }
 //       }
 //   }

 //   impl Part {
 //       pub fn new() -> Self {
 //           Self {
 //               title: Title::new(),
 //               chapter_list: [].to_vec(),
 //           }
 //       }
 //   }

 //   impl Book {
 //       pub fn new() -> Self {
 //           Self {
 //               title: Title::new(),
 //               part_list: [].to_vec(),
 //           }
 //       }
 //   }

//    impl PartFns for Book {
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
//    }
}

// --- implementations.rs
