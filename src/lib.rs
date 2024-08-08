use dioxus::prelude::*;

pub fn main() {
    env_logger::init();

    dioxus::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        // rust logo
        h1 { "EN kajsd" }
        div { "asdasdjlaksdj {count} !!!" }
        div { "Wow hotreload works {count}" }
        div { "asdasdjlaksdj {count} !!!" }
        img { src: "assets/logo.png" }
        button { onclick: move |_| count += 1, "Click me" }
        button { onclick: move |_| count -= 1, "Click me" }
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
    stop_unwind(|| main());
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            com_dioxuslabs,
            WryActivity,
            wry::android_setup, // pass the wry::android_setup function to tao which will invoke when the event loop is created
            _start_app
        );
        wry::android_binding!(com_example, mobiledioxus);
    }

    #[cfg(target_os = "ios")]
    _start_app()
}
