//! Types for JSON-LD / Schema.org structured data (Phase 3)
//!
//! JSON-LD is the fastest-growing format (41% adoption) that enables
//! Google Rich Results, AI/LLM training, and rich metadata extraction.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Helper module for deserializing numeric values that might be strings or numbers
mod string_or_number {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrNumber {
            String(String),
            Number(f64),
        }

        match Option::<StringOrNumber>::deserialize(deserializer)? {
            None => Ok(None),
            Some(StringOrNumber::String(s)) => {
                s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
            }
            Some(StringOrNumber::Number(n)) => Ok(Some(n)),
        }
    }
}

/// Helper module for deserializing integer values that might be strings or numbers
mod string_or_int {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i32),
        }

        match Option::<StringOrInt>::deserialize(deserializer)? {
            None => Ok(None),
            Some(StringOrInt::String(s)) => {
                s.parse::<i32>().map(Some).map_err(serde::de::Error::custom)
            }
            Some(StringOrInt::Int(n)) => Ok(Some(n)),
        }
    }
}

/// A JSON-LD object with Schema.org vocabulary
///
/// JSON-LD objects can be of any Schema.org type (Article, Product, Person, etc.)
/// and may contain nested objects and arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonLdObject {
    /// @context - usually "https://schema.org" or similar
    #[serde(rename = "@context")]
    pub context: Option<Value>,

    /// @type - the Schema.org type (e.g., "Article", "Product", "Person")
    #[serde(rename = "@type")]
    pub type_: Option<Value>, // Can be string or array of strings

    /// @id - unique identifier for this object
    #[serde(rename = "@id")]
    pub id: Option<String>,

    /// @graph - array of JSON-LD objects when multiple are present
    #[serde(rename = "@graph")]
    pub graph: Option<Vec<JsonLdObject>>,

    /// All other properties as a flat map
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Article type (most common JSON-LD type)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Article {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or ImageObject

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Value>, // Can be string, Person, or Organization

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,

    #[serde(rename = "dateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Value>, // Usually Organization

    #[serde(rename = "mainEntityOfPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entity_of_page: Option<String>,

    #[serde(rename = "articleBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_body: Option<String>,

    #[serde(rename = "wordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<u32>,
}

/// Product type (e-commerce)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<Value>, // Can be string, Brand, or Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Value>, // Can be Offer or array of Offers

    #[serde(rename = "aggregateRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_rating: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Value>, // Can be Review or array of Reviews

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>,
}

/// Person type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Person {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "jobTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    #[serde(rename = "worksFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_for: Option<Value>, // Usually Organization
}

/// Organization type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Organization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Value>,

    #[serde(rename = "sameAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_as: Option<Vec<String>>, // Social media URLs

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Review type (Schema.org)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Review {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "reviewBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_body: Option<String>,

    #[serde(rename = "reviewRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_rating: Option<HashMap<String, Value>>,

    #[serde(rename = "itemReviewed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_reviewed: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<HashMap<String, Value>>,

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Event type (Schema.org Event)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>, // ISO 8601 datetime

    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>, // ISO 8601 datetime

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>, // ISO 8601 duration like "PT2H"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>, // Place object or string

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<Value>, // array of Person/Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Value>, // Person or Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Value>, // ticket offers

    #[serde(rename = "eventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>, // EventScheduled, EventCancelled, etc.

    #[serde(rename = "eventAttendanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_attendance_mode: Option<String>, // Online, Offline, Mixed

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "previousStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_start_date: Option<String>, // for rescheduled events
}

/// Recipe type (Schema.org Recipe)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(rename = "recipeIngredient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_ingredient: Option<Vec<String>>,

    #[serde(rename = "recipeInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_instructions: Option<Value>, // Can be string or array of strings

    #[serde(rename = "prepTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prep_time: Option<String>, // ISO 8601 duration (e.g., "PT30M")

    #[serde(rename = "cookTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cook_time: Option<String>, // ISO 8601 duration

    #[serde(rename = "totalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time: Option<String>, // ISO 8601 duration

    #[serde(rename = "recipeYield")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_yield: Option<String>, // e.g., "4 servings"

    #[serde(rename = "recipeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_category: Option<String>, // e.g., "Dessert"

    #[serde(rename = "recipeCuisine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_cuisine: Option<String>, // e.g., "Italian"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutrition: Option<Value>, // NutritionInformation object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Value>, // Can be string or Person object

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,

    #[serde(rename = "aggregateRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_rating: Option<Value>, // AggregateRating object
}

/// BreadcrumbList type (Schema.org BreadcrumbList)
/// Represents hierarchical navigation breadcrumbs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BreadcrumbList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "itemListElement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_list_element: Option<Vec<HashMap<String, Value>>>, // Array of ListItem objects

    #[serde(rename = "numberOfItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<i32>,
}

/// FAQPage type (Schema.org FAQPage)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FAQPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "mainEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entity: Option<Vec<HashMap<String, Value>>>, // Array of Question objects

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<HashMap<String, Value>>, // Can be Person or Organization
}

/// VideoObject type (Schema.org VideoObject)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Value>, // Can be string or array of strings

    #[serde(rename = "uploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>, // ISO 8601 date

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>, // ISO 8601 duration like "PT5M30S"

    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>, // Direct video file URL

    #[serde(rename = "embedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>, // Embeddable player URL

    #[serde(rename = "interactionStatistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_statistic: Option<Value>, // View count object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Value>, // Person or Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Value>, // Usually Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Movie type (Schema.org Movie)
/// Represents a movie, film, or motion picture
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Movie {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<Value>, // Can be Person or array of Persons

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Value>, // Can be Person or array of Persons

    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Value>, // Can be Person or Organization

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Value>, // Screenwriter - Person or array of Persons

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>, // ISO 8601 duration (e.g., "PT2H30M")

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>, // ISO 8601 date

    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>, // ISO 8601 date

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<Value>, // Can be string or array of strings

    #[serde(rename = "contentRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<String>, // e.g., "PG-13", "R", "G"

    #[serde(rename = "aggregateRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_rating: Option<Value>, // AggregateRating object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Value>, // Can be Review or array of Reviews

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Value>, // VideoObject

    #[serde(rename = "countryOfOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_origin: Option<String>, // e.g., "USA", "UK", "FR"

    #[serde(rename = "inLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_language: Option<String>, // e.g., "en", "fr", "es"

    #[serde(rename = "productionCompany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_company: Option<Value>, // Organization

    #[serde(rename = "musicBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_by: Option<Value>, // Person or MusicGroup

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awards: Option<String>, // e.g., "Academy Award for Best Picture"
}

/// ImageObject type (Schema.org ImageObject)
///
/// Represents an image file with metadata. Used for photo galleries, product images,
/// article hero images, and any other image content on the web.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>, // Direct URL to image file

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>, // Image page URL

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>, // Width in pixels

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>, // Height in pixels

    #[serde(rename = "encodingFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>, // MIME type (image/jpeg, image/png, etc.)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Value>, // Thumbnail ImageObject

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Value>, // Creator/photographer (Person or Organization)

    #[serde(rename = "copyrightHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_holder: Option<Value>, // Copyright holder (Person or Organization)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>, // License URL

    #[serde(rename = "uploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>, // ISO 8601 date
}

/// LocalBusiness type (Schema.org LocalBusiness)
///
/// Represents a physical business location (store, restaurant, service provider, etc.)
/// This is one of the most important types for local SEO and Google Business Profile.
/// Supports subtypes like Restaurant, Store, CafeOrCoffeeShop, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBusiness {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<HashMap<String, Value>>, // PostalAddress object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<HashMap<String, Value>>, // GeoCoordinates with latitude/longitude

    #[serde(rename = "openingHoursSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_hours_specification: Option<Vec<HashMap<String, Value>>>, // Hours array

    #[serde(rename = "aggregateRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_rating: Option<HashMap<String, Value>>, // AggregateRating object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Vec<HashMap<String, Value>>>, // Review array

    #[serde(rename = "priceRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_range: Option<String>, // e.g., "$$"

    #[serde(rename = "servesCuisine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serves_cuisine: Option<Vec<String>>, // For restaurants
}

/// HowTo type (Schema.org HowTo)
///
/// Represents step-by-step instructions for accomplishing a task.
/// Commonly used for DIY guides, tutorials, and instructional content.
/// Enables Google to show rich results with steps, tools, and estimated time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HowTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // Title of the how-to

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(rename = "totalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time: Option<String>, // ISO 8601 duration (e.g., "PT2H")

    #[serde(rename = "estimatedCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost: Option<Value>, // MonetaryAmount object or string

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<Value>, // HowToTool array or single object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply: Option<Value>, // HowToSupply array or single object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<Vec<HashMap<String, Value>>>, // Array of HowToStep objects

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Value>, // Person or Organization

    #[serde(rename = "datePublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>, // ISO 8601 date
}

/// WebSite type (Schema.org WebSite)
///
/// Represents a website with metadata like name, description, search capabilities, etc.
/// Important for site-level SEO and enabling Google sitelinks search box.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "potentialAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potential_action: Option<Value>, // SearchAction object for site search

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Value>, // Publisher organization or person

    #[serde(rename = "inLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_language: Option<String>, // Primary language (e.g., "en-US")

    #[serde(rename = "copyrightHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_holder: Option<Value>, // Copyright holder organization or person

    #[serde(rename = "copyrightYear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_year: Option<String>, // Copyright year

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Logo or image (can be string, ImageObject, or array)
}

