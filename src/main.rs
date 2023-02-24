use std::borrow::Cow;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::http::response::Builder;
use wry::http::{header, Response};
use wry::webview::WebViewBuilder;

const CONTENT: &str = include_str!("google.html");
const CONTENT_BYTE: &[u8] = include_bytes!("google.html");
const EMPTY: &[u8] = b"HELLO";

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("demo")
        .with_visible(true)
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        // .with_html(CONTENT)  // error
        .with_url("wry://dev/google") // success
        .unwrap()
        .with_devtools(true)
        .with_custom_protocol("wry".into(), move |request| {
            let uri = request.uri().to_string();
            let url = uri.strip_prefix("wry://dev/").unwrap();


            let (content, resp) = match url {
                "google" => (CONTENT_BYTE, common_resp("text/html")),
                _ => (EMPTY, common_resp("text/plain")),
            };

            resp.body(Cow::from(content)).map_err(Into::into)
        }).build().unwrap();

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    })
}

fn common_resp(content_type: &str) -> Builder {
    Response::builder()
        .header("Origin", "http://localhost/")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::CONTENT_TYPE, content_type)
        .header("Access-Control-Request-Method", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(200)
}
