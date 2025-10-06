use std::path::Path;

use crate::{
    Tokenizer, TranslationOptions, Translator, TranslatorConfig, translator::TranslatorError,
};

pub struct Translator2<T: Tokenizer> {
    t: Translator,
    tokenizer: T,
}

#[inline]
pub(crate) fn encode_all<T: Tokenizer, U: AsRef<str>>(
    tokenizer: &T,
    sources: &[U],
) -> anyhow::Result<Vec<Vec<String>>> {
    sources
        .iter()
        .map(|s| tokenizer.encode(s.as_ref()))
        .collect()
}

impl<T: Tokenizer> Translator2<T> {
    pub fn new<P: AsRef<Path>>(
        model_path: P,
        config: &TranslatorConfig,
        tokenizer: T,
    ) -> Result<Self, TranslatorError> {
        Ok(Translator2 {
            t: Translator::new(model_path, config)?,
            tokenizer,
        })
    }

    pub fn translate_batch(
        &self,
        sources: &[String],
        options: TranslationOptions,
    ) -> anyhow::Result<Vec<(String, f32)>> {
        let out = self
            .t
            .translate_batch(&encode_all(&self.tokenizer, sources)?, options)?;
        let mut res = Vec::new();
        for r in out.into_iter() {
            let score = r.score();
            res.push((
                self.tokenizer
                    .decode(r.output())
                    .map_err(|err| anyhow::anyhow!("failed to decode: {err}"))?,
                score,
            ));
        }
        Ok(res)
    }
}