/// Course type (Schema.org Course)
///
/// Represents an educational course, including online courses, university courses,
/// training programs, etc. Important for educational platforms and online learning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Course {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Value>, // Organization offering the course

    #[serde(rename = "hasCourseInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_course_instance: Option<Vec<HashMap<String, Value>>>, // Course instances

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teaches: Option<Vec<String>>, // What the course teaches

    #[serde(rename = "coursePrerequisites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_prerequisites: Option<Vec<String>>, // Prerequisites

    #[serde(rename = "educationalLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub educational_level: Option<String>, // Beginner, Intermediate, Advanced

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>, // Can be string or array of strings

    #[serde(rename = "aggregateRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_rating: Option<Value>, // AggregateRating object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Value>, // Price/enrollment offers (can be Offer or array)
}

/// JobPosting type (Schema.org JobPosting)
///
/// Represents a job listing for job search engines like Google for Jobs.
/// Essential for recruiting and career pages to appear in job search results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPosting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "hiringOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiring_organization: Option<Value>, // Organization object

    #[serde(rename = "jobLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_location: Option<Value>, // Place object with address

    #[serde(rename = "datePosted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_posted: Option<String>, // ISO 8601 date

    #[serde(rename = "validThrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_through: Option<String>, // Application deadline (ISO 8601)

    #[serde(rename = "employmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<Value>, // FULL_TIME, PART_TIME, CONTRACTOR, etc. (can be string or array)

    #[serde(rename = "baseSalary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_salary: Option<Value>, // MonetaryAmount object

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<String>, // Required qualifications

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<String>, // Job responsibilities

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>, // Application URL

    #[serde(rename = "workHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>, // Working hours
}

/// AggregateRating type (Schema.org)
///
/// Represents the average rating based on multiple ratings or reviews.
/// Used in products, businesses, recipes, and other rated items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AggregateRating {
    #[serde(rename = "ratingValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "string_or_number::deserialize")]
    pub rating_value: Option<f64>,

    #[serde(rename = "bestRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "string_or_number::deserialize")]
    pub best_rating: Option<f64>,

    #[serde(rename = "worstRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "string_or_number::deserialize")]
    pub worst_rating: Option<f64>,

    #[serde(rename = "ratingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "string_or_int::deserialize")]
    pub rating_count: Option<i32>,

    #[serde(rename = "reviewCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "string_or_int::deserialize")]
    pub review_count: Option<i32>,

    #[serde(rename = "itemReviewed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_reviewed: Option<Value>,
}

/// Helper function to convert serde_json::Value to Python objects recursively
#[cfg(feature = "python")]
fn json_value_to_py(py: Python, value: &Value) -> PyObject {
    match value {
        Value::String(s) => s.to_object(py),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_object(py)
            } else if let Some(f) = n.as_f64() {
                f.to_object(py)
            } else {
                n.to_string().to_object(py)
            }
        }
        Value::Bool(b) => b.to_object(py),
        Value::Null => py.None(),
        Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty_bound(py);
            for item in arr {
                py_list.append(json_value_to_py(py, item)).unwrap();
            }
            py_list.to_object(py)
        }
        Value::Object(map) => {
            let py_dict = PyDict::new_bound(py);
            for (key, val) in map {
                py_dict.set_item(key, json_value_to_py(py, val)).unwrap();
            }
            py_dict.to_object(py)
        }
    }
}

// Python conversion for JsonLdObject
#[cfg(feature = "python")]
impl JsonLdObject {
    /// Convert JsonLdObject to Python dictionary
    ///
    /// This method converts the Rust JsonLdObject into a Python dict,
    /// preserving all JSON-LD special properties (@context, @type, @id, @graph)
    /// and all other Schema.org properties.
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(ref context) = self.context {
            dict.set_item("@context", json_value_to_py(py, context)).unwrap();
        }

        if let Some(ref type_) = self.type_ {
            dict.set_item("@type", json_value_to_py(py, type_)).unwrap();
        }

        if let Some(ref id) = self.id {
            dict.set_item("@id", id).unwrap();
        }

        if let Some(ref graph) = self.graph {
            let graph_list: Vec<_> = graph.iter().map(|obj| obj.to_py_dict(py)).collect();
            dict.set_item("@graph", graph_list).unwrap();
        }

        // Convert all other properties using deep conversion
        for (key, value) in &self.properties {
            dict.set_item(key, json_value_to_py(py, value)).unwrap();
        }

