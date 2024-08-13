use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::PathBuf;
use std::time::SystemTime;

use crate::lexer::Lexer;

use crate::extract::Extractor;

use crate::snowball::stem::stem;
use crate::snowball::StemmingAlgorithm;

// TF: The frequency of each term in an individual document
// DF: The number of occurrences of a term in the entire document set

type Documents = HashMap<PathBuf, Document>;
type TermFrequency = HashMap<String, usize>;
type DocumentFrequency = HashMap<String, usize>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Map of every term in the document and its respective frequency
    tf: TermFrequency,
    /// The total number of tokens in document
    count: usize,
    /// Time this document was indexed
    last_modified: SystemTime,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Model {
    /// A map of paths to respective Document struct
    documents: Documents,

    /// Map of each term in the corpus and its respective frequency
    df: DocumentFrequency,
}

impl Model {
    pub fn from(path: &PathBuf) -> Result<Model, Box<dyn Error>> {
        if path.exists() {
            println!("Using model at: {:?}", path);
            let content = fs::read_to_string(path)?;
            let model: Model = serde_json::from_str(&content)?;
            Ok(model)
        } else {
            Ok(Default::default())
        }
    }

    // Saves the updated model
    pub fn save(&self, path: &PathBuf) -> io::Result<()> {
        println!("Saving model to {:?}", path);

        let output = File::create(path)?;
        let writer = BufWriter::new(output);

        // Use pretty writer for dev
        serde_json::to_writer(writer, &self)?;
        println!("Saved!");
        Ok(())
    }

    // Should probably just return document...
    pub fn query(&self, query: &str) -> Vec<(f32, &PathBuf)> {
        let mut matches = Vec::new();

        // Turn the query into stemmed tokens
        let stemmed_tokens: Vec<_> = Lexer::new(query)
            .filter_map(|tok| stem(tok, StemmingAlgorithm::Porter2).map(|s| s.to_string()))
            .collect();

        for (path, doc) in &self.documents {
            let rank = stemmed_tokens.iter().fold(0.0, |score, tok| {
                score + compute_tf(&tok, doc) + compute_idf(self.documents.len(), &tok, &self.df)
            });

            // Only push document that have some form of match
            if rank > 0.0 {
                matches.push((rank, path));
            }
        }

        matches.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

        matches
    }

    // Gets a model path and a list of doc paths, parses and add
    pub fn add(&mut self, path: &PathBuf) -> io::Result<()> {
        let content = Extractor::extract(path)?;

        let lex = Lexer::new(&content);

        let stemmed_tokens = lex.filter_map(|tok| stem(tok, StemmingAlgorithm::Porter2));

        // Build out the term frequency map of the document, and the number of tokens it has
        let (tf, count) =
            stemmed_tokens.fold((TermFrequency::new(), 0), |(mut tf, mut count), tok| {
                count += 1;
                let tok = tok.to_string();
                if let Some(freq) = tf.get_mut(&tok) {
                    *freq += 1;
                } else {
                    tf.insert(tok, 1);
                }

                (tf, count)
            });

        // Now need to update document freq for the model
        for term in tf.keys() {
            if let Some(freq) = self.df.get_mut(term) {
                *freq += 1
            } else {
                self.df.insert(term.to_string(), 1);
            }
        }

        self.documents.insert(
            path.to_path_buf(),
            Document {
                tf,
                count,
                last_modified: SystemTime::now(),
            },
        );

        self.save(&PathBuf::from("./output.json"))?;

        Ok(())
    }

    pub fn remove(&mut self, path: &PathBuf) {
        if let Some(doc) = self.documents.remove(path) {
            // go through each of the documents keys
            for term in doc.tf.keys() {
                // Decrement in models docfreq
                if let Some(freq) = self.df.get_mut(term) {
                    *freq -= 1;
                }
            }
        }
    }
}

fn compute_tf(t: &str, d: &Document) -> f32 {
    // Total number of terms in document
    let count = d.count as f32;

    // Ensure div by 0 does not occur
    if count == 0.0 {
        return 0.0;
    }

    // Number of times t appears in d
    let freq = *d.tf.get(t).unwrap_or(&0) as f32;

    freq / count
}

fn compute_idf(n: usize, t: &str, df: &DocumentFrequency) -> f32 {
    // Total number of documents in the model
    let n = n as f32;

    // Total number of document in the model containing t
    let freq = *df.get(t).unwrap_or(&0) as f32;

    // Ensure inf doesnt get returned
    if freq == 0.0 {
        return 0.0;
    }

    (n / freq).log10()
}
