use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct MergeAbstraction {
    pub sorting: i64,
    pub lines: Vec<String>,
    pub line_count: u32,
}
impl MergeAbstraction {
    /**
     * Does not allow for a negative sorting value. For that, see construct_base
     */
    fn construct(string: String, sorting: i64) -> MergeAbstraction {
        return MergeAbstraction {
            sorting: sorting,
            lines: string.lines().map(|line: &str| line.to_string()).collect(),
            line_count: string.lines().count() as u32,
        };
    }
    pub fn construct_from_string(string: String, sorting: u32) -> MergeAbstraction {
        return Self::construct(string, sorting as i64);
    }
    pub fn construct_base(string: String) -> MergeAbstraction {
        return Self::construct(string, -1);
    }
    pub fn construct_sorted(a: &String, b: &String) -> (MergeAbstraction, MergeAbstraction) {
        let hash1: u64 = hash_string(&a);
        let hash2: u64 = hash_string(&b);

        fn hash_string(string: &str) -> u64 {
            let mut hasher = DefaultHasher::new();
            string.hash(&mut hasher);
            hasher.finish()
        }
        let sorting_1: i64 = if hash1 < hash2 { 0 } else { 1 };
        let sorting_2: i64 = if hash1 < hash2 { 1 } else { 0 };
        return (
            Self::construct(a.clone(), sorting_1),
            Self::construct(b.clone(), sorting_2),
        );
    }
}
