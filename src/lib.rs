mod queue;
mod config;
mod events;
mod webview;
mod convert;

use tap::prelude::*;
use bevy::prelude::*;


/// The plugin for enabling the webview embedment inside the Bevy window.
#[derive(Debug, Default)]
pub struct WuiPlugin;


impl Plugin for WuiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, (
        webview::sys_create_webview,
        webview::sys_update_webview,
        webview::sys_remove_webview,
        events ::sys_webview_events,
      ).chain())
      .insert_non_send_resource(webview::Webviews::default())
  ;}
}


pub mod prelude {
  pub use crate::WuiPlugin;
  pub use crate::config::*;
  pub use crate::webview::Webview;
}
