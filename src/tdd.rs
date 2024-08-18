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
        let book_title = Title {
                sort_by: "sort_by_book_title".into(),
                display_by: "display_by_book_title".into(),
            };

        let part1_title = Title {
                sort_by: "sort_by_part1".into(),
                display_by: "display_by_part1".into(),
            };

        let part2_title = Title {
                sort_by: "sort_by_part2".into(),
                display_by: "display_by_part2".into(),
            };

        let part1 :Part = Part {title: part1_title};
        let part2 :Part = Part {title: part2_title};

        let result: Book = Book {
            title: book_title ,
            part_list: vec![part1, part2],
        };
        assert_eq!(result.title.sort_by, "sort_by_book_title");
        assert_eq!(result.title.display_by, "display_by_book_title");
    }

    #[test]
    fn book_new() {
        let result = Book::new();
        assert_eq!(result.title.sort_by, "001 - The Title");
        assert_eq!(result.title.display_by, "The Title");
    }

     #[test]
    fn book_change() {
        let mut result = Book::new();
        let new_sort_by = "001 - My New Book";
        let new_display_by = "The New Book Title";

        result.title.sort_by = new_sort_by.into();
        result.title.display_by = new_display_by.into();

        assert_eq!(result.title.sort_by, new_sort_by);
        assert_eq!(result.title.display_by, new_display_by);
    }

    #[ test]
    fn book_must_have_parts(){
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_part2 = Part::new();
        let mut new_part3 = Part::new();

        new_part1.title.sort_by = "001 - the First Part".into();
        new_part2.title.sort_by = "002 - the Second Part".into();
        new_part3.title.sort_by = "003 - the tricky Third Part".into();

        new_book.part_list.push(new_part1);
        new_book.part_list.push(new_part2);
        new_book.part_list.push(new_part3);

        assert_eq!(new_book.part_list[0].title.sort_by, "001 - the First Part");
        assert_eq!(new_book.part_list[1].title.sort_by, "002 - the Second Part");
        assert_eq!(new_book.part_list[2].title.sort_by, "003 - the tricky Third Part");
    }

    #[test]
    fn book_must_sort_parts(){
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_part2 = Part::new();
        let mut new_part3 = Part::new();
        let mut new_part4 = Part::new();

        new_part2.title.sort_by = "001".into();
        new_part2.title.display_by = "the First Part".into();
        new_part3.title.sort_by = "002".into();
        new_part3.title.display_by = "the Second Part".into();
        new_part4.title.sort_by = "003a".into();
        new_part4.title.display_by = "the tricky Third Part".into();
        new_part1.title.sort_by = "003b".into();
        new_part1.title.display_by = "the tricky Fourth Part".into();

        new_book.part_list.push(new_part1);
        new_book.part_list.push(new_part2);
        new_book.part_list.push(new_part3);
        new_book.part_list.push(new_part4);

        new_book = Book::sort_part_list(new_book);

       assert_eq!(new_book.part_list[0].title.sort_by, "001");
       assert_eq!(new_book.part_list[0].title.display_by, "the First Part");

       assert_eq!(new_book.part_list[1].title.sort_by, "002");
       assert_eq!(new_book.part_list[1].title.display_by, "the Second Part");

       assert_eq!(new_book.part_list[2].title.sort_by, "003a");
       assert_eq!(new_book.part_list[2].title.display_by, "the tricky Third Part");

       assert_eq!(new_book.part_list[3].title.sort_by, "003b");
       assert_eq!(new_book.part_list[3].title.display_by, "the tricky Fourth Part");
    }

    #[test]
    fn book_part_titles_smart_human_defaults(){
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_part2 = Part::new();
        new_part2.title.sort_by = "0A0B== Part 1 - the First Part".into();
        new_part2.title.display_by = Book::smart_title(&new_part2.title.sort_by).into();
        new_part1.title.sort_by = "0A0C== Part 2 - the Second Part".into();
        new_part1.title.display_by = Book::smart_title(&new_part1.title.sort_by).into();

        new_book.part_list.push(new_part1);
        new_book.part_list.push(new_part2);
        new_book = Book::sort_part_list(new_book);

        assert_eq!(new_book.part_list[0].title.display_by, "Part 1 - the First Part");
        assert_eq!(new_book.part_list[1].title.display_by, "Part 2 - the Second Part");
    }
} // mod tests
