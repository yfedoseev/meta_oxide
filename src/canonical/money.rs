//! Money + availability primitives shared by every commerce-flavoured
//! canonical type ([`Product`](super::Product), [`Event`](super::Event),
//! [`Recipe`](super::Recipe) etc.).

use serde::{Deserialize, Serialize};

/// A monetary amount with currency code.
///
/// Currency is stored as the raw 3-letter ISO 4217 code as it appeared in the
/// source (`"USD"`, `"EUR"`, …) — `meta_oxide` does not normalise or convert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    /// Numeric amount.
    pub amount: f64,
    /// ISO 4217 currency code as it appeared in the source.
    pub currency: String,
}

impl Money {
    /// Construct a [`Money`] value.
    pub fn new(amount: f64, currency: impl Into<String>) -> Self {
        Self { amount, currency: currency.into() }
    }
}

/// Schema.org product availability.
///
/// Maps `https://schema.org/InStock`, `https://schema.org/OutOfStock`, etc.
/// onto a closed enum, with [`Availability::Other`] catching anything we don't
/// recognise so callers don't lose information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    /// `https://schema.org/InStock`
    InStock,
    /// `https://schema.org/OutOfStock`
    OutOfStock,
    /// `https://schema.org/PreOrder`
    PreOrder,
    /// `https://schema.org/Discontinued`
    Discontinued,
    /// `https://schema.org/LimitedAvailability`
    LimitedAvailability,
    /// `https://schema.org/SoldOut`
    SoldOut,
    /// `https://schema.org/BackOrder`
    BackOrder,
    /// Anything we didn't recognise — payload is the raw string from the source.
    Other(String),
}

impl Availability {
    /// Parse a schema.org availability value (full URL or bare name).
    pub fn parse(raw: &str) -> Self {
        let bare = raw.rsplit('/').next().unwrap_or(raw).trim();
        match bare.to_ascii_lowercase().as_str() {
            "instock" | "in_stock" => Availability::InStock,
            "outofstock" | "out_of_stock" => Availability::OutOfStock,
            "preorder" | "pre_order" => Availability::PreOrder,
            "discontinued" => Availability::Discontinued,
            "limitedavailability" | "limited_availability" => Availability::LimitedAvailability,
            "soldout" | "sold_out" => Availability::SoldOut,
            "backorder" | "back_order" => Availability::BackOrder,
            _ => Availability::Other(raw.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_holds_amount_and_currency() {
        let m = Money::new(29.99, "USD");
        assert_eq!(m.amount, 29.99);
        assert_eq!(m.currency, "USD");
    }

    #[test]
    fn availability_parses_schema_url() {
        assert_eq!(Availability::parse("https://schema.org/InStock"), Availability::InStock);
        assert_eq!(Availability::parse("http://schema.org/OutOfStock"), Availability::OutOfStock);
    }

    #[test]
    fn availability_parses_bare_name_case_insensitive() {
        assert_eq!(Availability::parse("InStock"), Availability::InStock);
        assert_eq!(Availability::parse("instock"), Availability::InStock);
        assert_eq!(Availability::parse("PreOrder"), Availability::PreOrder);
    }

    #[test]
    fn availability_other_preserves_raw() {
        let av = Availability::parse("https://schema.org/MysteryStatus");
        assert_eq!(av, Availability::Other("https://schema.org/MysteryStatus".to_string()));
    }
}
