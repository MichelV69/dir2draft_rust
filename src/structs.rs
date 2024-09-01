// --- structs.rs
pub mod List {

    #[derive(Clone)]
    pub struct AppCfg {
        pub content_path: String,
        pub output_file: String,
    }

    #[derive(Clone, PartialEq)]
    pub struct Title {
        pub sort_by: String,
        pub display_by: String,
    }

    #[derive(Clone)]
    pub struct Book {
        pub title: Title,
        pub part_list: Vec<Part>,
    }

    #[derive(Clone)]
    pub struct Part {
        pub title: Title,
        pub chapter_list: Vec<Chapter>,
    }

    #[derive(Clone)]
    pub struct Chapter {
        pub title: Title,
        pub scene_list: Vec<Scene>,
    }

    #[derive(Clone)]
    pub struct Scene {
        pub title: Title,
        pub content: String,
    }
}

// --- structs.rs
