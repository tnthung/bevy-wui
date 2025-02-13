use crate::queue::*;
use crate::config::*;

use bevy::prelude::*;
use bevy::ecs::entity::hash_map::EntityHashMap;
use bevy::winit::WinitWindows;
use wry::raw_window_handle::{HasWindowHandle, WindowHandle};


/// The component representing a webview. Spawning this will create a new webview in the window and
/// removing / despawning it will remove the webview from the window. Spawning this component alone,
/// will create a new default window with a new webview, because the `Window` component is required.
/// To spawn in an existing window, query the window entity and add this component to it.
///
/// # Example
///
/// ```rust, no_run
/// use bevy::prelude::*;
/// use bevy_wui::prelude::*;
///
/// fn add_webview(mut cmds: Commands, wnd: Query<Entity, (With<Window>, Without<Webview>)>) {
///   for wnd in &wnd {
///     cmds.entity(wnd).insert(Webview::default());
///   }
/// }
///
/// fn del_webview(mut cmds: Commands, wnd: Query<Entity, (With<Window>, With<Webview>)>) {
///   for wnd in &wnd {
///     cmds.entity(wnd).remove::<Webview>();
///   }
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, Component)]
#[require(Window)]
pub struct Webview {
  /// The devtools configuration for current webview. \
  /// This option cannot be changed after the webview is created.
  pub devtools: DevTools,
  /// The context menu configuration for current webview.
  pub context_menu: ContextMenu,
}


impl Webview {
  /// Set the devtools configuration.
  pub fn devtools(mut self, devtools: DevTools) -> Self {
    self.devtools = devtools;
    self
  }

  /// Set the context menu configuration.
  pub fn context_menu(mut self, context_menu: ContextMenu) -> Self {
    self.context_menu = context_menu;
    self
  }
}


struct WebviewHandle {
  webview: wry::WebView,
  i_queue: Queue<String>,   // input to webview
  o_queue: Queue<String>,   // output from webview
}


/// Storage for `Entity -> wry::WebView` mapping.
#[derive(Default)]
pub(crate) struct Webviews(EntityHashMap<WebviewHandle>);


pub(crate) fn sys_create_webview(
  mut commands: Commands,
  mut webviews: NonSendMut<Webviews>,
  winit_window: NonSend<WinitWindows>,
  wnd_entities: Query<(Entity, &Webview, &Window), Added<Webview>>,
) {
  fn create_webview(hwnd: &WindowHandle<'_>, config: &Webview) -> wry::Result<WebviewHandle> {
    let i_queue = Queue::default();
    let o_queue = Queue::default();

    let mut init_script = r#"
      function post(event, data) {
        window.ipc.postMessage(
          JSON.stringify({ event, data }));
      }

      let __contextMenuEnabled = <<CTX_MENU_ENABLED>>;
      let __contextMenuKey     = <<CTX_MENU_KEY>>;
      let __keyCodePressing    = new Set();

      document.addEventListener("keydown", e => __keyCodePressing.add   (e.code));
      document.addEventListener("keyup"  , e => __keyCodePressing.delete(e.code));

      document.addEventListener("contextmenu", e => {
        const activated = __contextMenuKey === null || __keyCodePressing.has(__contextMenuKey);
        (!__contextMenuEnabled || !activated) ? e.preventDefault() : __keyCodePressing.clear();
      });
    "#.to_string();

    if let Some(key) = config.context_menu.is_enabled() {
      init_script = init_script.replace("<<CTX_MENU_ENABLED>>", "true");

      if let Some(key) = key {
        init_script = init_script.replace("<<CTX_MENU_KEY>>", &format!("'{:?}'", key));
      } else {
        init_script = init_script.replace("<<CTX_MENU_KEY>>", "null");
      }
    } else {
      init_script = init_script.replace("<<CTX_MENU_ENABLED>>", "false");
      init_script = init_script.replace("<<CTX_MENU_KEY>>", "null");
    }

    wry::WebViewBuilder::new()
      .with_transparent(true)
      .with_devtools(config.devtools.is_enabled())
      // for the initialization script to work,
      // either `with_url` or `with_html` must be called
      .with_html("<html><head></head><body></body></html>")
      .with_initialization_script(&init_script)
      .with_ipc_handler({
        let o_queue = o_queue.clone();
        move |r| { o_queue.lock().push(r.body().clone()); }
      })
      .build(hwnd)
      .map(|webview| WebviewHandle { webview, i_queue, o_queue })
  }

  for (entity, config, window) in &wnd_entities {
    if window.clip_children {
      warn!("Window entity {entity:?} has `clip_children` enabled, \
        which will prevent the webview to be transparent.");
    }

    let Some(window) = winit_window.get_window(entity)
      else { continue; };

    let Ok(handle) = window.window_handle() else {
      error!("Failed to get window handle for window entity {:?}", entity);
      commands.get_entity(entity).map(|mut e| { e.remove::<Webview>(); });
      continue;
    };

    let webview = match create_webview(&handle, config) {
      Ok(webview) => webview,
      Err(err) => {
        error!("Failed to create webview for window entity {:?}: {:?}", entity, err);
        commands.get_entity(entity).map(|mut e| { e.remove::<Webview>(); });
        continue;
      },
    };

    webviews.0.insert(entity, webview);
    info!("Created webview for window entity {entity:?}");
  }
}


pub(crate) fn sys_update_webview(
  mut webviews: NonSendMut<Webviews>,
      entities: Query<(Entity, &Webview), Changed<Webview>>,
) {
  for (entity, webview) in &entities {
    if let Some(handle) = webviews.0.get_mut(&entity) {
      let mut script = String::new();

      if let Some(key) = webview.context_menu.is_enabled() {
        script += "__contextMenuEnabled = true;\n";
        script += &key.map(|k| format!("__contextMenuKey = '{k:?}'"))
          .unwrap_or("__contextMenuKey = null".to_string());
      } else {
        script += "__contextMenuEnabled = false;\n";
      }

      handle.webview.evaluate_script(&script).ok();
    }
  }
}


pub(crate) fn sys_remove_webview(
  mut removeds: RemovedComponents<Webview>,
  mut webviews: NonSendMut<Webviews>,
) {
  for entity in removeds.read() {
    webviews.0.remove(&entity);
    info!("Removed webview from window entity {entity:?}");
  }
}
