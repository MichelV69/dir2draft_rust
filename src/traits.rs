// --- traits.rs
pub mod List {
    use crate::error_handling::*;
    use crate::structs::List::*;

    trait Title {
        fn new() -> Self;
    }

    trait Book {
        fn new() -> Self;
    }

    trait Part {
        fn new() -> Self;
    }

    //  pub trait PartImpls {
    //      fn is_a_new_part(&mut self, unsorted_title: &str) -> bool;
    //      fn for_part(&mut self, new_title: &str) -> Result<Option<&Part>, appErrors>;
    //  }
    //
    //  pub trait ChapterImpls {
    //      fn is_a_new_chapter(&mut self, unsorted_title: &str) -> bool;
    //      fn for_chapter(&mut self, new_title: &str) -> Result<Option<&Chapter>, appErrors>;
    //  }
}

// --- traits.rs
