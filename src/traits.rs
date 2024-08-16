// --- traits.rs
pub mod List {
    pub trait PartFns {
        fn is_a_new_part(&mut self, unsorted_title: &str) -> bool;

        fn for_part(&mut self, new_title: &str, new_content: &str) -> Result<(), ()>;
    }
}

// --- traits.rs
