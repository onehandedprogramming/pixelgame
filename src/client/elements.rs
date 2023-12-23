use lazy_static::lazy_static;

lazy_static! {
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
        attributes: vec![Attribute::CanFall, Attribute::Liquid, Attribute::CanEvaporate(STEAM.clone())],
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

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    CanEvaporate(Element),
    CanCondensate(Element),
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
    pub name: Box<str>,
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
