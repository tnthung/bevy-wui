use crate::convert::*;
use crate::webview::*;
use bevy::prelude::*;
use bevy::input::mouse::*;
use bevy::input::keyboard::*;
use serde::Deserialize;
use bevy::platform_support::collections::HashSet;
use bevy::input::ButtonState;


pub(crate) fn sys_webview_events(
      webviews: NonSend<Webviews>,
  mut event_ki: EventWriter<KeyboardInput>,
  mut event_mm: EventWriter<MouseMotion>,
  mut event_mb: EventWriter<MouseButtonInput>,
  mut presseds: Local<HashSet<Key>>,
) {
  for (entity, webview) in webviews.0.iter() {
    for event in webview.o_queue.lock().drain(..) {
      let Some((name, data)) = event.split_once('\u{1}')
        else { error!("Invalid event: {event}"); continue; };

      match name {
        "kd" => {
          let Ok(data) = serde_json::from_str::<KeyboardInputPayload>(&data)
            else { error!("Failed to deserialize event data: {data}"); continue; };

          let logical_key = to_key    (&data.key );
          let key_code    = to_keycode(&data.code);

          let repeat = presseds.contains(&logical_key);
          presseds.insert(logical_key.clone());

          event_ki.send(KeyboardInput {
            state: ButtonState::Pressed,
            text: None,
            window: *entity,
            repeat,
            key_code,
            logical_key,
          });
        }

        "ku" => {
          let Ok(data) = serde_json::from_str::<KeyboardInputPayload>(&data)
            else { error!("Failed to deserialize event data: {data}"); continue; };

          let logical_key = to_key    (&data.key );
          let key_code    = to_keycode(&data.code);

          presseds.remove(&logical_key);

          event_ki.send(KeyboardInput {
            state: ButtonState::Released,
            text: None,
            window: *entity,
            repeat: false,
            key_code,
            logical_key,
          });
        }

        "mm" => {
          let Ok(data) = serde_json::from_str::<MouseMotionPayload>(&data)
            else { error!("Failed to deserialize event data: {data}"); continue; };
          event_mm.send(MouseMotion { delta: Vec2::new(data.rel_x, data.rel_y) });
        }

        "md" => {
          let Ok(data) = serde_json::from_str::<MouseButtonPayload>(&data)
            else { error!("Failed to deserialize event data: {data}"); continue; };

          event_mb.send(MouseButtonInput {
            button: to_mouse(data.button),
            state : ButtonState::Pressed,
            window: *entity,
          });
        }

        "mu" => {
          let Ok(data) = serde_json::from_str::<MouseButtonPayload>(&data)
            else { error!("Failed to deserialize event data: {data}"); continue; };

          event_mb.send(MouseButtonInput {
            button: to_mouse(data.button),
            state : ButtonState::Released,
            window: *entity,
          });
        }

        _ => { error!("Unknown event: {name}, {data}"); }
      }
    }
  }
}


#[derive(Debug, Deserialize)]
struct MouseButtonPayload {
  button: u16,
}


#[derive(Debug, Deserialize)]
struct MouseMotionPayload {
  rel_x: f32,
  rel_y: f32,
}


#[derive(Debug, Deserialize)]
struct KeyboardInputPayload {
  key : String,
  code: String,
}
