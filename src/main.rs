use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use mime_guess::from_path;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use pulldown_cmark::{html, Options, Parser as MdParser};
use std::{
    cmp::Ordering,
    fmt::Write as _,
    path::{Path as FsPath, PathBuf},
    sync::Arc,
};
use tokio::fs;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

const URL_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'#')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

#[derive(Clone)]
struct AppState {
    root: Arc<PathBuf>,
    root_real: Arc<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(name = "md_explorer", about = "Markdown File Explorer Web Server")]
struct Args {
    /// Root directory to serve (default: ./content)
    #[arg(short, long, default_value = "content")]
    root: PathBuf,

    /// Address to bind (default: 127.0.0.1:3000)
    // #[arg(short = 'a', long, default_value = "127.0.0.1:3000")]
    #[arg(short = 'a', long, default_value = "0.0.0.0:3000")]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    let args = Args::parse();

    fs::create_dir_all(&args.root).await?;
    let root_real = fs::canonicalize(&args.root).await?;

    info!("Serving root: {}", root_real.display());
    info!("Open: http://{}/", &args.addr);

    let state = AppState {
        root: Arc::new(args.root),
        root_real: Arc::new(root_real),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/*path", get(handle_any))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    // Always render the *canonical* root directory for "/"
    directory_listing_html(&state, (*state.root_real).clone()).await
}

async fn handle_any(State(state): State<AppState>, Path(path): Path<String>) -> Response {
    // Percent-decode the URL path so names with spaces etc. work
    let decoded = percent_decode_str(&path).decode_utf8_lossy().to_string();

    match map_request_path_to_fs(&state, &decoded).await {
        Ok(MappedPath { fs_path, is_dir }) if is_dir => directory_listing_html(&state, fs_path).await,
        Ok(MappedPath { fs_path, .. }) => serve_file_or_markdown(&state, fs_path).await,
        Err(MapError::NotFound) => error_page(StatusCode::NOT_FOUND, "Not Found"),
        Err(MapError::Forbidden) => error_page(StatusCode::FORBIDDEN, "Forbidden"),
        Err(MapError::Io(err)) => {
            error!("IO error: {err:?}");
            error_page(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        }
    }
}

struct MappedPath {
    fs_path: PathBuf,
    is_dir: bool,
}

enum MapError {
    NotFound,
    Forbidden,
    Io(std::io::Error),
}

impl From<std::io::Error> for MapError {
    fn from(e: std::io::Error) -> Self {
        MapError::Io(e)
    }
}

async fn map_request_path_to_fs(state: &AppState, req_path: &str) -> Result<MappedPath, MapError> {
    // Empty path or "." => root
    let clean = req_path.trim_start_matches('/');
    let target = if clean.is_empty() || clean == "." {
        (*state.root_real).clone()
    } else {
        state.root.as_ref().join(clean)
    };

    let md = fs::metadata(&target).await.ok().ok_or(MapError::NotFound)?;

    let real = fs::canonicalize(&target).await.map_err(MapError::Io)?;
    if !is_within(&real, &state.root_real) {
        return Err(MapError::Forbidden);
    }

    Ok(MappedPath {
        is_dir: md.is_dir(),
        fs_path: real,
    })
}

fn is_within(path: &FsPath, root: &FsPath) -> bool {
    path.starts_with(root)
}

async fn directory_listing_html(state: &AppState, fs_dir: PathBuf) -> Response {
    let rel = relative_from_root_display(state, &fs_dir);

    let mut rd = match fs::read_dir(&fs_dir).await {
        Ok(rd) => rd,
        Err(e) => {
            error!("read_dir failed for {}: {e:?}", fs_dir.display());
            return error_page(StatusCode::FORBIDDEN, "Cannot read directory");
        }
    };

    let mut items: Vec<(String, bool)> = Vec::new();
    while let Ok(Some(entry)) = rd.next_entry().await {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        match entry.metadata().await {
            Ok(md) => items.push((name, md.is_dir())),
            Err(_) => continue,
        }
    }

    items.sort_by(|(a, ad), (b, bd)| match (ad, bd) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => a.to_lowercase().cmp(&b.to_lowercase()),
    });

    let mut list_html = String::new();
    if fs_dir != *state.root_real {
        let parent = fs_dir.parent().unwrap_or(&state.root_real);
        let parent_rel = path_rel_url(state, parent);
        let _ = write!(
            &mut list_html,
            r#"<li><a class="up" href="/{parent_rel}">⬆ Up</a></li>"#
        );
    }

    for (name, is_dir) in items {
        let child = fs_dir.join(&name);
        let rel_url = path_rel_url(state, &child);
        let icon = if is_dir { "📁" } else { "📄" };
        let _ = write!(
            &mut list_html,
            r#"<li><a href="/{rel_url}">{icon} {}</a></li>"#,
            html_escape(&name)
        );
    }

    let title = if rel.is_empty() { "/".to_string() } else { format!("/{rel}") };

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Index of {}</title>
<style>
  :root {{ font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Arial; }}
  body {{ margin: 0; background:#0b0d12; color:#e5e7eb; }}
  header {{ padding: 16px 24px; background: #0f1320; border-bottom:1px solid #1f2433; }}
  h1 {{ margin:0; font-size:18px; color:#cbd5e1; }}
  main {{ max-width: 900px; margin: 0 auto; padding: 24px; }}
  ul {{ list-style:none; padding:0; margin:0; }}
  li {{ border:1px solid #1f2433; margin-bottom:8px; border-radius:12px; background:#111726; }}
  a {{ display:block; padding:12px 14px; color:#c7d2fe; text-decoration:none; }}
  a:hover {{ background:#0f1524; }}
  a.up {{ color:#a7f3d0; }}
  .footer {{ opacity:0.6; margin-top:24px; font-size:12px; }}
</style>
</head>
<body>
<header><h1>Index of {}</h1></header>
<main>
  <ul>{}</ul>
  <div class="footer">Markdown files render when clicked. Other files download or display raw.</div>
</main>
</body>
</html>"#,
        html_escape(&title),
        html_escape(&title),
        list_html
    ))
    .into_response()
}

async fn serve_file_or_markdown(_state: &AppState, fs_path: PathBuf) -> Response {
    let ext = fs_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "md" | "markdown" => match fs::read_to_string(&fs_path).await {
            Ok(md) => {
                let html = render_markdown(&md);
                let title = fs_path.file_name().and_then(|s| s.to_str()).unwrap_or("Markdown");
                Html(page_wrap(title, &html, &fs_path.display().to_string())).into_response()
            }
            Err(_) => error_page(StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file"),
        },
        _ => match fs::read(&fs_path).await {
            Ok(bytes) => {
                let mime = from_path(&fs_path).first_or_octet_stream();
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap());
                (headers, bytes).into_response()
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error_page(StatusCode::NOT_FOUND, "Not Found")
            }
            Err(_) => error_page(StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file"),
        },
    }
}

fn page_wrap(title: &str, body_html: &str, subtitle: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>{}</title>
<style>
  :root {{ color-scheme: dark; }}
  body {{ margin: 0; font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Arial; background:#0b0d12; color:#e5e7eb; }}
  header {{ padding: 16px 24px; background: #0f1320; border-bottom:1px solid #1f2433; display:flex; gap:12px; align-items:center; }}
  a.back {{ color:#a7f3d0; text-decoration:none; padding:6px 10px; border:1px solid #1f2433; border-radius:8px; background:#101626; }}
  a.back:hover {{ background:#0f1524; }}
  main {{ max-width: 900px; margin: 0 auto; padding: 24px; }}
  .markdown {{ line-height:1.7; }}
  .markdown h1, .markdown h2, .markdown h3 {{ margin-top: 1.2em; }}
  .markdown code {{ background:#111726; padding:2px 6px; border-radius:6px; }}
  .markdown pre code {{ display:block; padding:14px; overflow:auto; }}
  .markdown blockquote {{ border-left:4px solid #334155; padding-left:12px; color:#cbd5e1; }}
  .markdown a {{ color:#c7d2fe; text-decoration:none; }}
  .markdown a:hover {{ text-decoration:underline; }}
  hr {{ border-color:#1f2433; }}
</style>
</head>
<body>
<header>
  <a class="back" href="javascript:history.back()">⬅ Back</a>
  <div>{}</div>
</header>
<main>
  <article class="markdown">{}</article>
</main>
</body>
</html>"#,
        html_escape(title),
        html_escape(subtitle),
        body_html
    )
}

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = MdParser::new_ext(md, options);

    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

fn relative_from_root_display(state: &AppState, abs: &FsPath) -> String {
    if let Ok(rel) = abs.strip_prefix(&*state.root_real) {
        rel.to_string_lossy().replace('\\', "/")
    } else {
        String::new()
    }
}

fn path_rel_url(state: &AppState, abs: &FsPath) -> String {
    let rel = relative_from_root_display(state, abs);
    rel.split('/')
        .map(|seg| utf8_percent_encode(seg, URL_ENCODE_SET).to_string())
        .collect::<Vec<_>>()
        .join("/")
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn error_page(code: StatusCode, msg: &str) -> Response {
    let html = format!(
        r#"<!DOCTYPE html><meta charset="utf-8">
<title>{} {}</title>
<body style="background:#0b0d12;color:#e5e7eb;font-family:ui-sans-serif,system-ui">
<div style="max-width:700px;margin:5rem auto">
<h1 style="margin-bottom:0.5rem">{}</h1>
<p style="opacity:0.8">The server couldn't complete your request.</p>
<a href="javascript:history.back()" style="color:#a7f3d0">Go back</a>
</div>
</body>"#,
        code.as_u16(),
        msg,
        html_escape(&format!("{} {}", code.as_u16(), msg))
    );
    (code, Html(html)).into_response()
}
