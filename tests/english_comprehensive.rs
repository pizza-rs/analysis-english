//! Comprehensive tests for pizza-analysis-english.

use pizza_analysis_english::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// EnglishPossessiveFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn possessive_construction() {
    let _f = EnglishPossessiveFilter::new();
}

#[test]
fn possessive_removes_trailing_s() {
    let f = EnglishPossessiveFilter::new();
    let mut token = make_token("john's");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "john");
}

#[test]
fn possessive_removes_uppercase() {
    let f = EnglishPossessiveFilter::new();
    let mut token = make_token("John's");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "John");
}

#[test]
fn possessive_no_change_without_apostrophe() {
    let f = EnglishPossessiveFilter::new();
    let mut token = make_token("books");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "books");
}

#[test]
fn possessive_empty_string() {
    let f = EnglishPossessiveFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn possessive_single_char() {
    let f = EnglishPossessiveFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// KStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn kstem_construction() {
    let _f = KStemFilter::new();
}

#[test]
fn kstem_plural_dogs() {
    let f = KStemFilter::new();
    let mut token = make_token("dogs");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "dog");
}

#[test]
fn kstem_plural_churches() {
    let f = KStemFilter::new();
    let mut token = make_token("churches");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "church");
}

#[test]
fn kstem_ing_suffix() {
    let f = KStemFilter::new();
    let mut token = make_token("running");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    // KStem should reduce -ing forms
    assert_ne!(token.term.as_ref(), "running");
}

#[test]
fn kstem_tion_suffix() {
    let f = KStemFilter::new();
    let mut token = make_token("connection");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn kstem_ly_suffix() {
    let f = KStemFilter::new();
    let mut token = make_token("quickly");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn kstem_short_word() {
    let f = KStemFilter::new();
    let mut token = make_token("go");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn kstem_empty_string() {
    let f = KStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// EnglishStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = EnglishStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = EnglishStopFilter::new();
    let stop_words = ["the", "is", "at", "which", "on", "a", "an", "and", "or", "not"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = EnglishStopFilter::new();
    let content_words = ["book", "house", "computer", "search"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = EnglishStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("english_possessive").is_some());
    assert!(factory.get_token_filter("kstem").is_some());
    assert!(factory.get_token_filter("english_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("english").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("english").unwrap();
    let mut input = String::from("The quick brown fox jumps over the lazy dog");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("english").unwrap();
    let mut input = String::from("the cat is on the mat");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"the"));
    assert!(!terms.contains(&"is"));
    assert!(!terms.contains(&"on"));
}

#[test]
fn analyzer_pipeline_possessives_stripped() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("english").unwrap();
    let mut input = String::from("john's book");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.iter().any(|t| t.contains("'")));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("english").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_single_word() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("english").unwrap();
    let mut input = String::from("searching");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
