use crate::{conf::AppConf, utils};
use log::info;
use std::time::SystemTime;
use tauri::{utils::config::WindowUrl, window::WindowBuilder, Manager, async_runtime};
use uuid::Uuid;

pub fn tray_window(handle: &tauri::AppHandle) {
  let app_conf = AppConf::read();
  let theme = AppConf::theme_mode();
  let app = handle.clone();

  tauri::async_runtime::spawn(async move {
    let link = if app_conf.tray_dashboard {
      "index.html"
    } else {
      &app_conf.tray_origin
    };
    let mut tray_win = WindowBuilder::new(&app, "tray", WindowUrl::App(link.into()))
      .title("Horton")
      .resizable(false)
      .fullscreen(false)
      .inner_size(app_conf.tray_width, app_conf.tray_height)
      .decorations(false)
      .always_on_top(true)
      .theme(Some(theme))
      .initialization_script(&utils::user_script())
      .initialization_script(include_str!("../scripts/core.js"))
      .user_agent(&app_conf.ua_tray);

    if app_conf.tray_origin == "https://www.mopublicnotices.com/(S(wcvlwvy4gyyzekdilmwvclqv))/authenticate.aspx" && !app_conf.tray_dashboard {
      tray_win = tray_win
        .initialization_script(include_str!("../vendors/floating-ui-core.js"))
        .initialization_script(include_str!("../vendors/floating-ui-dom.js"))
        .initialization_script(include_str!("../scripts/cmd.js"))
        .initialization_script(include_str!("../scripts/horton.js"))
        .initialization_script(include_str!("../scripts/popup.core.js"))
        .initialization_script(include_str!("../scripts/auth.missouri-public-notices.js"))
    }

    tray_win.build().unwrap().hide().unwrap();
  });
}

pub fn dalle2_window(
  handle: &tauri::AppHandle,
  query: Option<String>,
  title: Option<String>,
  is_new: Option<bool>,
) {
  info!("dalle2_query: {:?}", query);
  let theme = AppConf::theme_mode();
  let app = handle.clone();

  let query = if query.is_some() {
    format!("window.addEventListener('DOMContentLoaded', function() {{\nwindow.__HORTON_QUERY='{}';\n}})", query.unwrap())
  } else {
    "".to_string()
  };

  let label = if is_new.unwrap_or(true) {
    let timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    format!("dalle2_{}", timestamp)
  } else {
    "dalle2".to_string()
  };

  if app.get_window("dalle2").is_none() {
    tauri::async_runtime::spawn(async move {
      WindowBuilder::new(
        &app,
        label,
        WindowUrl::App("https://labs.openai.com".into()),
      )
      .title(title.unwrap_or_else(|| "DALL·E 2".to_string()))
      .resizable(true)
      .fullscreen(false)
      .inner_size(800.0, 600.0)
      .always_on_top(false)
      .theme(Some(theme))
      .initialization_script(include_str!("../scripts/core.js"))
      .initialization_script(&query)
      .initialization_script(include_str!("../scripts/dalle2.js"))
      .build()
      .unwrap();
    });
  } else {
    let dalle2_win = app.get_window("dalle2").unwrap();
    dalle2_win.show().unwrap();
    dalle2_win.set_focus().unwrap();
  }
}

pub mod cmd {
  use super::*;
  use log::info;
  use tauri::{command, utils::config::WindowUrl, window::WindowBuilder, Manager};

  #[tauri::command]
  pub fn dalle2_search_window(app: tauri::AppHandle, query: String) {
    dalle2_window(
      &app.app_handle(),
      Some(query),
      Some("Horton & DALL·E 2".to_string()),
      None,
    );
  }

