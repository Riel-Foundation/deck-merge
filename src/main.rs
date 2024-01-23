#![allow(dead_code, unused_imports)]
mod model;
use model::merge_abstraction::MergeAbstraction;
#[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn abstraction_line_count() {
            let test_string: String = "Hello\nWorld\n".to_string();
            let test_abstraction: MergeAbstraction = MergeAbstraction::construct_base(test_string);
            assert_eq!(test_abstraction.line_count, 2);
        }
        #[test]
        fn merge_trivial_empty() {
            // Base represents all aces of a deck of poker cards.
            // Imagine in alfa, we add the nine of spades.
            // Imagine in bravo, we simply remove the Ace of Diamonds.
            let base: String = "AceHearts\nAceSpades\nAceClubs\nAceDiamonds\n".to_string();
            let alfa: String = "AceHearts\nAceSpades\nAceClubs\nAceDiamonds\nNineSpades\n".to_string();
            let bravo: String = "AceHearts\nAceSpades\nAceClubs\n".to_string();
            let result: String = merge(base, alfa, bravo);
            assert_eq!(result, "AceHearts\nAceSpades\nAceClubs\nNineSpades\n");
        }
    }
fn main() {
    println!("Deck-Merge, from Riel-Foundation");
}
/**
 * Merge returns a string in which all lines of base are present,
 * except the ones that got removed in alfa and bravo,
 * and also all lines that got addded in alfa and bravo. 
 */
fn merge(base: String, alfa: String, bravo: String) -> String {
  todo!()
}