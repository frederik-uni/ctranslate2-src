// bart.rs
//
// Copyright (c) 2023-2024 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

//! Summarize a file using a BART model.
//!
//! This example uses the [BART](https://huggingface.co/facebook/bart-large-cnn) model
//! that was fine-tuned on CNN Daily Mail for text summarization.
//!
//! The original Python version of the code can be found in the
//! [CTranslate2 documentation](https://opennmt.net/CTranslate2/guides/transformers.html#bart).
//!
//! First, convert the model files with the following command:
//!
//! ```bash
//! pip install -U ctranslate2 huggingface_hub torch transformers
//!
//! ct2-transformers-converter --model facebook/bart-large-cnn --output_dir bart-large-cnn \
//!     --copy_files tokenizer.json
//! ```
//!
//! Note: The above command copies `tokenizer.json` because it is provided by the
//! [facebook/bart-large-cnn](https://huggingface.co/facebook/bart-large-cnn) repository.
//! If you prefer to use another repository that offers `source.spm` and `target.spm`,
//! you can copy it using the option `--copy_files source.spm target.spm`.
//!
//! Create a file named `prompt.txt`, write the sentence you want to translate into it,
//! and save the file.
//! Then, execute the sample code below with the following command:
//!
//! ```bash
//! cargo run --example bart -- ./bart-large-cnn
//! ```
//!

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

use anyhow::Result;
use clap::Parser;

use ctranslate2::tokenizer::rust_tokenizers::SentenceTokenizer;
use ctranslate2::{Device, Translator2, TranslatorConfig};

/// Summarize a file using a BART model.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file contains prompts.
    #[arg(short, long, value_name = "FILE", default_value = "prompt.txt")]
    prompt: String,
    /// Use CUDA.
    #[arg(short, long)]
    cuda: bool,
    /// Path to the directory that contains model.bin.
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = if args.cuda {
        TranslatorConfig {
            device: Device::Cuda,
            device_indices: vec![0],
            ..TranslatorConfig::default()
        }
    } else {
        TranslatorConfig::default()
    };

    let t = Translator2::new(&args.path, &cfg, SentenceTokenizer::new(&args.path))?;

    let source = BufReader::new(File::open(args.prompt)?).lines().try_fold(
        String::new(),
        |mut acc, line| {
            line.map(|l| {
                acc.push_str(&l);
                acc
            })
        },
    )?;

    let now = time::Instant::now();
    let res = t.translate_batch(&[source], Default::default())?;
    let elapsed = now.elapsed();

    for (res, _) in res {
        // Trim special tokens.
        println!("{}", res.replace("<s>", ""));
    }
    println!("Time taken: {elapsed:?}");

    Ok(())
}
