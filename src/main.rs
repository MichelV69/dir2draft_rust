extern crate walkdir;
use walkdir::WalkDir;
fn main() {

    let content_path = "./content";

    for e in WalkDir::new(content_path).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            if e.path().contains(".md") {
                println!("{}", e.path().display());
            }

        }
    }
}
