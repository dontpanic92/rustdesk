use hbb_common::{config::LocalConfig, log};
use std::ops::Deref;

mod cn;
mod en;
mod fr;
mod it;
mod tw;
mod de;
mod ru;
mod eo;
mod ptbr;

lazy_static::lazy_static! {
    pub static ref AVAILABLE_LANGS: std::collections::HashMap<&'static str, &'static std::collections::HashMap<&'static str, &'static str>> = [
        ("en", en::T.deref()),
        ("fr", fr::T.deref()),
        ("cn", cn::T.deref()),
        ("it", it::T.deref()),
        ("tw", tw::T.deref()),
        ("de", de::T.deref()),
        ("ru", ru::T.deref()),
        ("eo", eo::T.deref()),
        ("ptbr", ptbr::T.deref()),
        ("br", ptbr::T.deref()),
        ("pt", ptbr::T.deref()),
    ].iter().cloned().collect();
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn translate(name: String) -> String {
    let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
    log::trace!("The current locale is {}", locale);
    translate_locale(name, &locale)
}

pub fn translate_locale(name: String, locale: &str) -> String {
    let mut lang = LocalConfig::get_option("lang");
    if lang.is_empty() {
        lang = locale
            .split("-")
            .last()
            .map(|x| x.split("_").last().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();
    }
    let lang = lang.to_lowercase();
    let default_t = &en::T.deref();
    let m = AVAILABLE_LANGS.get(lang.as_str()).or(Some(default_t)).unwrap();
    if let Some(v) = m.get(&name as &str) {
        v.to_string()
    } else {
        if lang != "en" {
            if let Some(v) = en::T.get(&name as &str) {
                return v.to_string();
            }
        }
        name
    }
}
