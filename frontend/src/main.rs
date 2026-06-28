// Todo Yew Frontend Entrypoint
//
// This is the main entry point for the Todo web application.
// It initializes and renders the main App component using Yew.
//
// Copyright (c) 2026 Todo Authors. All rights reserved.

mod api;
mod app;
mod components;
mod i18n;
mod storage;
mod theme;
mod types;

fn main() {
    // Initialize logging or other runtime settings if needed in the future
    yew::Renderer::<app::App>::new().render();
}
