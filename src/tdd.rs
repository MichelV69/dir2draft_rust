// ---- modules
// mod error_handling;
// mod implementations;
// mod structs;
// mod traits;
//
// use crate::implementations::List::*;
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::List::*;
    use crate::traits::List::*;

    #[test]
    fn title_make() {
        let result: Title = Title {
            sort_by: "001".into(),
            display_by: "The Title".into(),
        };
        assert_eq!(result.sort_by, "001");
    }

    #[test]
    fn title_new() {
        let result = Title::new();
        assert_eq!(result.sort_by, "001 - The Title");
        assert_eq!(result.display_by, "The Title");
    }

    #[test]
    fn title_change() {
        let mut result = Title::new();
        let new_sort_by = "001 - My New Book";
        let new_display_by = "The New Book Title";

        result.sort_by = new_sort_by.into();
        result.display_by = new_display_by.into();

        assert_eq!(result.sort_by, new_sort_by);
        assert_eq!(result.display_by, new_display_by);
    }

    #[test]
    fn book_make() {
        let result: Book = Book {
            title: Title {
                sort_by: "sort_by_text".into(),
                display_by: "display_by_text".into(),
            },
        };
        assert_eq!(result.title.sort_by, "sort_by_text");
        assert_eq!(result.title.display_by, "display_by_text");
    }

    #[test]
    fn book_new() {
        let result = Book::new();
        assert_eq!(result.title.sort_by, "001 - The Title");
        assert_eq!(result.title.display_by, "The Title");
    }

    fn book_change() {

    }
} // mod tests
