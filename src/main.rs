#![allow(dead_code, unused_imports)]
mod model;
use model::merge_abstraction::MergeAbstraction;
#[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_line_count() {
            let test_string: String = "Hello\nWorld\n".to_string();
            let test_abstraction: MergeAbstraction = MergeAbstraction::construct_base(test_string);
            assert_eq!(test_abstraction.line_count, 2);
        }
    }
fn main() {
    println!("Deck-Merge, from Riel-Foundation");
}