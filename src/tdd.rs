// ---- test_driven_design
#[cfg(test)]
mod test_driven_design {
    // ---- need walkdir for directory traversal
    extern crate walkdir;
    use std::fs::File;
    use std::io::Write;
    use walkdir::WalkDir;

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

    #[test]
    fn app_can_open_path_and_read_structure() {
        let mut my_app = App::new();
        my_app.content_path = "./content".into();
        my_app.output_file = "my_book_title".into();

        let my_path_elements = App::get_path_elements(&my_app.content_path.into());
        println!("{:#?}", my_path_elements);
        assert_eq!(13, my_path_elements.len());
    }

    #[test]
    fn app_can_load_path_structure_into_book_structure() {
        let mut my_app = App::new();
        my_app.content_path = "./content".into();
        my_app.output_file = "my_book_title".into();

        let mut this_book = Book::new();

        let my_path_elements = App::get_path_elements(&my_app.content_path.clone().into());

        let mut part_index: usize = 0;
        let mut chapter_index: usize = 0;

        for dir_entry in my_path_elements {
            if part_index == 0
                && !dir_entry.contains(".md") {
                let mut to_add = Part::new();
                to_add.title.sort_by = dir_entry.clone();
                to_add.title.display_by = Scene::smart_title(&to_add.title.sort_by).into();
                this_book.part_list.push(to_add);
            }

            if part_index > 0
                && chapter_index == 0
                && !dir_entry.contains(".md") {
                let mut to_add = Chapter::new();
                to_add.title.sort_by = dir_entry.clone();
                to_add.title.display_by = Scene::smart_title(&to_add.title.sort_by).into();
                this_book.part_list[part_index].chapter_list.push(to_add);
            }

            if part_index > 0
                && chapter_index > 0
                && dir_entry.contains(".md") {
    //            part_index = this_book.find_part(&current_part);
    //            chapter_index = this_book.part_list[part_index]
    //                .find_chapter(&current_chapter)
    //                .expect(&getExpected(AppErrors::ValidPartIndex));

                let mut this_scene = Scene::new();
                this_scene.title.sort_by = dir_entry.clone();
                this_scene.title.display_by = Scene::smart_title(&this_scene.title.sort_by).into();
                this_scene.content = Scene::get_content_for(my_app.content_path.clone(), &dir_entry);

                this_book.part_list[part_index]
                    .chapter_list[chapter_index]
                    .scene_list.push(this_scene);
            }// do we see a File (some_cool_scene.md)?

            // do we see a chapter? ("Ch 1 - Nothing To See, Hear")

            // do we see a part? ("Part 1 - Fourteen Weeks Later")
        }

        assert_eq!(true, false);
    }
} // mod tests
