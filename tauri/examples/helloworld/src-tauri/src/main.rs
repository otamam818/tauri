#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

#[derive(tauri::FromTauriContext)]
#[config_path = "examples/helloworld/src-tauri/tauri.conf.json"]
struct Context;

fn main() {
  tauri::AppBuilder::<Context>::new()
    .invoke_handler(|_webview, arg| async move {
      use cmd::Cmd::*;
      match serde_json::from_str(&arg) {
        Err(e) => Err(e.into()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(().into())
        }
      }
    })
    .build()
    .unwrap()
    .run();
}
