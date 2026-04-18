//! Site-ownership verification meta tags.
//!
//! Search engines, social platforms, and analytics vendors ask site owners
//! to add `<meta name="…-verification" content="token">` (or equivalent)
//! to prove control of a domain. These tags are trust signals: the presence
//! of a Google verification doesn't mean Google endorses the content, but
//! it does mean somebody with Search Console access owns the page.
//!
//! This extractor only *collects* the tokens — it does not verify them.
//! Verification requires a roundtrip to the issuing vendor's API.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Site verification tokens collected from a page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Verifications {
    /// Google Search Console (`google-site-verification`).
    pub google: Option<String>,
    /// Bing Webmaster Tools (`msvalidate.01`).
    pub bing: Option<String>,
    /// Facebook / Meta domain verification (`facebook-domain-verification`).
    pub facebook: Option<String>,
    /// Yandex Webmaster (`yandex-verification`).
    pub yandex: Option<String>,
    /// Baidu (`baidu-site-verification`).
    pub baidu: Option<String>,
    /// Naver (`naver-site-verification`).
    pub naver: Option<String>,
    /// Pinterest (`p:domain_verify` / `pinterest-site-verification`).
    pub pinterest: Option<String>,
    /// TikTok / ByteDance (`bytedance-verification-code`).
    pub tiktok: Option<String>,
    /// Norton Safe Web (`norton-safeweb-site-verification`).
    pub norton: Option<String>,
    /// Alexa / Amazon (`alexaVerifyID`).
    pub alexa: Option<String>,
    /// Cloudflare (`cloudflare-verify`).
    pub cloudflare: Option<String>,
    /// Shopify store verification (`shopify-digital-wallet`, etc.).
    pub shopify: Option<String>,
    /// Catch-all for verification-ish tags we don't model explicitly.
    /// Keyed by the raw `<meta name>` value.
    pub other: BTreeMap<String, String>,
}

impl Verifications {
    /// True when no verification tokens were found.
    pub fn is_empty(&self) -> bool {
        self.google.is_none()
            && self.bing.is_none()
            && self.facebook.is_none()
            && self.yandex.is_none()
            && self.baidu.is_none()
            && self.naver.is_none()
            && self.pinterest.is_none()
            && self.tiktok.is_none()
            && self.norton.is_none()
            && self.alexa.is_none()
            && self.cloudflare.is_none()
            && self.shopify.is_none()
            && self.other.is_empty()
    }
}

#[cfg(feature = "python")]
impl Verifications {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        macro_rules! opt {
            ($f:ident, $k:literal) => {
                if let Some(v) = &self.$f {
                    dict.set_item($k, v).ok();
                }
            };
        }
        opt!(google, "google");
        opt!(bing, "bing");
        opt!(facebook, "facebook");
        opt!(yandex, "yandex");
        opt!(baidu, "baidu");
        opt!(naver, "naver");
        opt!(pinterest, "pinterest");
        opt!(tiktok, "tiktok");
        opt!(norton, "norton");
        opt!(alexa, "alexa");
        opt!(cloudflare, "cloudflare");
        opt!(shopify, "shopify");
        if !self.other.is_empty() {
            let other = PyDict::new(py);
            for (k, v) in &self.other {
                other.set_item(k, v).ok();
            }
            dict.set_item("other", other).ok();
        }
        dict.unbind()
    }
}
