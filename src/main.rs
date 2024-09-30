use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dioxus_sdk::i18n::Language;
use dioxus_sdk::i18n::{use_i18, use_init_i18n};
use dioxus_web::Config;
use serde_json::json;
use std::fs;
use std::str::FromStr;
use unic_langid_impl::LanguageIdentifier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prebuild")]
    {
        let language = "en".parse::<LanguageIdentifier>().unwrap();
        let mut vdom = VirtualDom::new_with_props(App, AppProps { language });
        vdom.rebuild_in_place();
        let content = fs::read_to_string("./dist/index.html")?
            .replace("<!-- REPLACE ME -->", &dioxus::ssr::pre_render(&vdom));
        fs::write("./dist/index.html", content)?;
    }
    #[cfg(not(feature = "prebuild"))]
    {
        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        LaunchBuilder::web()
            .with_cfg(Config::new().hydrate(true))
            .launch(|| {
                let language = "sk".parse::<LanguageIdentifier>().unwrap();
                rsx! {
                    App { language }
                }
            });
    }
    Ok(())
}

#[component]
fn App(language: LanguageIdentifier) -> Element {
    use_init_i18n(language.clone(), language, || {
        vec![
            json!({"id": "en", "texts": {"title": "Hello"}}),
            json!({"id": "sk", "texts": {"title": "Ahoj"}}),
        ]
        .into_iter()
        .map(|json| Language::from_str(&json.to_string()).unwrap())
        .collect()
    });
    let mut i18 = use_i18();
    dioxus_logger::tracing::info!("{:?}", i18.translate("title"));
    rsx! {
        h1 {
            {i18.translate("title")}
        }
        button {
            onclick: move |_| i18.set_language(LanguageIdentifier::from_str("en").unwrap()),
            "en"
        }
        button {
            onclick: move |_| i18.set_language(LanguageIdentifier::from_str("sk").unwrap()),
            "sk"
        }
    }
}