  #[tauri::command]
  pub fn control_window(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
      if handle.get_window("main").is_none() {
        WindowBuilder::new(
          &handle,
          "main",
          WindowUrl::App("index.html?type=control".into()),
        )
        .title("Control Center")
        .resizable(true)
        .fullscreen(false)
        .inner_size(1200.0, 700.0)
        .min_inner_size(1000.0, 600.0)
        .build()
        .unwrap();
      } else {
        let main_win = handle.get_window("main").unwrap();
        main_win.show().unwrap();
        main_win.set_focus().unwrap();
      }
    });
  }

  #[command]
  pub fn post_horton_dashboard_notice(
    app: tauri::AppHandle,
    content: String,
    link: String,
    pdf: Option<String>,
  ) {
    info!("post_horton_dashboard_notice: link: {}, content: {}", link, content);
  
    // clean text
    
  }

  #[command]
  pub fn handle_missouri_public_notice(
    app: tauri::AppHandle,
    pdf: Option<String>,
    link: String,
    content: String,
  ) {

    if let Some(pdf) = pdf {
      info!("pdf is Some: {}", pdf);

      pdf_window(app, link.clone(), content, pdf, link)
      // pdf is now unwrapped and contains the String value
      // Your code for handling the pdf case
    } else {
      post_horton_dashboard_notice(app, content, link, pdf)
    }
  }

  #[command]
  pub fn pdf_window(
      app: tauri::AppHandle,
      link: String,
      content: String,
      pdf: String,
      url: String,
  ) {
      let unique_string = Uuid::new_v4().to_string();
  
      let win = app.get_window(&unique_string);
      if win.is_none() {
          async_runtime::spawn(async move {
              WindowBuilder::new(&app, unique_string, WindowUrl::App(pdf.parse().unwrap()))
                  .initialization_script(include_str!("../scripts/missouri-public-notice.notice.pdf.js"))
                  .initialization_script(include_str!("../scripts/core.js"))
                  .initialization_script(&format!("
                      window.NOTICE_LINK = '{}';
                      window.addEventListener('DOMContentLoaded', () => {{
                          const anchors = document.getElementsByTagName('a');
                          for (const anchor of anchors) {{
                              if (anchor.href === window.NOTICE_LINK) {{
                                  window.close();
                                  break;
                              }}
                          }}
                      }});
                  ", url))
                  .inner_size(960.0, 700.0)
                  .resizable(true)
                  .build()
                  .unwrap();
          });
      } else if let Some(v) = win {
          if !v.is_visible().unwrap() {
              v.show().unwrap();
          }
          v.eval("window.location.reload()").unwrap();
          v.set_focus().unwrap();
      }
  }
  

  #[command]
  pub fn notice_window(
    app: tauri::AppHandle,
    title: String,
    url: String,
  ) {
    let unique_string = Uuid::new_v4().to_string();

    info!("notice_window: {} :=> {}", title, url);
    let win = app.get_window(&unique_string);
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        tauri::WindowBuilder::new(&app, unique_string, tauri::WindowUrl::App(url.parse().unwrap()))
          .initialization_script(include_str!("../scripts/missouri-public-notice.notice.js"))
          .initialization_script(include_str!("../scripts/core.js"))
          .title(title)
          .inner_size(960.0, 700.0)
          .resizable(true)
          .build()
          .unwrap();
      });
    } else if let Some(v) = win {
      if !v.is_visible().unwrap() {
        v.show().unwrap();
      }
      v.eval("window.location.reload()").unwrap();
      v.set_focus().unwrap();
    }
  }


  #[command]
  pub fn wa_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    url: String,
    script: Option<String>,
  ) {
    info!("wa_window: {} :=> {}", title, url);
    let win = app.get_window(&label);
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App(url.parse().unwrap()))
          .initialization_script(&script.unwrap_or_default())
          .initialization_script(include_str!("../scripts/core.js"))
          .title(title)
          .inner_size(960.0, 700.0)
          .resizable(true)
          .build()
          .unwrap();
      });
    } else if let Some(v) = win {
      if !v.is_visible().unwrap() {
        v.show().unwrap();
      }
      v.eval("window.location.reload()").unwrap();
      v.set_focus().unwrap();
    }
  }

  #[command]
  pub fn window_reload(app: tauri::AppHandle, label: &str) {
    app
      .app_handle()
      .get_window(label)
      .unwrap()
      .eval("window.location.reload()")
      .unwrap();
  }
}
