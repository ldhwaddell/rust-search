use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;
use std::time::SystemTime;

use super::extract::Extractor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Map of every term in the document and its repective frequency
    tf: HashMap<String, usize>,
    /// Time this document was indexed
    last_modified: SystemTime,
}

type Documents = HashMap<PathBuf, Document>;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Model {
    documents: Documents,
}

impl Model {
    pub fn from(path: &PathBuf) -> Result<Model, Box<dyn Error>> {
        if path.exists() {
            println!("Using model at: {:?}", path);
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let model: Model = serde_json::from_reader(reader)?;
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

        serde_json::to_writer(writer, self)?;
        println!("Saved!");
        Ok(())
    }

    // Should probably just return document...
    pub fn query(&self) -> Option<Vec<Document>> {
        None
    }

    // Gets a model path and a list of doc paths, parses and add
    pub fn add(&mut self, path: &PathBuf) -> io::Result<()> {
        let content = Extractor::extract(path)?;

        // Do indexing here
        println!("{}", content);

        // This will be done by lexer...

        let mut tf: HashMap<String, usize> = HashMap::new();

        for word in content.split_whitespace() {
            if let Some(freq) = tf.get_mut(word) {
                *freq += 1;
            } else {
                tf.insert(word.to_string(), 1);
            }
        }

        self.documents.insert(
            path.to_path_buf(),
            Document {
                tf,
                last_modified: SystemTime::now(),
            },
        );

        self.save(&PathBuf::from("./output.json"))?;

        Ok(())
    }
}
