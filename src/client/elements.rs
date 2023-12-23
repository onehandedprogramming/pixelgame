use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ElementType {
    Air,
    Water,
    Sand,
    Stone,
    Steam,
}

lazy_static! {
    pub static ref DEF_ELEMS: HashMap<ElementType, Element> = {
        let mut m = HashMap::new();
        m.insert(
            ElementType::Air,
            Element {
                id: "Air".into(),
                attributes: vec![Attribute::Air],
                color: ElementColor {
                    r: 80.0 / 255.0,
                    g: 180.0 / 255.0,
                    b: 210.0 / 255.0,
                },
                heat: 0.0,
                moisture: 0.0,
                density: 0.05,
            },
        );
        m.insert(
            ElementType::Water,
            Element {
                id: "Water".into(),
                attributes: vec![
                    Attribute::CanFall,
                    Attribute::Liquid,
                    Attribute::CanEvaporate(ElementType::Steam),
                ],
                color: ElementColor {
                    r: 10.0 / 255.0,
                    g: 10.0 / 255.0,
                    b: 255.0 / 255.0,
                },
                heat: 0.0,
                moisture: 0.0,
                density: 1.0,
            },
        );
        m.insert(
            ElementType::Sand,
            Element {
                id: "Sand".into(),
                attributes: vec![Attribute::CanFall, Attribute::Solid],
                color: ElementColor {
                    r: 210.0 / 255.0,
                    g: 190.0 / 255.0,
                    b: 110.0 / 255.0,
                },
                heat: 0.0,
                moisture: 0.0,
                density: 5.0,
            },
        );
        m.insert(
            ElementType::Stone,
            Element {
                id: "Stone".into(),
                attributes: vec![Attribute::Immovable, Attribute::Solid],
                color: ElementColor {
                    r: 60.0 / 255.0,
                    g: 60.0 / 255.0,
                    b: 60.0 / 255.0,
                },
                heat: 0.0,
                moisture: 0.0,
                density: 10.0,
            },
        );
        m.insert(
            ElementType::Steam,
            Element {
                id: "Steam".into(),
                attributes: vec![
                    Attribute::CanFall,
                    Attribute::Gas,
                    Attribute::CanCondensate(ElementType::Water),
                ],
                color: ElementColor {
                    r: 150.0 / 255.0,
                    g: 220.0 / 255.0,
                    b: 230.0 / 255.0,
                },
                heat: 0.0,
                moisture: 0.0,
                density: 0.01,
            },
        );
        m
    };
}

#[macro_export]
macro_rules! get_element {
    ($element_type:expr) => {
        crate::client::elements::DEF_ELEMS.get(&$element_type).unwrap().clone()
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    CanEvaporate(ElementType),
    CanCondensate(ElementType),
    CanFall,
    Solid,
    Liquid,
    Gas,
    Immovable,
    Air,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub id: Box<str>,
    pub attributes: Vec<Attribute>,
    pub color: ElementColor,
    pub heat: f32,
    pub moisture: f32,
    pub density: f32,
}

impl Element {
    pub fn render(&self) -> u32 {
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}
