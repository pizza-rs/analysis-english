# pizza-analysis-english

English language analysis with possessive filter, KStem stemmer (Krovetz), and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `english_possessive` | Token Filter | Strips trailing 's from English possessives |
| `kstem` | Token Filter | Krovetz stemmer — produces valid English words as stems, handles irregulars |
| `english_stop` | Token Filter | English stop words filter (174 words) |
| `english` | Analyzer | Full pipeline: lowercase → possessive → stop → kstem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "english"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["english_possessive", "kstem", "english_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
