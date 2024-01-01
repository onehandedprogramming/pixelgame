use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::client::elements::ElementType;

lazy_static! {
    static ref REACTIONS: HashMap<ReactionKey, ChemicalReaction> = {
        let mut m = HashMap::new();

        m.insert(
            ReactionKey::new(ElementType::Sand, ElementType::Stone),
            ChemicalReaction {
                result: ElementType::Bendium,
            },
        );
        
        m
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReactionKey(ElementType, ElementType);


impl ReactionKey {
    fn new(a: ElementType, b: ElementType) -> Self {
        let (min, max) = if a <= b { (a, b) } else { (b, a) };
        ReactionKey(min, max)
    }
}

pub fn check_reaction(a: &ElementType, b: &ElementType) -> Option<ChemicalReaction> {
    REACTIONS.get(&ReactionKey::new(a.clone(), b.clone())).cloned()
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChemicalReaction {
    pub result: ElementType,
    // Add like heat release/ required and other stufg
}