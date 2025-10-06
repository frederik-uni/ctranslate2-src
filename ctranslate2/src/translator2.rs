use std::path::Path;

use crate::{
    Tokenizer, TranslationOptions, Translator, TranslatorConfig,
    tokenizer::rust_tokenizers::SentenceTokenizer, translator::TranslatorError,
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

    pub fn translate_batch_with_prefixes<U, V>(
        &self,
        sources: &[U],
        target_prefixes: &Vec<Vec<V>>,
        options: TranslationOptions,
    ) -> anyhow::Result<Vec<(String, f32)>>
    where
        U: AsRef<str>,
        V: AsRef<str>,
    {
        let out = self.t.translate_batch2(
            &encode_all(&self.tokenizer, sources)?,
            target_prefixes,
            options,
        )?;
        let mut res = Vec::new();
        for (r, prefix) in out.into_iter().zip(target_prefixes) {
            let score = r.score();
            let mut hypotheses = r.output();
            hypotheses.drain(0..prefix.len());

            res.push((
                self.tokenizer
                    .decode(hypotheses)
                    .map_err(|err| anyhow::anyhow!("failed to decode: {err}"))?,
                score,
            ));
        }
        Ok(res)
    }
}

#[test]
fn tessss() {
    let t = Translator2::new(Path::new("/Users/frederik/code/rust/ctranslate2-src/ctranslate2/model/ja-en-base"), &Default::default(), SentenceTokenizer::new("/Users/frederik/code/rust/ctranslate2-src/ctranslate2/model/spm.nopretok/spm.en.nopretok.model")).unwrap();
    t.translate_batch(&vec!["Hello World".to_owned()], Default::default());
}
