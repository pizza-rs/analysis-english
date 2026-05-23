//! Register English analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, Normalizer, StandardTokenizer, TokenFilter, Tokenizer,
    LowercaseNormalizer,
};

use crate::{EnglishPossessiveFilter, EnglishStopFilter, KStemFilter};

/// Register English token filters and the `"english"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("english_possessive", Box::new(EnglishPossessiveFilter::new()));
    factory.register_token_filter("kstem", Box::new(KStemFilter::new()));
    factory.register_token_filter("english_stop", Box::new(EnglishStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(EnglishPossessiveFilter::new()),
        Box::new(EnglishStopFilter::new()),
        Box::new(KStemFilter::new()),
    ];
    factory.register_analyzer("english", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("english_possessive").is_some());
        assert!(factory.get_token_filter("kstem").is_some());
        assert!(factory.get_token_filter("english_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("english").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("english").unwrap();
        let mut input = String::from("The cat's running quickly");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        // "The" is stop word, "cat's" → "cat" (possessive) → stem
        assert!(!tokens.iter().any(|t| t.term == "the"));
        assert!(!tokens.iter().any(|t| t.term.contains("'s")));
        assert!(tokens.len() >= 2);
    }
}
