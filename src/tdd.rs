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
        let result: Title = Title {sort_by : "001".into(), display : "The Title".into()};
        assert_eq!(result.sort_by, "001");
    }

    #[test]
    fn title_new(){
        let result = Title::new();
        assert_eq!(result.sort_by, "001 - The Title");
        assert_eq!(result.display, "The Title");
    }

} // mod tests
