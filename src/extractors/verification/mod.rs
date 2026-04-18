//! Site-ownership verification meta-tag extractor.
//!
//! See [`crate::types::verification`] for the output shape.

use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::types::verification::Verifications;
use scraper::Html;

#[cfg(test)]
mod tests;

/// Extract verification tokens from an HTML string.
pub fn extract(html: &str) -> Result<Verifications> {
    extract_from_dom(&html_utils::parse_html(html))
}

/// Extract verification tokens from an already-parsed DOM.
pub fn extract_from_dom(document: &Html) -> Result<Verifications> {
    let mut v = Verifications::default();
    let Ok(selector) = html_utils::create_selector("meta[name][content], meta[property][content]")
    else {
        return Ok(v);
    };

    for element in document.select(&selector) {
        let name = html_utils::get_attr(&element, "name")
            .or_else(|| html_utils::get_attr(&element, "property"));
        let Some(name) = name else { continue };
        let Some(content) = html_utils::get_attr(&element, "content") else { continue };
        let content = content.trim().to_string();
        if content.is_empty() {
            continue;
        }
        let lname = name.to_ascii_lowercase();

        match lname.as_str() {
            "google-site-verification" => v.google = Some(content),
            "msvalidate.01" => v.bing = Some(content),
            "facebook-domain-verification" => v.facebook = Some(content),
            "yandex-verification" => v.yandex = Some(content),
            "baidu-site-verification" => v.baidu = Some(content),
            "naver-site-verification" => v.naver = Some(content),
            "p:domain_verify" | "pinterest-site-verification" => v.pinterest = Some(content),
            "bytedance-verification-code" | "tiktok-developers-site-verification" => {
                v.tiktok = Some(content)
            }
            "norton-safeweb-site-verification" => v.norton = Some(content),
            "alexaverifyid" => v.alexa = Some(content),
            "cloudflare-verify" => v.cloudflare = Some(content),
            "shopify-digital-wallet" | "shopify-checkout-api-token" => v.shopify = Some(content),
            _ if lname.contains("-verification") || lname.contains("-verify") => {
                v.other.insert(lname, content);
            }
            _ => {}
        }
    }

    Ok(v)
}
