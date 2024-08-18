// ---- modules
// mod error_handling;
// mod implementations;
// mod structs;
// mod traits;
//
// use crate::implementations::List::*;
// use crate::structs::List::*;
// use crate::traits::List::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_a_title() {
        let result: Title = {sort_by = "001", display = "The Title"};
        assert_eq!(result.sort_by, "001");
    }
}
