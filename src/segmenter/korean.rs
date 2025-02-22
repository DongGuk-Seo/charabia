use crate::segmenter::Segmenter;
use lindera::mode::{Mode, Penalty};
use lindera::tokenizer::{Tokenizer, TokenizerConfig, DictionaryConfig, DictionaryKind};
use once_cell::sync::Lazy;

/// Korean specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct KoreanSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    let config =
        TokenizerConfig {dictionary: DictionaryConfig {
            kind: DictionaryKind::KoDic,
            path: None,
        },mode: Mode::Decompose(Penalty::default()),..TokenizerConfig::default()};
    Tokenizer::with_config(config).unwrap()
});

impl Segmenter for KoreanSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segment_iterator = LINDERA.tokenize(to_segment).unwrap();
        Box::new(segment_iterator.into_iter().map(|token| token.text))
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "오늘은월요일입니다";

    const SEGMENTED: &[&str] = &[
        "오늘",
        "은",
        "월요일",
        "입니다"
    ];

    const TOKENIZED: &[&str] = SEGMENTED;

    // Macro that run several tests on the Segmenter.
    test_segmenter!(KoreanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Hangul, Language::Kor);
}