        dict.unbind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonld_object_deserialization() {
        let json = r#"{
            "@context": "https://schema.org",
            "@type": "Article",
            "@id": "https://example.com/article",
            "headline": "Test Article",
            "description": "A test article"
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.context.is_some());
        assert!(obj.type_.is_some());
        assert_eq!(obj.id, Some("https://example.com/article".to_string()));
        assert_eq!(obj.properties.get("headline").unwrap().as_str(), Some("Test Article"));
    }

    #[test]
    fn test_jsonld_object_with_graph() {
        let json = r#"{
            "@context": "https://schema.org",
            "@graph": [
                {
                    "@type": "Article",
                    "headline": "Article 1"
                },
                {
                    "@type": "Person",
                    "name": "John Doe"
                }
            ]
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.graph.is_some());
        let graph = obj.graph.unwrap();
        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn test_article_deserialization() {
        let json = r#"{
            "headline": "Test Article",
            "description": "Description",
            "datePublished": "2024-01-15",
            "wordCount": 500
        }"#;

        let article: Article = serde_json::from_str(json).unwrap();
        assert_eq!(article.headline, Some("Test Article".to_string()));
        assert_eq!(article.word_count, Some(500));
    }

    #[test]
    fn test_product_deserialization() {
        let json = r#"{
            "name": "Test Product",
            "description": "Product description",
            "sku": "ABC123"
        }"#;

        let product: Product = serde_json::from_str(json).unwrap();
        assert_eq!(product.name, Some("Test Product".to_string()));
        assert_eq!(product.sku, Some("ABC123".to_string()));
    }

    #[test]
    fn test_person_deserialization() {
        let json = r#"{
            "name": "John Doe",
            "email": "john@example.com",
            "jobTitle": "Developer"
        }"#;

        let person: Person = serde_json::from_str(json).unwrap();
        assert_eq!(person.name, Some("John Doe".to_string()));
        assert_eq!(person.job_title, Some("Developer".to_string()));
    }

    #[test]
    fn test_organization_deserialization() {
        let json = r#"{
            "name": "Acme Corp",
            "url": "https://acme.com",
            "sameAs": [
                "https://twitter.com/acme",
                "https://facebook.com/acme"
            ]
        }"#;

        let org: Organization = serde_json::from_str(json).unwrap();
        assert_eq!(org.name, Some("Acme Corp".to_string()));
        assert_eq!(org.same_as.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_jsonld_object_multiple_types() {
        let json = r#"{
            "@context": "https://schema.org",
            "@type": ["Article", "BlogPosting"],
            "headline": "Test"
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.type_.is_some());
        if let Some(Value::Array(types)) = obj.type_ {
            assert_eq!(types.len(), 2);
        } else {
            panic!("Expected array of types");
        }
    }

    #[test]
    fn test_jsonld_object_nested_properties() {
        let json = r#"{
            "@context": "https://schema.org",
            "@type": "Article",
            "author": {
                "@type": "Person",
                "name": "John Doe"
            }
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.properties.contains_key("author"));
        let author = obj.properties.get("author").unwrap();
        assert!(author.is_object());
    }

    #[test]
    fn test_article_complex_author() {
        let json = r#"{
            "headline": "Test Article",
            "author": {
                "@type": "Person",
                "name": "Jane Smith"
            }
        }"#;

        let article: Article = serde_json::from_str(json).unwrap();
        assert!(article.author.is_some());
    }

    #[test]
    fn test_product_with_offers() {
        let json = r#"{
            "name": "Test Product",
            "offers": {
                "@type": "Offer",
                "price": "29.99",
                "priceCurrency": "USD"
            }
        }"#;

        let product: Product = serde_json::from_str(json).unwrap();
        assert!(product.offers.is_some());
    }

    #[test]
    fn test_jsonld_object_empty() {
        let json = r#"{}"#;
        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.context.is_none());
        assert!(obj.type_.is_none());
        assert!(obj.id.is_none());
        assert!(obj.properties.is_empty());
    }

    #[test]
    fn test_article_empty() {
        let json = r#"{}"#;
        let article: Article = serde_json::from_str(json).unwrap();
        assert!(article.headline.is_none());
        assert!(article.description.is_none());
    }

    #[test]
    fn test_product_empty() {
        let json = r#"{}"#;
        let product: Product = serde_json::from_str(json).unwrap();
        assert!(product.name.is_none());
        assert!(product.sku.is_none());
    }

    #[test]
    fn test_person_empty() {
        let json = r#"{}"#;
        let person: Person = serde_json::from_str(json).unwrap();
        assert!(person.name.is_none());
        assert!(person.email.is_none());
    }

    #[test]
    fn test_organization_empty() {
        let json = r#"{}"#;
        let org: Organization = serde_json::from_str(json).unwrap();
        assert!(org.name.is_none());
        assert!(org.url.is_none());
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_jsonld_object_to_py_dict() {
        Python::with_gil(|py| {
            let json = r#"{
                "@context": "https://schema.org",
                "@type": "Article",
                "@id": "https://example.com/article",
                "headline": "Test Article"
            }"#;

            let obj: JsonLdObject = serde_json::from_str(json).unwrap();
            let py_dict = obj.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("@context").unwrap());
            assert!(dict.contains("@type").unwrap());
            assert!(dict.contains("@id").unwrap());
            assert!(dict.contains("headline").unwrap());
        });
    }

    #[test]
    fn test_jsonld_object_numeric_properties() {
        let json = r#"{
            "@type": "Article",
            "wordCount": 1000,
            "rating": 4.5,
            "views": 12345
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert_eq!(obj.properties.get("wordCount").unwrap().as_i64(), Some(1000));
        assert_eq!(obj.properties.get("rating").unwrap().as_f64(), Some(4.5));
    }

    #[test]
    fn test_jsonld_object_boolean_properties() {
        let json = r#"{
            "@type": "Product",
            "inStock": true,
            "discontinued": false
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert_eq!(obj.properties.get("inStock").unwrap().as_bool(), Some(true));
        assert_eq!(obj.properties.get("discontinued").unwrap().as_bool(), Some(false));
    }

    #[test]
    fn test_jsonld_object_null_property() {
        let json = r#"{
            "@type": "Thing",
            "name": "Test",
            "description": null
        }"#;

        let obj: JsonLdObject = serde_json::from_str(json).unwrap();
        assert!(obj.properties.contains_key("description"));
        assert!(obj.properties.get("description").unwrap().is_null());
    }

    #[test]
    fn test_article_date_formats() {
        let json = r#"{
            "headline": "Test",
            "datePublished": "2024-01-15T10:30:00Z",
            "dateModified": "2024-01-16"
        }"#;

        let article: Article = serde_json::from_str(json).unwrap();
        assert!(article.date_published.is_some());
        assert!(article.date_modified.is_some());
    }

    #[test]
    fn test_organization_multiple_same_as() {
        let json = r#"{
            "name": "Test Org",
            "sameAs": [
                "https://twitter.com/test",
                "https://facebook.com/test",
                "https://linkedin.com/company/test"
            ]
        }"#;

        let org: Organization = serde_json::from_str(json).unwrap();
        assert_eq!(org.same_as.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_person_with_organization() {
        let json = r#"{
            "name": "John Doe",
            "worksFor": {
                "@type": "Organization",
                "name": "Acme Corp"
            }
        }"#;

        let person: Person = serde_json::from_str(json).unwrap();
        assert!(person.works_for.is_some());
    }

    #[test]
    fn test_product_array_of_reviews() {
        let json = r#"{
            "name": "Test Product",
            "review": [
                {
                    "@type": "Review",
                    "reviewRating": {
                        "ratingValue": "5"
                    }
                }
            ]
        }"#;

        let product: Product = serde_json::from_str(json).unwrap();
        assert!(product.review.is_some());
    }

    #[test]
    fn test_review_deserialization() {
        let json = r#"{
            "name": "Great Product Review",
            "reviewBody": "This product is excellent!",
            "reviewRating": {
                "@type": "Rating",
                "ratingValue": "5"
            }
        }"#;

        let review: Review = serde_json::from_str(json).unwrap();
        assert_eq!(review.name, Some("Great Product Review".to_string()));
        assert_eq!(review.review_body, Some("This product is excellent!".to_string()));
        assert!(review.review_rating.is_some());
    }

    #[test]
    fn test_review_with_author() {
        let json = r#"{
            "reviewBody": "Excellent service",
            "author": {
                "@type": "Person",
                "name": "Jane Doe"
            },
            "datePublished": "2024-01-15"
        }"#;

        let review: Review = serde_json::from_str(json).unwrap();
        assert_eq!(review.review_body, Some("Excellent service".to_string()));
        assert!(review.author.is_some());
        assert_eq!(review.date_published, Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_review_with_item_reviewed() {
        let json = r#"{
            "reviewBody": "Amazing headphones!",
            "itemReviewed": {
                "@type": "Product",
                "name": "Wireless Headphones",
                "brand": "Sony"
            },
            "reviewRating": {
                "@type": "Rating",
                "ratingValue": "4.5"
            }
        }"#;

        let review: Review = serde_json::from_str(json).unwrap();
        assert!(review.item_reviewed.is_some());
        assert!(review.review_rating.is_some());
    }

    #[test]
    fn test_review_complete() {
        let json = r#"{
            "name": "Comprehensive Review",
            "reviewBody": "Detailed review text here",
            "reviewRating": {
                "@type": "Rating",
                "ratingValue": "5"
            },
            "itemReviewed": {
                "@type": "Product",
                "name": "Test Product"
            },
            "author": {
                "@type": "Person",
                "name": "John Smith"
            },
            "datePublished": "2024-01-15",
            "publisher": {
                "@type": "Organization",
                "name": "Review Site"
            },
            "url": "https://example.com/review"
        }"#;

        let review: Review = serde_json::from_str(json).unwrap();
        assert_eq!(review.name, Some("Comprehensive Review".to_string()));
        assert!(review.review_body.is_some());
        assert!(review.review_rating.is_some());
        assert!(review.item_reviewed.is_some());
        assert!(review.author.is_some());
        assert!(review.date_published.is_some());
        assert!(review.publisher.is_some());
        assert_eq!(review.url, Some("https://example.com/review".to_string()));
    }

    #[test]
    fn test_review_empty() {
        let json = r#"{}"#;
        let review: Review = serde_json::from_str(json).unwrap();
        assert!(review.name.is_none());
        assert!(review.review_body.is_none());
        assert!(review.review_rating.is_none());
    }

    #[test]
    fn test_recipe_deserialization() {
        let json = r#"{
            "name": "Chocolate Chip Cookies",
            "description": "Classic homemade cookies",
            "prepTime": "PT15M",
            "cookTime": "PT10M",
            "totalTime": "PT25M"
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert_eq!(recipe.name, Some("Chocolate Chip Cookies".to_string()));
        assert_eq!(recipe.description, Some("Classic homemade cookies".to_string()));
        assert_eq!(recipe.prep_time, Some("PT15M".to_string()));
        assert_eq!(recipe.cook_time, Some("PT10M".to_string()));
        assert_eq!(recipe.total_time, Some("PT25M".to_string()));
    }

    #[test]
    fn test_recipe_with_ingredients() {
        let json = r#"{
            "name": "Simple Omelette",
            "recipeIngredient": [
                "3 eggs",
                "2 tablespoons butter",
                "Salt and pepper to taste"
            ]
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert_eq!(recipe.name, Some("Simple Omelette".to_string()));
        assert!(recipe.recipe_ingredient.is_some());
        assert_eq!(recipe.recipe_ingredient.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_recipe_with_instructions_string() {
        let json = r#"{
            "name": "Toast",
            "recipeInstructions": "Place bread in toaster. Toast until golden brown."
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.recipe_instructions.is_some());
        assert!(recipe.recipe_instructions.as_ref().unwrap().is_string());
    }

    #[test]
    fn test_recipe_with_instructions_array() {
        let json = r#"{
            "name": "Pancakes",
            "recipeInstructions": [
                "Mix dry ingredients",
                "Add wet ingredients",
                "Cook on griddle"
            ]
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.recipe_instructions.is_some());
        assert!(recipe.recipe_instructions.as_ref().unwrap().is_array());
    }

    #[test]
    fn test_recipe_with_metadata() {
        let json = r#"{
            "name": "Apple Pie",
            "recipeYield": "8 servings",
            "recipeCategory": "Dessert",
            "recipeCuisine": "American",
            "datePublished": "2024-01-15"
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert_eq!(recipe.recipe_yield, Some("8 servings".to_string()));
        assert_eq!(recipe.recipe_category, Some("Dessert".to_string()));
        assert_eq!(recipe.recipe_cuisine, Some("American".to_string()));
        assert_eq!(recipe.date_published, Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_recipe_with_author() {
        let json = r#"{
            "name": "French Onion Soup",
            "author": {
                "@type": "Person",
                "name": "Julia Child"
            }
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.author.is_some());
    }

    #[test]
    fn test_recipe_with_nutrition() {
        let json = r#"{
            "name": "Grilled Salmon",
            "nutrition": {
                "@type": "NutritionInformation",
                "calories": "320 calories",
                "proteinContent": "35g"
            }
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.nutrition.is_some());
    }

    #[test]
    fn test_recipe_with_rating() {
        let json = r#"{
            "name": "Best Brownies",
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.8",
                "reviewCount": "245"
            }
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.aggregate_rating.is_some());
    }

    #[test]
    fn test_recipe_complete() {
        let json = r#"{
            "name": "Grandma's Apple Pie",
            "description": "A classic homemade apple pie",
            "image": "https://example.com/pie.jpg",
            "recipeIngredient": ["6 cups apples", "3/4 cup sugar"],
            "recipeInstructions": ["Preheat oven", "Mix ingredients", "Bake"],
            "prepTime": "PT30M",
            "cookTime": "PT1H",
            "totalTime": "PT1H30M",
            "recipeYield": "8 servings",
            "recipeCategory": "Dessert",
            "recipeCuisine": "American",
            "nutrition": {
                "@type": "NutritionInformation",
                "calories": "410 calories"
            },
            "author": {
                "@type": "Person",
                "name": "Jane Smith"
            },
            "datePublished": "2024-01-15",
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.9"
            }
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert_eq!(recipe.name, Some("Grandma's Apple Pie".to_string()));
        assert!(recipe.description.is_some());
        assert!(recipe.image.is_some());
        assert!(recipe.recipe_ingredient.is_some());
        assert!(recipe.recipe_instructions.is_some());
        assert!(recipe.prep_time.is_some());
        assert!(recipe.cook_time.is_some());
        assert!(recipe.total_time.is_some());
        assert!(recipe.recipe_yield.is_some());
        assert!(recipe.recipe_category.is_some());
        assert!(recipe.recipe_cuisine.is_some());
        assert!(recipe.nutrition.is_some());
        assert!(recipe.author.is_some());
        assert!(recipe.date_published.is_some());
        assert!(recipe.aggregate_rating.is_some());
    }

    #[test]
    fn test_recipe_empty() {
        let json = r#"{}"#;
        let recipe: Recipe = serde_json::from_str(json).unwrap();
        assert!(recipe.name.is_none());
        assert!(recipe.description.is_none());
        assert!(recipe.recipe_ingredient.is_none());
    }

    #[test]
    fn test_event_deserialization() {
        let json = r#"{
            "name": "Tech Conference 2024",
            "description": "Annual technology conference",
            "startDate": "2024-06-15T09:00:00Z",
            "endDate": "2024-06-17T18:00:00Z"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Tech Conference 2024".to_string()));
        assert_eq!(event.description, Some("Annual technology conference".to_string()));
        assert_eq!(event.start_date, Some("2024-06-15T09:00:00Z".to_string()));
        assert_eq!(event.end_date, Some("2024-06-17T18:00:00Z".to_string()));
    }

    #[test]
    fn test_event_with_location() {
        let json = r#"{
            "name": "Art Exhibition",
            "startDate": "2024-08-01T10:00:00Z",
            "location": {
                "@type": "Place",
                "name": "City Art Gallery",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "123 Main St"
                }
            }
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Art Exhibition".to_string()));
        assert!(event.location.is_some());
    }

    #[test]
    fn test_event_with_performer() {
        let json = r#"{
            "name": "Rock Concert",
            "startDate": "2024-10-15T20:00:00Z",
            "performer": [
                {
                    "@type": "Person",
                    "name": "John Doe"
                },
                {
                    "@type": "MusicGroup",
                    "name": "The Band"
                }
            ]
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Rock Concert".to_string()));
        assert!(event.performer.is_some());
    }

    #[test]
    fn test_event_with_organizer() {
        let json = r#"{
            "name": "Business Conference",
            "startDate": "2024-11-01T09:00:00Z",
            "organizer": {
                "@type": "Organization",
                "name": "Tech Events Inc",
                "url": "https://techevents.com"
            }
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Business Conference".to_string()));
        assert!(event.organizer.is_some());
    }

    #[test]
    fn test_event_with_offers() {
        let json = r#"{
            "name": "Music Festival",
            "startDate": "2024-12-20T12:00:00Z",
            "offers": [
                {
                    "@type": "Offer",
                    "price": "49.99",
                    "priceCurrency": "USD"
                }
            ]
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Music Festival".to_string()));
        assert!(event.offers.is_some());
    }

    #[test]
    fn test_event_virtual() {
        let json = r#"{
            "name": "Online Webinar",
            "startDate": "2024-05-10T14:00:00Z",
            "endDate": "2024-05-10T15:00:00Z",
            "eventAttendanceMode": "https://schema.org/OnlineEventAttendanceMode",
            "location": {
                "@type": "VirtualLocation",
                "url": "https://zoom.us/j/123456789"
            }
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Online Webinar".to_string()));
        assert_eq!(
            event.event_attendance_mode,
            Some("https://schema.org/OnlineEventAttendanceMode".to_string())
        );
        assert!(event.location.is_some());
    }

    #[test]
    fn test_event_with_status() {
        let json = r#"{
            "name": "Cancelled Event",
            "startDate": "2024-06-01T10:00:00Z",
            "eventStatus": "https://schema.org/EventCancelled"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.event_status, Some("https://schema.org/EventCancelled".to_string()));
    }

    #[test]
    fn test_event_with_duration() {
        let json = r#"{
            "name": "Workshop",
            "startDate": "2024-06-15T10:00:00Z",
            "duration": "PT2H"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.duration, Some("PT2H".to_string()));
    }

    #[test]
    fn test_event_rescheduled() {
        let json = r#"{
            "name": "Concert",
            "startDate": "2024-07-20T19:00:00Z",
            "previousStartDate": "2024-06-20T19:00:00Z"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.previous_start_date, Some("2024-06-20T19:00:00Z".to_string()));
        assert_eq!(event.start_date, Some("2024-07-20T19:00:00Z".to_string()));
    }

    #[test]
    fn test_event_with_image_string() {
        let json = r#"{
            "name": "Photo Event",
            "startDate": "2024-08-01T10:00:00Z",
            "image": "https://example.com/event.jpg"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.image.is_some());
        if let Some(serde_json::Value::String(s)) = event.image {
            assert_eq!(s, "https://example.com/event.jpg");
        } else {
            panic!("Expected string image");
        }
    }

    #[test]
    fn test_event_with_image_array() {
        let json = r#"{
            "name": "Photo Event",
            "startDate": "2024-08-01T10:00:00Z",
            "image": [
                "https://example.com/img1.jpg",
                "https://example.com/img2.jpg"
            ]
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.image.is_some());
        if let Some(serde_json::Value::Array(arr)) = event.image {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array of images");
        }
    }

    #[test]
    fn test_event_complete() {
        let json = r#"{
            "name": "Complete Event",
            "description": "A comprehensive event with all fields",
            "image": "https://example.com/event.jpg",
            "startDate": "2024-07-15T18:00:00Z",
            "endDate": "2024-07-15T22:00:00Z",
            "duration": "PT4H",
            "url": "https://example.com/event",
            "location": {
                "@type": "Place",
                "name": "Convention Center"
            },
            "performer": [
                {
                    "@type": "Person",
                    "name": "Performer One"
                }
            ],
            "organizer": {
                "@type": "Organization",
                "name": "Event Organizers Ltd"
            },
            "offers": [
                {
                    "@type": "Offer",
                    "price": "99.99",
                    "priceCurrency": "USD"
                }
            ],
            "eventStatus": "https://schema.org/EventScheduled",
            "eventAttendanceMode": "https://schema.org/OfflineEventAttendanceMode"
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, Some("Complete Event".to_string()));
        assert_eq!(event.description, Some("A comprehensive event with all fields".to_string()));
        assert!(event.image.is_some());
        assert!(event.start_date.is_some());
        assert!(event.end_date.is_some());
        assert_eq!(event.duration, Some("PT4H".to_string()));
        assert_eq!(event.url, Some("https://example.com/event".to_string()));
        assert!(event.location.is_some());
        assert!(event.performer.is_some());
        assert!(event.organizer.is_some());
        assert!(event.offers.is_some());
        assert_eq!(event.event_status, Some("https://schema.org/EventScheduled".to_string()));
        assert_eq!(
            event.event_attendance_mode,
            Some("https://schema.org/OfflineEventAttendanceMode".to_string())
        );
    }

    #[test]
    fn test_event_empty() {
        let json = r#"{}"#;
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.name.is_none());
        assert!(event.description.is_none());
        assert!(event.start_date.is_none());
        assert!(event.end_date.is_none());
        assert!(event.location.is_none());
    }

    #[test]
    fn test_localbusiness_deserialization() {
        let json = r#"{
            "name": "Joe's Coffee Shop",
            "description": "Best coffee in town",
            "telephone": "+1-555-123-4567",
            "email": "info@joescoffee.com",
            "url": "https://joescoffee.com"
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Joe's Coffee Shop".to_string()));
        assert_eq!(business.description, Some("Best coffee in town".to_string()));
        assert_eq!(business.telephone, Some("+1-555-123-4567".to_string()));
        assert_eq!(business.email, Some("info@joescoffee.com".to_string()));
        assert_eq!(business.url, Some("https://joescoffee.com".to_string()));
    }

    #[test]
    fn test_localbusiness_with_address() {
        let json = r#"{
            "name": "Main Street Bakery",
            "address": {
                "@type": "PostalAddress",
                "streetAddress": "123 Main Street",
                "addressLocality": "Springfield",
                "addressRegion": "IL",
                "postalCode": "62701"
            }
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Main Street Bakery".to_string()));
        assert!(business.address.is_some());
    }

    #[test]
    fn test_localbusiness_with_geo() {
        let json = r#"{
            "name": "Mountain View Cafe",
            "geo": {
                "@type": "GeoCoordinates",
                "latitude": "37.3861",
                "longitude": "-122.0839"
            }
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Mountain View Cafe".to_string()));
        assert!(business.geo.is_some());
    }

    #[test]
    fn test_localbusiness_with_hours() {
        let json = r#"{
            "name": "Daily Diner",
            "openingHoursSpecification": [
                {
                    "@type": "OpeningHoursSpecification",
                    "dayOfWeek": "Monday",
                    "opens": "08:00",
                    "closes": "18:00"
                }
            ]
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Daily Diner".to_string()));
        assert!(business.opening_hours_specification.is_some());
    }

    #[test]
    fn test_localbusiness_with_rating() {
        let json = r#"{
            "name": "Top Rated Store",
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.8",
                "reviewCount": "127"
            }
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Top Rated Store".to_string()));
        assert!(business.aggregate_rating.is_some());
    }

    #[test]
    fn test_localbusiness_restaurant() {
        let json = r#"{
            "name": "Italian Bistro",
            "servesCuisine": ["Italian", "Mediterranean"],
            "priceRange": "$$"
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Italian Bistro".to_string()));
        assert_eq!(business.price_range, Some("$$".to_string()));
        assert!(business.serves_cuisine.is_some());
        assert_eq!(business.serves_cuisine.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_localbusiness_with_reviews() {
        let json = r#"{
            "name": "Reviewed Business",
            "review": [
                {
                    "@type": "Review",
                    "author": {
                        "@type": "Person",
                        "name": "John Doe"
                    },
                    "reviewRating": {
                        "@type": "Rating",
                        "ratingValue": "5"
                    }
                }
            ]
        }"#;

        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert_eq!(business.name, Some("Reviewed Business".to_string()));
        assert!(business.review.is_some());
    }

    #[test]
    fn test_localbusiness_empty() {
        let json = r#"{}"#;
        let business: LocalBusiness = serde_json::from_str(json).unwrap();
        assert!(business.name.is_none());
        assert!(business.description.is_none());
        assert!(business.address.is_none());
        assert!(business.telephone.is_none());
    }

    #[test]
    fn test_faqpage_deserialization() {
        let json = r#"{
            "name": "Common Questions",
            "description": "Frequently asked questions",
            "mainEntity": [
                {
                    "@type": "Question",
                    "name": "What is JSON-LD?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "JSON-LD is a lightweight Linked Data format."
                    }
                }
            ]
        }"#;

        let faqpage: FAQPage = serde_json::from_str(json).unwrap();
        assert_eq!(faqpage.name, Some("Common Questions".to_string()));
        assert_eq!(faqpage.description, Some("Frequently asked questions".to_string()));
        assert!(faqpage.main_entity.is_some());
        assert_eq!(faqpage.main_entity.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_faqpage_empty() {
        let json = r#"{}"#;
        let faqpage: FAQPage = serde_json::from_str(json).unwrap();
        assert!(faqpage.name.is_none());
        assert!(faqpage.description.is_none());
        assert!(faqpage.main_entity.is_none());
    }

    #[test]
    fn test_breadcrumblist_deserialization() {
        let json = r#"{
            "name": "Product Navigation",
            "numberOfItems": 3,
            "itemListElement": [
                {
                    "@type": "ListItem",
                    "position": 1,
                    "name": "Home",
                    "item": "https://example.com"
                },
                {
                    "@type": "ListItem",
                    "position": 2,
                    "name": "Products",
                    "item": "https://example.com/products"
                },
                {
                    "@type": "ListItem",
                    "position": 3,
                    "name": "Laptops",
                    "item": "https://example.com/products/laptops"
                }
            ]
        }"#;

        let breadcrumb: BreadcrumbList = serde_json::from_str(json).unwrap();
        assert_eq!(breadcrumb.name, Some("Product Navigation".to_string()));
        assert_eq!(breadcrumb.number_of_items, Some(3));
        assert!(breadcrumb.item_list_element.is_some());
        assert_eq!(breadcrumb.item_list_element.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_breadcrumblist_minimal() {
        let json = r#"{
            "itemListElement": [
                {
                    "@type": "ListItem",
                    "position": 1,
                    "name": "Home"
                }
            ]
        }"#;

        let breadcrumb: BreadcrumbList = serde_json::from_str(json).unwrap();
        assert!(breadcrumb.name.is_none());
        assert!(breadcrumb.number_of_items.is_none());
        assert!(breadcrumb.item_list_element.is_some());
        assert_eq!(breadcrumb.item_list_element.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_breadcrumblist_empty() {
        let json = r#"{}"#;
        let breadcrumb: BreadcrumbList = serde_json::from_str(json).unwrap();
        assert!(breadcrumb.name.is_none());
        assert!(breadcrumb.number_of_items.is_none());
        assert!(breadcrumb.item_list_element.is_none());
    }

    #[test]
    fn test_breadcrumblist_with_thing_item() {
        let json = r#"{
            "itemListElement": [
                {
                    "@type": "ListItem",
                    "position": 1,
                    "name": "Home",
                    "item": {
                        "@type": "Thing",
                        "@id": "https://example.com",
                        "name": "Homepage"
                    }
                }
            ]
        }"#;

        let breadcrumb: BreadcrumbList = serde_json::from_str(json).unwrap();
        assert!(breadcrumb.item_list_element.is_some());
        let items = breadcrumb.item_list_element.as_ref().unwrap();
        assert_eq!(items.len(), 1);
        // Verify the item field exists and is an object
        assert!(items[0].contains_key("item"));
    }

    #[test]
    fn test_video_object_deserialization() {
        let json = r#"{
            "name": "Python Tutorial",
            "description": "Learn Python from scratch",
            "uploadDate": "2024-01-15T10:00:00Z",
            "duration": "PT5M30S"
        }"#;

        let video: VideoObject = serde_json::from_str(json).unwrap();
        assert_eq!(video.name, Some("Python Tutorial".to_string()));
        assert_eq!(video.description, Some("Learn Python from scratch".to_string()));
        assert_eq!(video.upload_date, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(video.duration, Some("PT5M30S".to_string()));
    }

    #[test]
    fn test_video_object_with_urls() {
        let json = r#"{
            "name": "Sample Video",
            "contentUrl": "https://example.com/video.mp4",
            "embedUrl": "https://example.com/embed/video123",
            "url": "https://example.com/watch?v=123"
        }"#;

        let video: VideoObject = serde_json::from_str(json).unwrap();
        assert_eq!(video.name, Some("Sample Video".to_string()));
        assert_eq!(video.content_url, Some("https://example.com/video.mp4".to_string()));
        assert_eq!(video.embed_url, Some("https://example.com/embed/video123".to_string()));
        assert_eq!(video.url, Some("https://example.com/watch?v=123".to_string()));
    }

    #[test]
    fn test_video_object_with_dimensions() {
        let json = r#"{
            "name": "HD Video",
            "width": 1920,
            "height": 1080
        }"#;

        let video: VideoObject = serde_json::from_str(json).unwrap();
        assert_eq!(video.width, Some(1920));
        assert_eq!(video.height, Some(1080));
    }

    #[test]
    fn test_video_object_complete() {
        let json = r#"{
            "name": "Complete Video Example",
            "description": "A comprehensive video with all metadata",
            "thumbnailUrl": "https://example.com/thumbnail.jpg",
            "uploadDate": "2024-01-15T10:00:00Z",
            "duration": "PT10M30S",
            "contentUrl": "https://example.com/video.mp4",
            "embedUrl": "https://example.com/embed/video",
            "url": "https://example.com/watch?v=abc123",
            "width": 1920,
            "height": 1080
        }"#;

        let video: VideoObject = serde_json::from_str(json).unwrap();
        assert_eq!(video.name, Some("Complete Video Example".to_string()));
        assert_eq!(video.description, Some("A comprehensive video with all metadata".to_string()));
        assert_eq!(video.upload_date, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(video.duration, Some("PT10M30S".to_string()));
        assert_eq!(video.content_url, Some("https://example.com/video.mp4".to_string()));
        assert_eq!(video.embed_url, Some("https://example.com/embed/video".to_string()));
        assert_eq!(video.url, Some("https://example.com/watch?v=abc123".to_string()));
        assert_eq!(video.width, Some(1920));
        assert_eq!(video.height, Some(1080));
    }

    #[test]
    fn test_video_object_empty() {
        let json = r#"{}"#;
        let video: VideoObject = serde_json::from_str(json).unwrap();
        assert!(video.name.is_none());
        assert!(video.description.is_none());
        assert!(video.duration.is_none());
        assert!(video.upload_date.is_none());
        assert!(video.content_url.is_none());
        assert!(video.embed_url.is_none());
        assert!(video.url.is_none());
        assert!(video.width.is_none());
        assert!(video.height.is_none());
    }

    #[test]
    fn test_imageobject_deserialization() {
        let json = r#"{
            "name": "Beautiful Sunset",
            "description": "A stunning sunset over the ocean",
            "contentUrl": "https://example.com/sunset.jpg",
            "width": 1920,
            "height": 1080
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.name, Some("Beautiful Sunset".to_string()));
        assert_eq!(image.description, Some("A stunning sunset over the ocean".to_string()));
        assert_eq!(image.content_url, Some("https://example.com/sunset.jpg".to_string()));
        assert_eq!(image.width, Some(1920));
        assert_eq!(image.height, Some(1080));
    }

    #[test]
    fn test_imageobject_with_dimensions() {
        let json = r#"{
            "contentUrl": "https://example.com/photo.jpg",
            "width": 3840,
            "height": 2160
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.width, Some(3840));
        assert_eq!(image.height, Some(2160));
    }

    #[test]
    fn test_imageobject_with_encoding_format() {
        let json = r#"{
            "contentUrl": "https://example.com/photo.jpg",
            "encodingFormat": "image/jpeg"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.encoding_format, Some("image/jpeg".to_string()));
    }

    #[test]
    fn test_imageobject_with_thumbnail() {
        let json = r#"{
            "name": "High Resolution Photo",
            "contentUrl": "https://example.com/photo-4k.jpg",
            "thumbnail": {
                "@type": "ImageObject",
                "contentUrl": "https://example.com/photo-thumb.jpg",
                "width": 150,
                "height": 150
            }
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.name, Some("High Resolution Photo".to_string()));
        assert!(image.thumbnail.is_some());
    }

    #[test]
    fn test_imageobject_with_creator() {
        let json = r#"{
            "name": "Wildlife Photo",
            "contentUrl": "https://example.com/wildlife.jpg",
            "creator": {
                "@type": "Person",
                "name": "Jane Photographer"
            }
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.name, Some("Wildlife Photo".to_string()));
        assert!(image.creator.is_some());
    }

    #[test]
    fn test_imageobject_with_copyright() {
        let json = r#"{
            "contentUrl": "https://example.com/photo.jpg",
            "copyrightHolder": {
                "@type": "Person",
                "name": "Copyright Owner"
            },
            "license": "https://creativecommons.org/licenses/by/4.0/"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.license, Some("https://creativecommons.org/licenses/by/4.0/".to_string()));
        assert!(image.copyright_holder.is_some());
    }

    #[test]
    fn test_imageobject_with_caption() {
        let json = r#"{
            "contentUrl": "https://example.com/historic.jpg",
            "caption": "Historic moment captured on film"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.caption, Some("Historic moment captured on film".to_string()));
    }

    #[test]
    fn test_imageobject_with_upload_date() {
        let json = r#"{
            "contentUrl": "https://example.com/recent.jpg",
            "uploadDate": "2024-01-15T10:30:00Z"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.upload_date, Some("2024-01-15T10:30:00Z".to_string()));
    }

    #[test]
    fn test_imageobject_with_urls() {
        let json = r#"{
            "contentUrl": "https://example.com/images/photo.jpg",
            "url": "https://example.com/gallery/photo123"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.content_url, Some("https://example.com/images/photo.jpg".to_string()));
        assert_eq!(image.url, Some("https://example.com/gallery/photo123".to_string()));
    }

    #[test]
    fn test_imageobject_complete() {
        let json = r#"{
            "name": "Professional Landscape",
            "description": "Award-winning landscape photograph",
            "contentUrl": "https://example.com/landscape-4k.jpg",
            "url": "https://example.com/gallery/landscape",
            "width": 3840,
            "height": 2160,
            "encodingFormat": "image/jpeg",
            "thumbnail": {
                "@type": "ImageObject",
                "contentUrl": "https://example.com/landscape-thumb.jpg"
            },
            "caption": "Mountain vista at sunset",
            "creator": {
                "@type": "Person",
                "name": "Photographer Name"
            },
            "copyrightHolder": {
                "@type": "Person",
                "name": "Copyright Owner"
            },
            "license": "https://creativecommons.org/licenses/by-nc-nd/4.0/",
            "uploadDate": "2024-02-15T14:22:30Z"
        }"#;

        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert_eq!(image.name, Some("Professional Landscape".to_string()));
        assert_eq!(image.description, Some("Award-winning landscape photograph".to_string()));
        assert_eq!(image.content_url, Some("https://example.com/landscape-4k.jpg".to_string()));
        assert_eq!(image.url, Some("https://example.com/gallery/landscape".to_string()));
        assert_eq!(image.width, Some(3840));
        assert_eq!(image.height, Some(2160));
        assert_eq!(image.encoding_format, Some("image/jpeg".to_string()));
        assert_eq!(image.caption, Some("Mountain vista at sunset".to_string()));
        assert_eq!(
            image.license,
            Some("https://creativecommons.org/licenses/by-nc-nd/4.0/".to_string())
        );
        assert_eq!(image.upload_date, Some("2024-02-15T14:22:30Z".to_string()));
        assert!(image.thumbnail.is_some());
        assert!(image.creator.is_some());
        assert!(image.copyright_holder.is_some());
    }

    #[test]
    fn test_imageobject_empty() {
        let json = r#"{}"#;
        let image: ImageObject = serde_json::from_str(json).unwrap();
        assert!(image.name.is_none());
        assert!(image.description.is_none());
        assert!(image.content_url.is_none());
        assert!(image.url.is_none());
        assert!(image.width.is_none());
        assert!(image.height.is_none());
        assert!(image.encoding_format.is_none());
        assert!(image.thumbnail.is_none());
        assert!(image.caption.is_none());
        assert!(image.creator.is_none());
        assert!(image.copyright_holder.is_none());
        assert!(image.license.is_none());
        assert!(image.upload_date.is_none());
    }

    #[test]
    fn test_website_deserialization() {
        let json = r#"{
            "name": "Example Tech Blog",
            "description": "A tech blog about software",
            "url": "https://example.com"
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Example Tech Blog".to_string()));
        assert_eq!(website.description, Some("A tech blog about software".to_string()));
        assert_eq!(website.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_website_with_search_action() {
        let json = r#"{
            "name": "Example Site",
            "url": "https://example.com",
            "potentialAction": {
                "@type": "SearchAction",
                "target": "https://example.com/search?q={search_term_string}",
                "query-input": "required name=search_term_string"
            }
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Example Site".to_string()));
        assert!(website.potential_action.is_some());
    }

    #[test]
    fn test_website_with_publisher() {
        let json = r#"{
            "name": "Tech News",
            "url": "https://technews.example.com",
            "publisher": {
                "@type": "Organization",
                "name": "Tech Media Corp",
                "logo": {
                    "@type": "ImageObject",
                    "url": "https://example.com/logo.png"
                }
            }
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Tech News".to_string()));
        assert!(website.publisher.is_some());
    }

    #[test]
    fn test_website_with_language() {
        let json = r#"{
            "name": "French Blog",
            "url": "https://fr.example.com",
            "inLanguage": "fr-FR"
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("French Blog".to_string()));
        assert_eq!(website.in_language, Some("fr-FR".to_string()));
    }

    #[test]
    fn test_website_with_copyright() {
        let json = r#"{
            "name": "Corporate Site",
            "url": "https://corporate.example.com",
            "copyrightHolder": {
                "@type": "Organization",
                "name": "Example Corp"
            },
            "copyrightYear": "2024"
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Corporate Site".to_string()));
        assert!(website.copyright_holder.is_some());
        assert_eq!(website.copyright_year, Some("2024".to_string()));
    }

    #[test]
    fn test_website_with_image_string() {
        let json = r#"{
            "name": "Site with Logo",
            "image": "https://example.com/logo.png"
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert!(website.image.is_some());
        if let Some(Value::String(s)) = website.image {
            assert_eq!(s, "https://example.com/logo.png");
        } else {
            panic!("Expected string image");
        }
    }

    #[test]
    fn test_website_with_image_object() {
        let json = r#"{
            "name": "Site with Image Object",
            "image": {
                "@type": "ImageObject",
                "url": "https://example.com/logo.png",
                "width": 600,
                "height": 60
            }
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert!(website.image.is_some());
        if let Some(Value::Object(_)) = website.image {
            // Successfully parsed as object
        } else {
            panic!("Expected object image");
        }
    }

    #[test]
    fn test_website_complete() {
        let json = r#"{
            "name": "Complete Example Site",
            "description": "A comprehensive website with all metadata",
            "url": "https://complete.example.com",
            "image": "https://example.com/logo.png",
            "inLanguage": "en-US",
            "copyrightYear": "2024",
            "copyrightHolder": {
                "@type": "Organization",
                "name": "Complete Inc"
            },
            "publisher": {
                "@type": "Organization",
                "name": "Complete Publishing"
            },
            "potentialAction": {
                "@type": "SearchAction",
                "target": "https://complete.example.com/search?q={search_term_string}",
                "query-input": "required name=search_term_string"
            }
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Complete Example Site".to_string()));
        assert_eq!(
            website.description,
            Some("A comprehensive website with all metadata".to_string())
        );
        assert_eq!(website.url, Some("https://complete.example.com".to_string()));
        assert!(website.image.is_some());
        assert_eq!(website.in_language, Some("en-US".to_string()));
        assert_eq!(website.copyright_year, Some("2024".to_string()));
        assert!(website.copyright_holder.is_some());
        assert!(website.publisher.is_some());
        assert!(website.potential_action.is_some());
    }

    #[test]
    fn test_website_empty() {
        let json = r#"{}"#;
        let website: WebSite = serde_json::from_str(json).unwrap();
        assert!(website.name.is_none());
        assert!(website.description.is_none());
        assert!(website.url.is_none());
        assert!(website.potential_action.is_none());
        assert!(website.publisher.is_none());
        assert!(website.in_language.is_none());
        assert!(website.copyright_holder.is_none());
        assert!(website.copyright_year.is_none());
        assert!(website.image.is_none());
    }

    #[test]
    fn test_website_minimal() {
        let json = r#"{
            "name": "Minimal Site"
        }"#;

        let website: WebSite = serde_json::from_str(json).unwrap();
        assert_eq!(website.name, Some("Minimal Site".to_string()));
        assert!(website.description.is_none());
        assert!(website.url.is_none());
    }

    #[test]
    fn test_jobposting_basic() {
        let json = r#"{
            "title": "Senior Software Engineer"
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Senior Software Engineer".to_string()));
        assert!(job.description.is_none());
    }

    #[test]
    fn test_jobposting_with_organization() {
        let json = r#"{
            "title": "Data Scientist",
            "hiringOrganization": {
                "@type": "Organization",
                "name": "Tech Corp",
                "sameAs": "https://www.techcorp.com"
            }
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Data Scientist".to_string()));
        assert!(job.hiring_organization.is_some());
    }

    #[test]
    fn test_jobposting_with_location() {
        let json = r#"{
            "title": "Product Manager",
            "jobLocation": {
                "@type": "Place",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "555 Clancy St",
                    "addressLocality": "Detroit",
                    "addressRegion": "MI",
                    "postalCode": "48201",
                    "addressCountry": "US"
                }
            }
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Product Manager".to_string()));
        assert!(job.job_location.is_some());
    }

    #[test]
    fn test_jobposting_with_salary() {
        let json = r#"{
            "title": "Frontend Developer",
            "baseSalary": {
                "@type": "MonetaryAmount",
                "currency": "USD",
                "value": {
                    "@type": "QuantitativeValue",
                    "value": 120000,
                    "unitText": "YEAR"
                }
            }
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Frontend Developer".to_string()));
        assert!(job.base_salary.is_some());
    }

    #[test]
    fn test_jobposting_with_employment_type_array() {
        let json = r#"{
            "title": "Full Stack Developer",
            "employmentType": ["FULL_TIME", "CONTRACTOR"]
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Full Stack Developer".to_string()));
        assert!(job.employment_type.is_some());
    }

    #[test]
    fn test_jobposting_with_employment_type_string() {
        let json = r#"{
            "title": "Intern",
            "employmentType": "INTERN"
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Intern".to_string()));
        assert!(job.employment_type.is_some());
    }

    #[test]
    fn test_jobposting_with_dates() {
        let json = r#"{
            "title": "DevOps Engineer",
            "datePosted": "2024-01-15",
            "validThrough": "2024-03-15T23:59:59Z"
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("DevOps Engineer".to_string()));
        assert_eq!(job.date_posted, Some("2024-01-15".to_string()));
        assert_eq!(job.valid_through, Some("2024-03-15T23:59:59Z".to_string()));
    }

    #[test]
    fn test_jobposting_complete() {
        let json = r#"{
            "title": "Senior Python Developer",
            "description": "We are seeking an experienced Python developer to join our team.",
            "datePosted": "2024-01-10",
            "validThrough": "2024-04-10T23:59:59Z",
            "employmentType": ["FULL_TIME", "PART_TIME"],
            "hiringOrganization": {
                "@type": "Organization",
                "name": "Example Company"
            },
            "jobLocation": {
                "@type": "Place",
                "address": {
                    "@type": "PostalAddress",
                    "streetAddress": "123 Tech Street",
                    "addressLocality": "San Francisco",
                    "addressRegion": "CA",
                    "postalCode": "94105"
                }
            },
            "baseSalary": {
                "@type": "MonetaryAmount",
                "currency": "USD",
                "value": {
                    "@type": "QuantitativeValue",
                    "minValue": 120000,
                    "maxValue": 180000,
                    "unitText": "YEAR"
                }
            },
            "qualifications": "Bachelor's degree in Computer Science or equivalent experience.",
            "responsibilities": "Design and develop scalable Python applications.",
            "url": "https://example.com/jobs/senior-python-dev",
            "workHours": "40 hours per week"
        }"#;

        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert_eq!(job.title, Some("Senior Python Developer".to_string()));
        assert_eq!(
            job.description,
            Some("We are seeking an experienced Python developer to join our team.".to_string())
        );
        assert_eq!(job.date_posted, Some("2024-01-10".to_string()));
        assert_eq!(job.valid_through, Some("2024-04-10T23:59:59Z".to_string()));
        assert!(job.employment_type.is_some());
        assert!(job.hiring_organization.is_some());
        assert!(job.job_location.is_some());
        assert!(job.base_salary.is_some());
        assert_eq!(
            job.qualifications,
            Some("Bachelor's degree in Computer Science or equivalent experience.".to_string())
        );
        assert_eq!(
            job.responsibilities,
            Some("Design and develop scalable Python applications.".to_string())
        );
        assert_eq!(job.url, Some("https://example.com/jobs/senior-python-dev".to_string()));
        assert_eq!(job.work_hours, Some("40 hours per week".to_string()));
    }

    #[test]
    fn test_jobposting_empty() {
        let json = r#"{}"#;
        let job: JobPosting = serde_json::from_str(json).unwrap();
        assert!(job.title.is_none());
        assert!(job.description.is_none());
        assert!(job.hiring_organization.is_none());
        assert!(job.job_location.is_none());
        assert!(job.date_posted.is_none());
        assert!(job.valid_through.is_none());
        assert!(job.employment_type.is_none());
        assert!(job.base_salary.is_none());
        assert!(job.qualifications.is_none());
        assert!(job.responsibilities.is_none());
        assert!(job.url.is_none());
        assert!(job.work_hours.is_none());
    }

    #[test]
    fn test_course_deserialization() {
        let json = r#"{
            "name": "Introduction to Python Programming",
            "description": "Learn Python from scratch",
            "educationalLevel": "Beginner"
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Introduction to Python Programming".to_string()));
        assert_eq!(course.description, Some("Learn Python from scratch".to_string()));
        assert_eq!(course.educational_level, Some("Beginner".to_string()));
    }

    #[test]
    fn test_course_with_provider() {
        let json = r#"{
            "name": "Data Science Fundamentals",
            "provider": {
                "@type": "Organization",
                "name": "Tech University",
                "url": "https://techuniversity.edu"
            }
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Data Science Fundamentals".to_string()));
        assert!(course.provider.is_some());
    }

    #[test]
    fn test_course_with_prerequisites() {
        let json = r#"{
            "name": "Advanced Machine Learning",
            "coursePrerequisites": [
                "Introduction to Python",
                "Statistics 101"
            ]
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Advanced Machine Learning".to_string()));
        assert!(course.course_prerequisites.is_some());
        assert_eq!(course.course_prerequisites.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_course_with_teaches() {
        let json = r#"{
            "name": "Web Development Bootcamp",
            "teaches": [
                "HTML",
                "CSS",
                "JavaScript"
            ]
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Web Development Bootcamp".to_string()));
        assert!(course.teaches.is_some());
        assert_eq!(course.teaches.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_course_with_instances() {
        let json = r#"{
            "name": "Introduction to AI",
            "hasCourseInstance": [
                {
                    "@type": "CourseInstance",
                    "courseMode": "online",
                    "startDate": "2024-09-01"
                }
            ]
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Introduction to AI".to_string()));
        assert!(course.has_course_instance.is_some());
        assert_eq!(course.has_course_instance.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_course_with_rating() {
        let json = r#"{
            "name": "Full Stack Development",
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.8",
                "reviewCount": "325"
            }
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Full Stack Development".to_string()));
        assert!(course.aggregate_rating.is_some());
    }

    #[test]
    fn test_course_with_offers() {
        let json = r#"{
            "name": "Cloud Computing Essentials",
            "offers": {
                "@type": "Offer",
                "price": "299.99",
                "priceCurrency": "USD"
            }
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Cloud Computing Essentials".to_string()));
        assert!(course.offers.is_some());
    }

    #[test]
    fn test_course_complete() {
        let json = r#"{
            "name": "Complete Web Development Bootcamp",
            "description": "From zero to full-stack developer in 12 weeks",
            "url": "https://example.com/courses/web-dev",
            "image": "https://example.com/images/course.jpg",
            "provider": {
                "@type": "Organization",
                "name": "CodeAcademy Pro"
            },
            "hasCourseInstance": [
                {
                    "@type": "CourseInstance",
                    "courseMode": "online",
                    "startDate": "2024-09-15"
                }
            ],
            "teaches": ["HTML", "CSS", "JavaScript"],
            "coursePrerequisites": ["Basic computer skills"],
            "educationalLevel": "Beginner",
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.9"
            },
            "offers": {
                "@type": "Offer",
                "price": "499.99",
                "priceCurrency": "USD"
            }
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert_eq!(course.name, Some("Complete Web Development Bootcamp".to_string()));
        assert_eq!(
            course.description,
            Some("From zero to full-stack developer in 12 weeks".to_string())
        );
        assert_eq!(course.url, Some("https://example.com/courses/web-dev".to_string()));
        assert!(course.image.is_some());
        assert!(course.provider.is_some());
        assert!(course.has_course_instance.is_some());
        assert!(course.teaches.is_some());
        assert!(course.course_prerequisites.is_some());
        assert_eq!(course.educational_level, Some("Beginner".to_string()));
        assert!(course.aggregate_rating.is_some());
        assert!(course.offers.is_some());
    }

    #[test]
    fn test_course_with_image_string() {
        let json = r#"{
            "name": "Photography Course",
            "image": "https://example.com/photo-course.jpg"
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert!(course.image.is_some());
        if let Some(Value::String(s)) = course.image {
            assert_eq!(s, "https://example.com/photo-course.jpg");
        } else {
            panic!("Expected string image");
        }
    }

    #[test]
    fn test_course_with_image_array() {
        let json = r#"{
            "name": "Design Course",
            "image": [
                "https://example.com/img1.jpg",
                "https://example.com/img2.jpg"
            ]
        }"#;

        let course: Course = serde_json::from_str(json).unwrap();
        assert!(course.image.is_some());
        if let Some(Value::Array(arr)) = course.image {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array of images");
        }
    }

    #[test]
    fn test_course_empty() {
        let json = r#"{}"#;
        let course: Course = serde_json::from_str(json).unwrap();
        assert!(course.name.is_none());
        assert!(course.description.is_none());
        assert!(course.provider.is_none());
        assert!(course.has_course_instance.is_none());
        assert!(course.teaches.is_none());
        assert!(course.course_prerequisites.is_none());
        assert!(course.educational_level.is_none());
        assert!(course.url.is_none());
        assert!(course.image.is_none());
        assert!(course.aggregate_rating.is_none());
        assert!(course.offers.is_none());
    }

    #[test]
    fn test_aggregaterating_basic() {
        let json = r#"{
            "ratingValue": "4.5"
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.5));
        assert!(rating.best_rating.is_none());
        assert!(rating.worst_rating.is_none());
        assert!(rating.rating_count.is_none());
        assert!(rating.review_count.is_none());
    }

    #[test]
    fn test_aggregaterating_with_bounds() {
        let json = r#"{
            "ratingValue": "4.2",
            "bestRating": "5",
            "worstRating": "1"
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.2));
        assert_eq!(rating.best_rating, Some(5.0));
        assert_eq!(rating.worst_rating, Some(1.0));
    }

    #[test]
    fn test_aggregaterating_with_counts() {
        let json = r#"{
            "ratingValue": "4.8",
            "ratingCount": "156",
            "reviewCount": "89"
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.8));
        assert_eq!(rating.rating_count, Some(156));
        assert_eq!(rating.review_count, Some(89));
    }

    #[test]
    fn test_aggregaterating_with_item() {
        let json = r#"{
            "ratingValue": "4.7",
            "itemReviewed": {
                "@type": "Product",
                "name": "Wireless Headphones"
            }
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.7));
        assert!(rating.item_reviewed.is_some());
    }

    #[test]
    fn test_aggregaterating_complete() {
        let json = r#"{
            "ratingValue": "4.6",
            "bestRating": "5",
            "worstRating": "1",
            "ratingCount": "245",
            "reviewCount": "187",
            "itemReviewed": {
                "@type": "Product",
                "name": "Premium Laptop"
            }
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.6));
        assert_eq!(rating.best_rating, Some(5.0));
        assert_eq!(rating.worst_rating, Some(1.0));
        assert_eq!(rating.rating_count, Some(245));
        assert_eq!(rating.review_count, Some(187));
        assert!(rating.item_reviewed.is_some());
    }

    #[test]
    fn test_aggregaterating_integer_values() {
        let json = r#"{
            "ratingValue": "4",
            "bestRating": "5",
            "ratingCount": "100"
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.0));
        assert_eq!(rating.best_rating, Some(5.0));
        assert_eq!(rating.rating_count, Some(100));
    }

    #[test]
    fn test_aggregaterating_decimal_precision() {
        let json = r#"{
            "ratingValue": "4.687",
            "bestRating": "5.0"
        }"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.rating_value, Some(4.687));
        assert_eq!(rating.best_rating, Some(5.0));
    }

    #[test]
    fn test_aggregaterating_empty() {
        let json = r#"{}"#;

        let rating: AggregateRating = serde_json::from_str(json).unwrap();
        assert!(rating.rating_value.is_none());
        assert!(rating.best_rating.is_none());
        assert!(rating.worst_rating.is_none());
        assert!(rating.rating_count.is_none());
        assert!(rating.review_count.is_none());
        assert!(rating.item_reviewed.is_none());
    }

    #[test]
    fn test_howto_deserialization() {
        let json = r#"{
            "name": "How to Change a Tire",
            "description": "A step-by-step guide to changing a car tire",
            "totalTime": "PT30M"
        }"#;

        let howto: HowTo = serde_json::from_str(json).unwrap();
        assert_eq!(howto.name, Some("How to Change a Tire".to_string()));
        assert_eq!(
            howto.description,
            Some("A step-by-step guide to changing a car tire".to_string())
        );
        assert_eq!(howto.total_time, Some("PT30M".to_string()));
    }

    #[test]
    fn test_howto_with_steps() {
        let json = r#"{
            "name": "How to Make Coffee",
            "step": [
                {
                    "@type": "HowToStep",
                    "name": "Boil water",
                    "text": "Bring water to a boil",
                    "position": 1
                },
                {
                    "@type": "HowToStep",
                    "name": "Add coffee",
                    "text": "Add ground coffee to filter",
                    "position": 2
                }
            ]
        }"#;

        let howto: HowTo = serde_json::from_str(json).unwrap();
        assert_eq!(howto.name, Some("How to Make Coffee".to_string()));
        assert!(howto.step.is_some());
        assert_eq!(howto.step.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_howto_with_tools_and_supplies() {
        let json = r#"{
            "name": "How to Build a Shelf",
            "tool": [
                {
                    "@type": "HowToTool",
                    "name": "Drill"
                },
                {
                    "@type": "HowToTool",
                    "name": "Screwdriver"
                }
            ],
            "supply": [
                {
                    "@type": "HowToSupply",
                    "name": "Wood boards"
                },
                {
                    "@type": "HowToSupply",
                    "name": "Screws"
                }
            ]
        }"#;

        let howto: HowTo = serde_json::from_str(json).unwrap();
        assert_eq!(howto.name, Some("How to Build a Shelf".to_string()));
        assert!(howto.tool.is_some());
        assert!(howto.supply.is_some());
    }

    #[test]
    fn test_howto_complete() {
        let json = r#"{
            "name": "How to Build a Planter Box",
            "description": "Complete guide to building a wooden planter box",
            "totalTime": "PT2H",
            "estimatedCost": {
                "@type": "MonetaryAmount",
                "currency": "USD",
                "value": "40.00"
            },
            "tool": [{"@type": "HowToTool", "name": "Saw"}],
            "supply": [{"@type": "HowToSupply", "name": "Cedar boards"}],
            "step": [{"@type": "HowToStep", "name": "Cut boards", "text": "Cut the cedar boards to size", "position": 1}],
            "url": "https://example.com/planter-box",
            "author": {"@type": "Person", "name": "Bob Builder"},
            "datePublished": "2024-03-15"
        }"#;

        let howto: HowTo = serde_json::from_str(json).unwrap();
        assert_eq!(howto.name, Some("How to Build a Planter Box".to_string()));
        assert!(howto.tool.is_some());
        assert!(howto.step.is_some());
    }

    #[test]
    fn test_howto_empty() {
        let json = r#"{}"#;
        let howto: HowTo = serde_json::from_str(json).unwrap();
        assert!(howto.name.is_none());
        assert!(howto.tool.is_none());
        assert!(howto.step.is_none());
    }

    #[test]
    fn test_movie_deserialization() {
        let json = r#"{
            "name": "Inception",
            "description": "A thief who steals corporate secrets",
            "duration": "PT2H28M"
        }"#;

        let movie: Movie = serde_json::from_str(json).unwrap();
        assert_eq!(movie.name, Some("Inception".to_string()));
        assert_eq!(movie.duration, Some("PT2H28M".to_string()));
    }

    #[test]
    fn test_movie_with_director() {
        let json = r#"{
            "name": "Interstellar",
            "director": {
                "@type": "Person",
                "name": "Christopher Nolan"
            }
        }"#;

        let movie: Movie = serde_json::from_str(json).unwrap();
        assert_eq!(movie.name, Some("Interstellar".to_string()));
        assert!(movie.director.is_some());
    }

    #[test]
    fn test_movie_complete() {
        let json = r#"{
            "name": "Inception",
            "description": "A thief who steals corporate secrets through dream-sharing",
            "duration": "PT2H28M",
            "contentRating": "PG-13",
            "countryOfOrigin": "USA"
        }"#;

        let movie: Movie = serde_json::from_str(json).unwrap();
        assert_eq!(movie.name, Some("Inception".to_string()));
        assert_eq!(movie.duration, Some("PT2H28M".to_string()));
        assert_eq!(movie.content_rating, Some("PG-13".to_string()));
        assert_eq!(movie.country_of_origin, Some("USA".to_string()));
    }

    #[test]
    fn test_movie_empty() {
        let json = r#"{}"#;
        let movie: Movie = serde_json::from_str(json).unwrap();
        assert!(movie.name.is_none());
        assert!(movie.duration.is_none());
        assert!(movie.director.is_none());
    }
}
