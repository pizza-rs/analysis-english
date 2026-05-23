#![cfg_attr(not(feature = "std"), no_std)]
//! English language analysis for Pizza search engine.
//!
//! Provides a full-featured English analyzer matching Elasticsearch/Lucene
//! conventions with KStem, possessive handling, and stop words.
//!
//! # Components
//!
//! - [`EnglishPossessiveFilter`] — Removes trailing 's from English possessives
//! - [`KStemFilter`] — Krovetz stemmer (less aggressive than Porter)
//! - [`EnglishStopFilter`] — English stop words filter
//! - [`ENGLISH_STOP_WORDS`] — Default English stop word list
extern crate alloc;
mod possessive;
mod stem;
mod stop;

pub mod register;

pub use possessive::EnglishPossessiveFilter;
pub use register::register_all;
pub use stem::KStemFilter;
pub use stop::EnglishStopFilter;
