use std::{
   collections::BTreeMap,
   path::PathBuf,
};

use zellij_tile::prelude::LayoutInfo;

use super::ROOT;

#[derive(Debug)]
pub struct Config {
   pub root_dirs:       Vec<PathBuf>,
   pub individual_dirs: Vec<PathBuf>,
   pub show_hidden:     Vec<PathBuf>,
   pub layout:          LayoutInfo,
}

impl Default for Config {
   fn default() -> Self {
      Self {
         root_dirs:       vec![PathBuf::from(ROOT)],
         individual_dirs: vec![],
         show_hidden:     vec![PathBuf::from(".config")],
         layout:          LayoutInfo::BuiltIn("default".to_owned()),
      }
   }
}

fn parse_layout(layout: &str) -> LayoutInfo {
   // builtin: ":default" custom: "default"
   if layout.starts_with(':') {
      LayoutInfo::BuiltIn(layout.trim_start_matches(':').to_owned())
   } else {
      LayoutInfo::File(layout.to_owned())
   }
}

fn parse_dirs(dirs: &str) -> Vec<PathBuf> {
   dirs.split(';').map(PathBuf::from).collect()
}

impl From<BTreeMap<String, String>> for Config {
   fn from(config: BTreeMap<String, String>) -> Self {
      let root_dirs: Vec<PathBuf> = config.get("root_dirs").map_or_else(
         || vec![PathBuf::from(ROOT)],
         |root_dirs| parse_dirs(root_dirs),
      );

      let individual_dirs: Vec<PathBuf> = config
         .get("individual_dirs")
         .map_or_else(Vec::new, |individual_dirs| parse_dirs(individual_dirs));

      let show_hidden: Vec<PathBuf> = config.get("show_hidden").map_or_else(
         || vec![PathBuf::from(".config")],
         |show_hidden| parse_dirs(show_hidden),
      );

      let layout = config.get("session_layout").map_or_else(
         || LayoutInfo::BuiltIn("default".to_owned()),
         |layout| parse_layout(layout),
      );

      Self {
         root_dirs,
         individual_dirs,
         show_hidden,
         layout,
      }
   }
}
