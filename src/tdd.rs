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
        let result: Title = Title {sort_by : "001".into(), display_by : "The Title".into()};
        assert_eq!(result.sort_by, "001");
    }

    #[test]
    fn title_new(){
        let result = Title::new();
        assert_eq!(result.sort_by, "001 - The Title");
        assert_eq!(result.display_by, "The Title");
    }

    #[test]
    fn title_change(){
        let mut result = Title::new();
        let new_sort_by = "001 - My New Book";
        let new_display_by = "The New Book Title";

        result.sort_by = new_sort_by.into();
        result.display_by = new_display_by.into();

        assert_eq!(result.sort_by, new_sort_by);
        assert_eq!(result.display_by, new_display_by);
    }

} // mod tests
