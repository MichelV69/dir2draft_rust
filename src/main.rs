//--- start of file ---
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

// ---- need walkdir for directory traversal
extern crate walkdir;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    let args = Cli::parse();
    println!(
        "content path: {:?}, output file: {:?}, book title: '{:?}'",
        args.content_path, args.output_file, args.title_text
    );

    // todo!("app get args for book title.to_display");

    let mut my_app = AppCfg::new();
    my_app.content_path = args.content_path;
    my_app.output_file = args.output_file;
    let path_elm = AppCfg::get_path_elements(&my_app.content_path.clone());

    let mut this_book = Book::new();
    this_book.title.display_by = args.title_text;
    for dir_entry in &path_elm {
        this_book.add_content(&my_app, dir_entry);
    }

    let path_string = &format!("{}/../{}.md", &my_app.content_path, &my_app.output_file);
    let work_path = Path::new(path_string);
    let mut work_file = File::create(work_path).expect(&format!("{}", AppErrors::VaildPath));

    Book::write_toc(this_book.clone(), &mut work_file);
    Book::write_content(this_book, &mut work_file);
}
