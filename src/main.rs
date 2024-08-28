//--- start of file ---
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

// ---- need walkdir for directory traversal
extern crate walkdir;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

// ---- modules
mod error_handling;
mod implementations;
mod structs;
mod tdd;
mod traits;

use crate::error_handling::AppErrors::*;
use crate::error_handling::*;
use crate::implementations::List::*;
use crate::structs::List::*;
use crate::traits::List::*;

// --- the real work to be done
fn main() {
    // ---- will be incoming ARGs
    let content_path = "./content";
    let output_file = "my_book_title";

    // ---- main code
    //   let mut project = Book::new();
    //   let mut write_ptr =
    //       File::create(format!("{}.md", output_file))
    //       .expect(&getErr(appErrors::noVaildPath));

        //    if path_elemets.len() == 3 {
        //        let found_part = path_elemets[0];
        //        let found_chapter = path_elemets[1];

        //          if project.is_a_new_part(found_part) {
        //              let new_part = Part::new();
        //              new_part.title.sort_by = found_part.to_string();
        // -- set the reader-friendly title for the part
        //              project.part_list.push(new_part);

        //          }

        //          let mut current_part = project.for_part(found_part);
        //          if current_part.is_a_new_chapter(found_chapter) {
        //              let new_chapter = Chapter::new();
        //              new_chapter.title.sort_by = found_chapter;
        // -- set the reader-friendly title for the chapter
        //              current_part.part_list.push(new_chapter);
        //          }
        //    }

        // --- writitng to disk
        // --- --- sort part List
        // --- --- loop part List
        // --- --- --- sort chapter List
        // --- --- --- loop chapter List
        // --- --- --- --- sort scene List
        // --- --- --- --- loop scene List
        // --- --- --- --- first_time print_part_title
        // --- --- --- --- first_time print_chapter_title
        // --- --- --- --- print scene_title
        // --- --- --- --- print scene_text

}
