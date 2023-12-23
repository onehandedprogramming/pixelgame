use lazy_static::lazy_static;

use super::world::{Element, Attribute, ElementColor};

lazy_static!{
    pub static ref AIR: Element = Element {
        name: "Air".into(),
        attributes: vec![Attribute::Air],
        color: ElementColor {
            r: 80.0 / 255.0,
            g: 180.0 / 255.0,
            b: 210.0 / 255.0,
        },
        heat: 0.0,
        moisture: 0.0,
        density: 0.05,
    };

    pub static ref WATER: Element = Element {
        name: "Water".into(),
        attributes: vec![Attribute::CanFall, Attribute::Liquid],
        color: ElementColor {
            r: 10.0 / 255.0,
            g: 10.0 / 255.0,
            b: 255.0 / 255.0,
        },
        heat: 0.0,
        moisture: 0.0,
        density: 1.0,
    };

    pub static ref SAND: Element = Element {
        name: "Sand".into(),
        attributes: vec![Attribute::CanFall, Attribute::Solid],
        color: ElementColor {
            r: 210.0 / 255.0,
            g: 190.0 / 255.0,
            b: 110.0 / 255.0,
        },
        heat: 0.0,
        moisture: 0.0,
        density: 5.0,
    };

    pub static ref STONE: Element = Element {
        name: "Stone".into(),
        attributes: vec![Attribute::Immovable, Attribute::Solid],
        color: ElementColor {
            r: 60.0 / 255.0,
            g: 60.0 / 255.0,
            b: 60.0 / 255.0,
        },
        heat: 0.0,
        moisture: 0.0,
        density: 10.0,
    };

    pub static ref STEAM: Element = Element {
        name: "Steam".into(),
        attributes: vec![Attribute::CanFall, Attribute::Gas],
        color: ElementColor {
            r: 150.0 / 255.0,
            g: 220.0 / 255.0,
            b: 230.0 / 255.0,
        },
        heat: 0.0,
        moisture: 0.0,
        density: 0.01,
    };
}