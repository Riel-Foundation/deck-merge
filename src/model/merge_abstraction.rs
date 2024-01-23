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
    }
  }
  pub fn construct_from_string(string: String, sorting: u32) -> MergeAbstraction {
    return Self::construct(string, sorting as i64);
  }
  pub fn construct_base(string: String) -> MergeAbstraction {
    return Self::construct(string, -1);
  }
}