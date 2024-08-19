// ---- modules
// mod error_handling;
// mod implementations;
// mod structs;
// mod traits;
//
// use crate::implementations::List::*;
//

#[cfg(test)]
mod test_driven_design {
    use super::*;
    use crate::error_handling::*;
    use crate::structs::List::*;
    use crate::traits::List::*;

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

    #[test]
    fn book_must_have_parts() {
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
        assert_eq!(
            new_book.part_list[2].title.sort_by,
            "003 - the tricky Third Part"
        );
    }

    #[test]
    fn book_must_sort_parts() {
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
        assert_eq!(
            new_book.part_list[2].title.display_by,
            "the tricky Third Part"
        );

        assert_eq!(new_book.part_list[3].title.sort_by, "003b");
        assert_eq!(
            new_book.part_list[3].title.display_by,
            "the tricky Fourth Part"
        );
    }

    #[test]
    fn book_part_titles_smart_human_defaults() {
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_part2 = Part::new();
        new_part2.title.sort_by = "0A0B== Part 1 - the First Part".into();
        new_part2.title.display_by = Part::smart_title(&new_part2.title.sort_by).into();
        new_part1.title.sort_by = "0A0C== Part 2 - the Second Part".into();
        new_part1.title.display_by = Part::smart_title(&new_part1.title.sort_by).into();

        new_book.part_list.push(new_part1);
        new_book.part_list.push(new_part2);
        new_book = Book::sort_part_list(new_book);

        assert_eq!(
            new_book.part_list[0].title.display_by,
            "Part 1 - the First Part"
        );
        assert_eq!(
            new_book.part_list[1].title.display_by,
            "Part 2 - the Second Part"
        );
    }

    #[test]
    fn book_parts_must_have_chapters_with_smart_human_titles() {
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_chapter1 = Chapter::new();

        new_part1.title.sort_by = "0A0B== Part 1 - the First Part".into();
        new_part1.title.display_by = Part::smart_title(&new_part1.title.sort_by).into();

        new_chapter1.title.sort_by = "01-AA== Chapter 1 - the First Chapter".into();
        new_chapter1.title.display_by = Chapter::smart_title(&new_chapter1.title.sort_by).into();

        new_part1.chapter_list.push(new_chapter1);
        new_book.part_list.push(new_part1);

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .title
                .sort_by,
            "01-AA== Chapter 1 - the First Chapter"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .title
                .display_by,
            "Chapter 1 - the First Chapter"
        );
    }

    #[test]
    fn book_parts_must_sort_chapters() {
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_chapter1 = Chapter::new();
        let mut new_chapter2 = Chapter::new();
        let mut new_chapter3 = Chapter::new();

        new_part1.title.sort_by = "0A0B== Part 1 - the First Part".into();
        new_part1.title.display_by = Part::smart_title(&new_part1.title.sort_by).into();

        new_chapter3.title.sort_by = "01-AA== Chapter 1 - the First Chapter".into();
        new_chapter3.title.display_by = Chapter::smart_title(&new_chapter3.title.sort_by).into();

        new_chapter1.title.sort_by = "01-AB== Chapter 2 - the Second Chapter".into();
        new_chapter1.title.display_by = Chapter::smart_title(&new_chapter1.title.sort_by).into();

        new_chapter2.title.sort_by = "03-AA== Chapter 3 - the Third Chapter".into();
        new_chapter2.title.display_by = Chapter::smart_title(&new_chapter2.title.sort_by).into();

        new_part1.chapter_list.push(new_chapter1);
        new_part1.chapter_list.push(new_chapter2);
        new_part1.chapter_list.push(new_chapter3);

        new_part1 = Part::sort_chapter_list(new_part1);
        new_book.part_list.push(new_part1);

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .title
                .display_by,
            "Chapter 1 - the First Chapter"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .last()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .title
                .display_by,
            "Chapter 3 - the Third Chapter"
        );
    }

    #[test]
    fn chapters_must_have_scenes_with_smart_human_titles() {
        let mut new_book = Book::new();
        let mut new_part1 = Part::new();
        let mut new_chapter1 = Chapter::new();
        let mut new_scene1 = Scene::new();

        new_part1.title.sort_by = "0A0B== Part 1 - the First Part".into();
        new_part1.title.display_by = Part::smart_title(&new_part1.title.sort_by).into();

        new_chapter1.title.sort_by = "01-AA== Chapter 1 - the First Chapter".into();
        new_chapter1.title.display_by = Chapter::smart_title(&new_chapter1.title.sort_by).into();

        new_scene1.title.sort_by = "01-AA== The Big First Scene".into();
        new_scene1.title.display_by = Scene::smart_title(&new_scene1.title.sort_by).into();

        new_chapter1.scene_list.push(new_scene1);
        new_part1.chapter_list.push(new_chapter1);
        new_book.part_list.push(new_part1);

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .scene_list
                .first()
                .expect(&getExpected(AppErrors::ValidSceneList))
                .title
                .sort_by,
            "01-AA== The Big First Scene"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&getExpected(AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&getExpected(AppErrors::ValidChapterList))
                .scene_list
                .first()
                .expect(&getExpected(AppErrors::ValidSceneList))
                .title
                .display_by,
            "The Big First Scene"
        );
    }
} // mod tests
