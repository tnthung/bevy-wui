mod queue;
mod config;
mod webview;

use tap::prelude::*;
use bevy::prelude::*;


/// The plugin for enabling the webview embedment inside the Bevy window.
#[derive(Debug, Default)]
pub struct WuiPlugin;


impl Plugin for WuiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, webview::sys_create_webview)
      .add_systems(PreUpdate, webview::sys_update_webview)
      .add_systems(PreUpdate, webview::sys_remove_webview)
      .insert_non_send_resource(webview::Webviews::default())
  ;}
}


pub mod prelude {
  pub use crate::WuiPlugin;
  pub use crate::config::*;
  pub use crate::webview::Webview;
}
