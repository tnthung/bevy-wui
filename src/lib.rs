use tap::prelude::*;
use bevy::prelude::*;


/// The plugin for enabling the webview embedment in the Bevy window.
#[derive(Debug, Clone, Default)]
pub struct WuiPlugin {

}


impl Plugin for WuiPlugin {
  fn build(&self, app: &mut App) {

  }
}


pub mod prelude {
  pub use crate::WuiPlugin;
}
