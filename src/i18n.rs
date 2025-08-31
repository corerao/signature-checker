use i18n_embed::{
    DefaultLocalizer, LanguageLoader,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use i18n_embed_fl::fl;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct LocalizationsEmbed;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    // 这里直接加载 fallback 语言
    loader
        .load_fallback_language(&LocalizationsEmbed)
        .expect("Error while loading fallback language");
    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!(crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!(crate::i18n::LANGUAGE_LOADER, $message_id, $($args),*)
    }};
}
/// Get the hello world statement in whatever the currently selected
/// localization is.
pub fn hello_world() -> String {
    fl!("check_path")
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> DefaultLocalizer<'static> {
    DefaultLocalizer::new(&*LANGUAGE_LOADER, &LocalizationsEmbed)
}

#[test]
fn h1() {
    println!("{}", hello_world());
}
