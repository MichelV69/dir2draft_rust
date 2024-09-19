// --- structs.rs
pub mod List {

    use clap::Parser;

    #[derive(Parser)]
    #[command(name = "dir2draft")]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        /// The folder to look for your novel
        #[arg(short, long, default_value_t = ("./content".to_string()))]
        pub content_path: String,
        /// The path to the file to read
        #[arg(short, long, default_value_t = ("my_latest_novel".to_string()))]
        pub output_file: String,
        /// the plain-text title of your novel
         #[arg(short, long, default_value_t = ("My Latest Novel".to_string()))]
        pub title_text: String,
    }

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
