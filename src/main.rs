use std::{
   collections::BTreeMap,
   path::{
      Path,
      PathBuf,
   },
};

use config::Config;
use zellij_tile::prelude::*;

mod config;
mod dirlist;
mod filter;
mod textinput;
use dirlist::DirList;
use textinput::TextInput;

const ROOT: &str = "/host";

#[derive(Debug, Default)]
struct State {
   dirlist:         DirList,
   cwd:             PathBuf,
   textinput:       TextInput,
   current_session: String,

   config: Config,
   rows:   usize,
}

fn matches_key(
   key: &KeyWithModifier,
   bare_key: BareKey,
   modifiers: Option<&[KeyModifier]>,
) -> bool {
   if key.bare_key != bare_key {
      return false;
   }
   modifiers.map_or(key.key_modifiers.is_empty(), |mods| key.has_modifiers(mods))
}

register_plugin!(State);

impl State {
   fn change_root(&self, path: &Path) -> Option<PathBuf> {
      path.strip_prefix(ROOT).ok().map(|p| self.cwd.join(p))
   }

   fn switch_session_with_cwd(&self, dir: &Path) -> Result<(), String> {
      let session_name = dir
         .file_name()
         .and_then(|n| n.to_str())
         .ok_or_else(|| format!("invalid session path: {}", dir.display()))?;
      let host_layout_path = PathBuf::from(ROOT)
         .join(
            dir.strip_prefix("/")
               .map_err(|_| format!("expected absolute path: {}", dir.display()))?,
         )
         .join("layout.kdl");
      let layout = if host_layout_path.exists() {
         LayoutInfo::File(
            host_layout_path
               .to_str()
               .ok_or_else(|| format!("non-UTF-8 layout path: {}", host_layout_path.display()))?
               .into(),
         )
      } else {
         self.config.layout.clone()
      };
      // Switch session will panic if the session is the current session
      if session_name != self.current_session {
         switch_session_with_layout(Some(session_name), layout, Some(dir.to_path_buf()));
      }
      Ok(())
   }

   fn make_dirlist(&mut self, paths: &[(PathBuf, Option<FileMetadata>)]) -> Vec<String> {
      let show_hidden = self.config.show_hidden.clone();
      paths
         .iter()
         .filter(|(p, _)| p.is_dir() && !is_hidden(p, &show_hidden))
         .filter_map(|(p, _)| {
            if p.starts_with(ROOT) {
               self.change_root(p)
            } else {
               Some(p.to_path_buf())
            }
         })
         .map(|p| p.to_string_lossy().to_string())
         .collect()
   }
}

impl ZellijPlugin for State {
   fn load(&mut self, configuration: BTreeMap<String, String>) {
      self.cwd = get_plugin_ids().initial_cwd;
      self.config = Config::from(configuration);
      request_permission(&[
         PermissionType::RunCommands,
         PermissionType::ChangeApplicationState,
         PermissionType::ReadApplicationState,
      ]);
      subscribe(&[
         EventType::Key,
         EventType::FileSystemUpdate,
         EventType::SessionUpdate,
      ]);
      self.dirlist.reset();
      self.textinput.reset();
      let host = PathBuf::from(ROOT);
      for dir in &self.config.root_dirs {
         let relative_path = match dir.strip_prefix(self.cwd.as_path()) {
            Ok(p) => p,
            Err(_) => continue,
         };
         let host_path = host.join(relative_path);
         scan_host_folder(&host_path);
      }
      let show_hidden = &self.config.show_hidden;
      let individual_dirs: Vec<String> = self
         .config
         .individual_dirs
         .iter()
         .filter(|p| !is_hidden(p, show_hidden))
         .map(|p| p.to_string_lossy().to_string())
         .collect();
      self.dirlist.update_dirs(individual_dirs);
   }

   fn update(&mut self, event: Event) -> bool {
      let mut should_render = false;
      match event {
         Event::FileSystemUpdate(paths) => {
            let dirs = self.make_dirlist(&paths);
            self.dirlist.update_dirs(dirs);
            should_render = true;
         },
         Event::SessionUpdate(sessions, _) => {
            for session in sessions.iter() {
               if session.is_current_session {
                  self.current_session = session.name.clone();
                  break;
               }
            }
            should_render = true;
         },
         Event::Key(key) => {
            should_render = true;
            match key {
               k if matches_key(&k, BareKey::Esc, None)
                  || matches_key(&k, BareKey::Char('c'), Some(&[KeyModifier::Ctrl])) =>
               {
                  close_self()
               },

               k if matches_key(&k, BareKey::Up, None)
                  || matches_key(&k, BareKey::Tab, Some(&[KeyModifier::Shift]))
                  || matches_key(&k, BareKey::Char('p'), Some(&[KeyModifier::Ctrl])) =>
               {
                  self.dirlist.handle_up();
               },

               k if matches_key(&k, BareKey::Down, None)
                  || matches_key(&k, BareKey::Tab, None)
                  || matches_key(&k, BareKey::Char('n'), Some(&[KeyModifier::Ctrl])) =>
               {
                  self.dirlist.handle_down();
               },

               k if matches_key(&k, BareKey::Char('u'), Some(&[KeyModifier::Ctrl])) => {
                  self.dirlist.handle_half_page_up(self.rows);
               },

               k if matches_key(&k, BareKey::Char('d'), Some(&[KeyModifier::Ctrl])) => {
                  self.dirlist.handle_half_page_down(self.rows);
               },

               k if matches_key(&k, BareKey::Enter, None) => {
                  if let Some(selected) = self.dirlist.get_selected() {
                     let _ = self.switch_session_with_cwd(Path::new(&selected));
                     close_self();
                  }
               },

               k if matches_key(&k, BareKey::Backspace, None) => {
                  self.textinput.handle_backspace();
                  self.dirlist.set_search_term(self.textinput.get_text());
               },

               k if matches_key(&k, BareKey::Char('w'), Some(&[KeyModifier::Ctrl])) => {
                  self.textinput.handle_delete_word();
                  self.dirlist.set_search_term(self.textinput.get_text());
               },

               KeyWithModifier {
                  bare_key: BareKey::Char(c),
                  ..
               } => {
                  self.textinput.handle_char(c);
                  self.dirlist.set_search_term(self.textinput.get_text());
               },

               _ => (),
            }
         },
         _ => (),
      }
      should_render
   }

   fn render(&mut self, rows: usize, cols: usize) {
      self.rows = rows.saturating_sub(4);
      println!();
      self.dirlist.render(self.rows, cols);
      println!();
      self.textinput.render(rows, cols);
      println!();
   }
}

fn is_hidden(path: &Path, show_hidden: &[PathBuf]) -> bool {
   let name = path.file_name().and_then(|s| s.to_str());
   name.map_or(false, |s| {
      s.starts_with('.') && !show_hidden.iter().any(|d| d == Path::new(s))
   })
}
