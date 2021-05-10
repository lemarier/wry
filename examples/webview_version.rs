// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
  use wry::webview::webview_version;

  let version = webview_version()?;
  println!("version {}", version);

  Ok(())
}