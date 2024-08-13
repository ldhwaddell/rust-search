use std::borrow::Cow;

use super::algorithms::{porter, porter2};
use super::SnowballEnv;
use super::StemmingAlgorithm;
use crate::lexer::Token;

pub fn stem(tok: Token, alg: StemmingAlgorithm) -> Option<Cow<str>> {
    let tok_content = match tok {
        Token::Word(word) => Some(word),
        Token::Number(num) => Some(num),
        _ => None,
    };

    if let Some(content) = tok_content {
        let mut env = SnowballEnv::create(content);

        match alg {
            StemmingAlgorithm::Porter => porter::stem(&mut env),
            StemmingAlgorithm::Porter2 => porter2::stem(&mut env),
        };

        let stemmed_content = env.get_current();

        Some(stemmed_content)
    } else {
        None
    }
}
