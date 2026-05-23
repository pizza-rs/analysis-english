//! KStem (Krovetz stemmer) for English.
//!
//! A less aggressive stemmer than Porter that produces stems which are
//! typically valid English words.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// KStem token filter — Krovetz stemmer for English.
///
/// Produces stems that are actual English words, making it suitable for
/// search applications where readability of highlighted terms matters.
#[derive(Clone, Debug, Default)]
pub struct KStemFilter;

impl KStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for KStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let word = token.term.as_ref();
        if word.len() < 3 {
            return (false, None);
        }
        if let Some(stemmed) = kstem(word) {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn kstem(word: &str) -> Option<String> {
    let lower: String = word.to_lowercase();
    let bytes = lower.as_bytes();

    // Check irregulars first
    if let Some(s) = lookup_irregular(&lower) {
        return Some(s.to_owned());
    }

    if let Some(s) = handle_plurals(&lower, bytes) {
        return Some(s);
    }
    if let Some(s) = handle_past_tense(&lower, bytes) {
        return Some(s);
    }
    if let Some(s) = handle_aspect(&lower, bytes) {
        return Some(s);
    }
    if let Some(s) = handle_ity_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ness_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_tion_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ment_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ble_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ly_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ful_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ous_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ive_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_ize_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_al_suffix(&lower) {
        return Some(s);
    }
    if let Some(s) = handle_er_suffix(&lower) {
        return Some(s);
    }

    if lower != word {
        Some(lower)
    } else {
        None
    }
}

fn handle_plurals(word: &str, bytes: &[u8]) -> Option<String> {
    let len = bytes.len();
    if len < 4 {
        return None;
    }

    if word.ends_with("ies") && len > 4 {
        return Some(word[..len - 3].to_owned() + "y");
    }

    if word.ends_with("es") && len > 3 {
        if word.ends_with("sses") {
            return Some(word[..len - 2].to_owned());
        }
        if word.ends_with("shes")
            || word.ends_with("ches")
            || word.ends_with("xes")
            || word.ends_with("zes")
        {
            return Some(word[..len - 2].to_owned());
        }
        let stem = &word[..len - 2];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }

    if bytes[len - 1] == b's' && bytes[len - 2] != b's' && len > 3 {
        return Some(word[..len - 1].to_owned());
    }

    None
}

fn handle_past_tense(word: &str, bytes: &[u8]) -> Option<String> {
    let len = bytes.len();
    if !word.ends_with("ed") || len < 5 {
        return None;
    }

    if word.ends_with("ied") && len > 4 {
        return Some(word[..len - 3].to_owned() + "y");
    }
    if word.ends_with("eed") {
        return Some(word[..len - 1].to_owned());
    }

    if len > 4 {
        let c1 = bytes[len - 3];
        let c2 = bytes[len - 4];
        if c1 == c2 && !is_vowel(c1) && c1 != b'l' && c1 != b's' && c1 != b'z' {
            return Some(word[..len - 3].to_owned());
        }
    }

    let stem = &word[..len - 2];
    if stem.len() >= 3 {
        if needs_e(stem) {
            return Some(stem.to_owned() + "e");
        }
        return Some(stem.to_owned());
    }
    None
}

fn handle_aspect(word: &str, bytes: &[u8]) -> Option<String> {
    let len = bytes.len();
    if !word.ends_with("ing") || len < 5 {
        return None;
    }

    if len > 5 {
        let c1 = bytes[len - 4];
        let c2 = bytes[len - 5];
        if c1 == c2 && !is_vowel(c1) && c1 != b'l' && c1 != b's' && c1 != b'z' {
            return Some(word[..len - 4].to_owned());
        }
    }

    let stem = &word[..len - 3];
    if stem.len() >= 3 {
        if needs_e(stem) {
            return Some(stem.to_owned() + "e");
        }
        return Some(stem.to_owned());
    }
    None
}

fn handle_ity_suffix(word: &str) -> Option<String> {
    if word.ends_with("ity") && word.len() > 5 {
        let stem = &word[..word.len() - 3];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ness_suffix(word: &str) -> Option<String> {
    if word.ends_with("ness") && word.len() > 6 {
        let mut stem = word[..word.len() - 4].to_owned();
        if stem.ends_with('i') {
            stem.pop();
            stem.push('y');
        }
        if stem.len() >= 3 {
            return Some(stem);
        }
    }
    None
}

fn handle_tion_suffix(word: &str) -> Option<String> {
    if word.ends_with("tion") && word.len() > 6 {
        let stem = &word[..word.len() - 4];
        if stem.len() >= 3 {
            if word.ends_with("ation") && word.len() > 7 {
                return Some(word[..word.len() - 5].to_owned() + "ate");
            }
            return Some(stem.to_owned() + "t");
        }
    }
    None
}

fn handle_ment_suffix(word: &str) -> Option<String> {
    if word.ends_with("ment") && word.len() > 6 {
        let stem = &word[..word.len() - 4];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ble_suffix(word: &str) -> Option<String> {
    if word.ends_with("able") && word.len() > 6 {
        let stem = &word[..word.len() - 4];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    if word.ends_with("ible") && word.len() > 6 {
        let stem = &word[..word.len() - 4];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ly_suffix(word: &str) -> Option<String> {
    if word.ends_with("ly") && word.len() > 4 {
        let stem = &word[..word.len() - 2];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ful_suffix(word: &str) -> Option<String> {
    if word.ends_with("ful") && word.len() > 5 {
        let stem = &word[..word.len() - 3];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ous_suffix(word: &str) -> Option<String> {
    if word.ends_with("ous") && word.len() > 5 {
        let stem = &word[..word.len() - 3];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ive_suffix(word: &str) -> Option<String> {
    if word.ends_with("ive") && word.len() > 5 {
        let stem = &word[..word.len() - 3];
        if stem.len() >= 3 {
            if word.ends_with("ative") && word.len() > 7 {
                return Some(word[..word.len() - 5].to_owned() + "ate");
            }
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_ize_suffix(word: &str) -> Option<String> {
    if (word.ends_with("ize") || word.ends_with("ise")) && word.len() > 5 {
        let stem = &word[..word.len() - 3];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_al_suffix(word: &str) -> Option<String> {
    if word.ends_with("al") && word.len() > 5 && !word.ends_with("ial") {
        let stem = &word[..word.len() - 2];
        if stem.len() >= 3 {
            return Some(stem.to_owned());
        }
    }
    None
}

fn handle_er_suffix(word: &str) -> Option<String> {
    let len = word.len();
    if !word.ends_with("er") || len < 5 {
        return None;
    }
    // Avoid "her", "over", etc.
    let bytes = word.as_bytes();
    if len > 4 {
        let c1 = bytes[len - 3];
        let c2 = bytes[len - 4];
        // doubled consonant before -er: runner → run
        if c1 == c2 && !is_vowel(c1) && c1 != b'l' && c1 != b's' && c1 != b'z' {
            return Some(word[..len - 3].to_owned());
        }
    }
    let stem = &word[..len - 2];
    if stem.len() >= 3 {
        if needs_e(stem) {
            return Some(stem.to_owned() + "e");
        }
        return Some(stem.to_owned());
    }
    None
}

/// Lookup common English irregular forms.
fn lookup_irregular(word: &str) -> Option<&'static str> {
    match word {
        // Irregular plurals
        "mice" => Some("mouse"),
        "geese" => Some("goose"),
        "teeth" => Some("tooth"),
        "feet" => Some("foot"),
        "children" => Some("child"),
        "men" => Some("man"),
        "women" => Some("woman"),
        "oxen" => Some("ox"),
        "people" => Some("person"),
        "leaves" => Some("leaf"),
        "lives" => Some("life"),
        "knives" => Some("knife"),
        "wives" => Some("wife"),
        "halves" => Some("half"),
        "wolves" => Some("wolf"),
        "shelves" => Some("shelf"),
        "loaves" => Some("loaf"),
        "thieves" => Some("thief"),
        // Irregular verbs - past tense
        "ran" => Some("run"),
        "went" => Some("go"),
        "gone" => Some("go"),
        "saw" => Some("see"),
        "seen" => Some("see"),
        "took" => Some("take"),
        "taken" => Some("take"),
        "gave" => Some("give"),
        "given" => Some("give"),
        "came" => Some("come"),
        "wrote" => Some("write"),
        "written" => Some("write"),
        "drove" => Some("drive"),
        "driven" => Some("drive"),
        "spoke" => Some("speak"),
        "spoken" => Some("speak"),
        "broke" => Some("break"),
        "broken" => Some("break"),
        "chose" => Some("choose"),
        "chosen" => Some("choose"),
        "froze" => Some("freeze"),
        "frozen" => Some("freeze"),
        "woke" => Some("wake"),
        "woken" => Some("wake"),
        "knew" => Some("know"),
        "known" => Some("know"),
        "grew" => Some("grow"),
        "grown" => Some("grow"),
        "threw" => Some("throw"),
        "thrown" => Some("throw"),
        "drew" => Some("draw"),
        "drawn" => Some("draw"),
        "flew" => Some("fly"),
        "flown" => Some("fly"),
        "blew" => Some("blow"),
        "blown" => Some("blow"),
        "fell" => Some("fall"),
        "fallen" => Some("fall"),
        "began" => Some("begin"),
        "begun" => Some("begin"),
        "sang" => Some("sing"),
        "sung" => Some("sing"),
        "swam" => Some("swim"),
        "swum" => Some("swim"),
        "rang" => Some("ring"),
        "rung" => Some("ring"),
        "drank" => Some("drink"),
        "drunk" => Some("drink"),
        "sank" => Some("sink"),
        "sunk" => Some("sink"),
        "shook" => Some("shake"),
        "shaken" => Some("shake"),
        "forgot" => Some("forget"),
        "forgotten" => Some("forget"),
        "got" => Some("get"),
        "gotten" => Some("get"),
        "hid" => Some("hide"),
        "hidden" => Some("hide"),
        "rode" => Some("ride"),
        "ridden" => Some("ride"),
        "rose" => Some("rise"),
        "risen" => Some("rise"),
        "tore" => Some("tear"),
        "torn" => Some("tear"),
        "wore" => Some("wear"),
        "worn" => Some("wear"),
        "bore" => Some("bear"),
        "borne" => Some("bear"),
        "bit" => Some("bite"),
        "bitten" => Some("bite"),
        "ate" => Some("eat"),
        "eaten" => Some("eat"),
        "lay" => Some("lie"),
        "lain" => Some("lie"),
        "sat" => Some("sit"),
        "stood" => Some("stand"),
        "held" => Some("hold"),
        "told" => Some("tell"),
        "sold" => Some("sell"),
        "found" => Some("find"),
        "bound" => Some("bind"),
        "wound" => Some("wind"),
        "ground" => Some("grind"),
        "hung" => Some("hang"),
        "stuck" => Some("stick"),
        "struck" => Some("strike"),
        "slept" => Some("sleep"),
        "kept" => Some("keep"),
        "swept" => Some("sweep"),
        "wept" => Some("weep"),
        "crept" => Some("creep"),
        "felt" => Some("feel"),
        "dealt" => Some("deal"),
        "meant" => Some("mean"),
        "leapt" => Some("leap"),
        "dreamt" => Some("dream"),
        "burnt" => Some("burn"),
        "learnt" => Some("learn"),
        "built" => Some("build"),
        "sent" => Some("send"),
        "spent" => Some("spend"),
        "lent" => Some("lend"),
        "bent" => Some("bend"),
        "lost" => Some("lose"),
        "shot" => Some("shoot"),
        "led" => Some("lead"),
        "fed" => Some("feed"),
        "bred" => Some("breed"),
        "bled" => Some("bleed"),
        "fled" => Some("flee"),
        "sped" => Some("speed"),
        "met" => Some("meet"),
        "said" => Some("say"),
        "paid" => Some("pay"),
        "laid" => Some("lay"),
        "made" => Some("make"),
        "thought" => Some("think"),
        "brought" => Some("bring"),
        "bought" => Some("buy"),
        "caught" => Some("catch"),
        "fought" => Some("fight"),
        "sought" => Some("seek"),
        "taught" => Some("teach"),
        "had" => Some("have"),
        "was" | "were" | "been" => Some("be"),
        "did" | "done" => Some("do"),
        _ => None,
    }
}

#[inline]
fn is_vowel(b: u8) -> bool {
    matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn needs_e(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    let len = bytes.len();
    if len < 2 {
        return false;
    }
    let last = bytes[len - 1];
    let prev = bytes[len - 2];
    !is_vowel(last) && is_vowel(prev) && (len < 3 || !is_vowel(bytes[len - 3]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stem(word: &str) -> String {
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let filter = KStemFilter::new();
        filter.filter(&mut token);
        token.term.into_owned()
    }

    #[test]
    fn test_plurals() {
        assert_eq!(stem("ponies"), "pony");
        assert_eq!(stem("cats"), "cat");
    }

    #[test]
    fn test_past_tense() {
        assert_eq!(stem("agreed"), "agree");
        assert_eq!(stem("studied"), "study");
    }

    #[test]
    fn test_ing() {
        assert_eq!(stem("running"), "run");
        assert_eq!(stem("making"), "make");
    }

    #[test]
    fn test_ness() {
        assert_eq!(stem("happiness"), "happy");
        assert_eq!(stem("darkness"), "dark");
    }

    #[test]
    fn test_short_words_unchanged() {
        assert_eq!(stem("go"), "go");
        assert_eq!(stem("an"), "an");
    }
}
