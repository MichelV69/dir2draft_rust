extern crate walkdir;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{Write};

fn main() {

    let content_path = "./content";
    let output_file = "my_book_title";

    struct Book {
        part : String,
        chapter : String,
        scene : String,
    }

    let mut project = {Book {part: "".into(), chapter: "".into(), scene: "".into()}};
    let mut write_ptr = File::create(format!("{}.md",output_file))
        .expect("ER01 - Expect valid path");

    for entry in WalkDir::new(content_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

        let full_path = entry.path()
            .strip_prefix(content_path).expect("ER01 - Expect valid path")
            .to_string_lossy() ;

        let path_elemets : Vec<_>= full_path.split("/").collect();

        // println!("DEBUG full_path[{}]", full_path);
        // println!("DEBUG #path_elemets[{}]", path_elemets.len());
        // println!("DEBUG path_elemets.0[{}]", path_elemets[0]);

      if path_elemets.len() == 3 {

        let found_part = path_elemets[0];
        let found_chapter = path_elemets[1];

        if project.part != found_part.to_string() {
            project.part = found_part.to_string();

            let output = format!("#{}", project.part);
            println!("{}",output);
            writeln!(write_ptr,  "{} \n", output).expect("ER02 - Expect valid write pointer");
        }
        if project.chapter != found_chapter.to_string() {
            project.chapter = found_chapter.to_string();

            let output = format!("##{}", project.chapter);
            println!("{}",output);
            writeln!(write_ptr,  "{} \n",output).expect("ER02 - Expect valid write pointer");
        }
        project.scene = entry.file_name().to_string_lossy().to_string();
        if project.scene.ends_with(".md") {

            let output = format!("###{}", project.scene);
            println!("{}",output);
            writeln!(write_ptr,  "{} \n",output).expect("ER02 - Expect valid write pointer");
        }

      }

    }
}
