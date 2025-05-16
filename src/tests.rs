// ---- test_driven_design
#[cfg(test)]
mod suite {
    use crate::{
        error_handling::AppErrors,
        structs::{AppCfg, Book, Chapter, Part, Scene, Title},
        traits::{AppCfgImpls, AppCfgWG, BookImpls as _, ChapterImpls, PartImpls, SceneImpls},
    };

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

        new_book = Book::sort_part_list(&new_book);

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
        new_book = Book::sort_part_list(&new_book);

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
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&format!("{}", AppErrors::ValidChapterList))
                .title
                .sort_by,
            "01-AA== Chapter 1 - the First Chapter"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&format!("{}", AppErrors::ValidChapterList))
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
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&format!("{}", AppErrors::ValidChapterList))
                .title
                .display_by,
            "Chapter 1 - the First Chapter"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .last()
                .expect(&format!("{}", AppErrors::ValidChapterList))
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

        new_scene1.title.sort_by = "01-AA== The Big First Scene.md".into();
        new_scene1.title.display_by = Scene::smart_title(&new_scene1.title.sort_by).into();

        new_chapter1.scene_list.push(new_scene1);
        new_part1.chapter_list.push(new_chapter1);
        new_book.part_list.push(new_part1);

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&format!("{}", AppErrors::ValidChapterList))
                .scene_list
                .first()
                .expect(&format!("{}", AppErrors::ValidSceneList))
                .title
                .sort_by,
            "01-AA== The Big First Scene.md"
        );

        assert_eq!(
            new_book
                .part_list
                .first()
                .expect(&format!("{}", AppErrors::ValidPartList))
                .chapter_list
                .first()
                .expect(&format!("{}", AppErrors::ValidChapterList))
                .scene_list
                .first()
                .expect(&format!("{}", AppErrors::ValidSceneList))
                .title
                .display_by,
            "The Big First Scene"
        );
    }

    #[test]
    fn chapters_must_sort_scenes() {
        let mut new_chapter1 = Chapter::new();
        let mut new_scene1 = Scene::new();
        let mut new_scene2 = Scene::new();
        let mut new_scene3 = Scene::new();

        new_chapter1.title.sort_by = "01-AA== Chapter 1 - the First Chapter".into();
        new_chapter1.title.display_by = Chapter::smart_title(&new_chapter1.title.sort_by).into();

        new_scene1.title.sort_by = "03-BB== 3 Scene".into();
        new_scene1.title.display_by = Scene::smart_title(&new_scene1.title.sort_by).into();

        new_scene2.title.sort_by = "01-AA== The Big First Scene".into();
        new_scene2.title.display_by = Scene::smart_title(&new_scene2.title.sort_by).into();

        new_scene3.title.sort_by = "01-Ab== 2nd Scene".into();
        new_scene3.title.display_by = Scene::smart_title(&new_scene3.title.sort_by).into();

        new_chapter1.scene_list.push(new_scene1);
        new_chapter1.scene_list.push(new_scene2);
        new_chapter1.scene_list.push(new_scene3);
        new_chapter1 = Chapter::sort_scene_list(new_chapter1);

        assert_eq!(
            new_chapter1
                .scene_list
                .first()
                .expect(&format!("{}", AppErrors::ValidSceneList))
                .title
                .display_by,
            "The Big First Scene",
        );

        assert_eq!(
            new_chapter1
                .scene_list
                .last()
                .expect(&format!("{}", AppErrors::ValidSceneList))
                .title
                .display_by,
            "3 Scene",
        );
    } //fn chapters_must_sort_scenes()

    #[test]
    fn app_can_open_path_and_read_structure() {
        let content_path_element_count = 38;
        let mut my_app = AppCfg::new();
        my_app.content_path = "./content".into();
        my_app.output_file = "my_book_title".into();

        let my_path_elements = AppCfg::get_path_elements(&my_app.content_path.into());
        assert_eq!(content_path_element_count, my_path_elements.len());
    }

    #[test]
    fn app_can_load_path_structure_into_book_structure() {
        let expected_number_parts = 3;
        let expected_number_chapters_p1 = 2;
        let expected_number_scenes_p1ch1 = 5;

        let mut my_app = AppCfg::new();
        my_app.content_path = "./content".into();
        let path_elm = AppCfg::get_path_elements(&my_app.content_path.clone());

        let mut this_book = Book::new();

        for book_piece in &path_elm {
            this_book.add_content(&my_app, book_piece);
        }

        this_book = Book::sort_part_list(&this_book);
        assert_eq!(expected_number_parts, this_book.part_list.len());

        assert_eq!(
            expected_number_chapters_p1,
            this_book.part_list[1].chapter_list.len()
        );

        assert_eq!(
            expected_number_scenes_p1ch1,
            this_book.part_list[1].chapter_list[0].scene_list.len()
        );

        assert_eq!(
            "Part 1 - Fourteen Weeks Later",
            this_book.part_list[1].title.sort_by
        );

        assert_eq!(
            "Ch 1 - Nothing To See, Hear",
            this_book.part_list[1].chapter_list[0].title.sort_by
        );

        assert_eq!(
            "Ch 2 - Deja Voodoo",
            this_book.part_list[1].chapter_list[1].title.sort_by
        );

        let content_blob = this_book
            .part_list
            .last()
            .expect(&format!("{}", AppErrors::ValidPartList))
            .chapter_list
            .last()
            .expect(&format!("{}", AppErrors::ValidChapterList))
            .scene_list
            .last()
            .expect(&format!("{}", AppErrors::ValidSceneList))
            .content
            .clone();

        let expected_excerpt = "ty tonight.\"\r".to_string();
        let test_stop = content_blob.len() - 1;
        let test_start = test_stop - 13;
        assert_eq!(expected_excerpt, content_blob[test_start..test_stop]);
    }

    #[test]
    fn app_can_write_toc_to_disk_file() {
        let mut my_app = AppCfg::new();
        my_app.content_path = "./content".into();
        my_app.output_file = "my_book_title".into();
        let path_elm = AppCfg::get_path_elements(&my_app.content_path.clone());

        let mut this_book = Book::new();
        for dir_entry in &path_elm {
            this_book.add_content(&my_app, dir_entry);
        }

        use std::fs;
        use std::fs::File;
        use std::path::Path;

        let path_string = &format!("{}/../{}.md", &my_app.content_path, &my_app.output_file);
        let work_path = Path::new(path_string);
        let mut work_file = File::create(work_path).expect(&format!("{}", AppErrors::VaildPath));

        Book::write_toc(this_book.clone(), &mut work_file);
        Book::write_content(this_book, &mut work_file);

        let read_buffer =
            fs::read_to_string(path_string).expect(&format!("{}", AppErrors::ReadableFile));

        assert!(work_path.exists());
        assert!(read_buffer.contains("# Table of Contents\r\n"));
        assert!(read_buffer.contains("## Part 2 - Dogs of War\r\n"));
        assert!(read_buffer.contains("### Ch 1 - Nothing To See, Hear\r\n"));
        assert!(read_buffer.contains("---\r\n"));
        assert!(read_buffer.contains("\"You got statements from witnesses?\""));
        assert!(read_buffer.contains("Then he'd met Sharlene, and then Jeanie and Rosie."));
        assert!(read_buffer.contains("\"I *told* you that mine was a special kind of evil.\""));
    } // fn app_can_write_TOC_to_disk_file

    #[test]
    fn dni_content_tag_on_folders_respected() {
        let mut my_app = AppCfg::new();
        my_app.content_path = "./content".into();
        let path_elm = AppCfg::get_path_elements(&my_app.content_path.clone());

        let mut this_book = Book::new();

        for dir_entry in &path_elm {
            this_book.add_content(&my_app, dir_entry);
        }

        let mut dni_content_found = false;
        let dni_content_searched = "#Cover Art Image";
        this_book.part_list.into_iter().for_each(|part| {
            part.chapter_list.into_iter().for_each(|chapter| {
                chapter.scene_list.into_iter().for_each(|scene| {
                    if scene.content.contains(dni_content_searched) {
                        dni_content_found = true;
                    }
                }); // for scene
            }); //for chapter
        }); //for part

        assert!(!dni_content_found)
    } // fn dni_content_tag_on_folders_respected
} // mod tests
