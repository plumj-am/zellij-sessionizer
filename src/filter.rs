use nucleo_matcher::{
   Matcher,
   pattern::{
      CaseMatching,
      Normalization,
      Pattern,
   },
};

// from https://docs.rs/nucleo-matcher/0.3.1/nucleo_matcher/
//
// let paths = ["foo/bar", "bar/foo", "foobar"];
// let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
// let matches = Pattern::parse("foo bar", CaseMatching::Ignore,
// Normalization::Smart).match_list(paths, &mut matcher); assert_eq!(matches,
// vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]); let matches =
// Pattern::parse("^foo bar", CaseMatching::Ignore,
// Normalization::Smart).match_list(paths, &mut matcher); assert_eq!(matches,
// vec![("foo/bar", 168), ("foobar", 140)]);

pub fn fuzzy_filter(items: &[String], search_term: &str, matcher: &mut Matcher) -> Vec<String> {
   let mut results = Pattern::parse(search_term, CaseMatching::Ignore, Normalization::Smart)
      .match_list(items, matcher);
   results.sort_by_key(|a| a.1);
   results.into_iter().map(|(item, _)| item.clone()).collect()
}

#[cfg(test)]
mod tests {
   use nucleo_matcher::Config;

   use super::*;

   #[test]
   fn test_fuzzy_filter() {
      let items: Vec<String> = vec![
         "/home/laperlej/Projects/bioblend",
         "/home/laperlej/Projects/backup-rotation",
         "/home/laperlej/Projects/github.io",
      ]
      .into_iter()
      .map(|item| item.to_string())
      .collect();
      let search_term = "bio";
      let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
      let result = fuzzy_filter(&items, search_term, &mut matcher);
      assert_eq!(result, vec!["/home/laperlej/Projects/bioblend"]);
   }
}
