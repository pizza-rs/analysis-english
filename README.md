<div align="center">

# 🇬🇧 pizza-analysis-english

**English text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--english-blue)](https://github.com/pizza-rs/analysis-english)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

English language analysis with possessive stripping, KStem algorithmic stemmer,
and stop word removal. KStem provides less aggressive stemming than Porter/Snowball,
preserving more semantic distinction between word forms.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `english_possessive` | Strip trailing `'s` and `s'` |
| TokenFilter | `kstem` | KStem algorithmic stemmer (lighter than Porter) |
| TokenFilter | `english_stop` | English stop words (174 entries) |
| Analyzer | `english` | Full pipeline: lowercase → possessive → kstem → stop |

### KStem vs Porter/Snowball

| Feature | KStem | Porter/Snowball |
|:--------|:------|:----------------|
| Aggressiveness | Conservative | Aggressive |
| Output | Usually valid English words | May produce non-words |
| `operating` | `operate` | `oper` |
| `organization` | `organization` | `organ` |

KStem is preferred when stem readability matters (e.g., highlighting, suggestions).

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_english::register_all(&mut factory);

let analyzer = factory.get_analyzer("english").unwrap();
// "the runner's dogs" → ["runner", "dog"]
```

## Installation

```toml
[dependencies]
pizza-analysis-english = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["english"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
