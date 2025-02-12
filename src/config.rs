use bevy::prelude::KeyCode;


/// The configuration for the devtools.
///
/// Default: `DevTools::Debug`
#[derive(Debug, Clone, Copy, Default)]
pub enum DevTools {
  /// Always allow the devtools, even in release builds.
  Always,
  /// Allow the devtools only in debug builds.
  #[default]
  Debug,
  /// Never allow the devtools.
  Never,
}


impl DevTools {
  /// Check if the devtools is enabled.
  pub fn is_enabled(&self) -> bool {
    match self {
      DevTools::Always => true,
      DevTools::Debug  => cfg!(debug_assertions),
      DevTools::Never  => false,
    }
  }
}


/// The configuration for the context menu.
///
/// Default: `ContextMenu::Debug(None)`
#[derive(Debug, Clone, Copy)]
pub enum ContextMenu {
  /// Always allow the context menu, even in release builds. (Optionally, activate when custom key pressed)
  Always(Option<KeyCode>),
  /// Allow the context menu only in debug builds. (Optionally, activate when custom key pressed)
  Debug(Option<KeyCode>),
  /// Never allow the context menu.
  Never,
}


impl ContextMenu {
  /// Check if the context menu is enabled.
  pub fn is_enabled(&self) -> Option<Option<KeyCode>> {
    match self {
      ContextMenu::Always(key) => Some(key.clone()),
      ContextMenu::Debug(key)  => if cfg!(debug_assertions) { Some(key.clone()) } else { None },
      ContextMenu::Never       => None,
    }
  }
}


impl Default for ContextMenu {
  fn default() -> Self {
    ContextMenu::Debug(None)
  }
}
