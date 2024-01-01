use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub enum ElementType {
    Air,
    Water,
    Sand,
    Dirt,
    Stone,
    Metal,
    Steam,
    Robustium,
    Bendium,
}

lazy_static! {
    pub static ref DEF_ELEMS: HashMap<ElementType, Element> = {
        let mut m = HashMap::new();
        m.insert(
            ElementType::Air,
            Element::new(
                "Air",
                ElementType::Air,
                vec![Attribute::Air],
                ElementColor {
                    r: 80.0 / 255.0,
                    g: 180.0 / 255.0,
                    b: 210.0 / 255.0,
                    rv: 0.0,
                    gv: 0.0,
                    bv: 0.0,
                    dv: 0.0,
                    ..Default::default()
                },
                0.0,
                0.0,
                0.12,
            ),
        );
        m.insert(
            ElementType::Water,
            Element::new(
                "Water",
                ElementType::Water,
                vec![
                    Attribute::CanFall,
                    Attribute::Liquid,
                    Attribute::CanEvaporate(ElementType::Steam),
                    Attribute::Sparkle,
                ],
                ElementColor {
                    r: 10.0 / 255.0,
                    g: 80.0 / 255.0,
                    b: 235.0 / 255.0,
                    rv: 0.00,
                    gv: 0.00,
                    bv: 0.0,
                    dv: 0.004,
                    max_dist: 0.015,
                },
                0.0,
                0.0,
                1.0,
            ),
        );
        m.insert(
            ElementType::Sand,
            Element::new(
                "Sand",
                ElementType::Sand,
                vec![Attribute::CanFall, Attribute::Solid],
                ElementColor {
                    r: 210.0 / 255.0,
                    g: 190.0 / 255.0,
                    b: 110.0 / 255.0,
                    rv: 0.1,
                    gv: 0.01,
                    bv: 0.01,
                    dv: 0.08,
                    ..Default::default()
                },
                0.0,
                0.0,
                1.5,
            ),
        );
        m.insert(
            ElementType::Dirt,
            Element::new(
                "Dirt",
                ElementType::Dirt,
                vec![Attribute::CanFall, Attribute::Solid],
                ElementColor {
                    r: 26.0 / 255.0,
                    g: 15.0 / 255.0,
                    b: 7.3 / 255.0,
                    rv: 0.005,
                    gv: 0.005,
                    bv: 0.00,
                    dv: 0.02,
                    ..Default::default()
                },
                0.0,
                0.0,
                1.2,
            ),
        );
        m.insert(
            ElementType::Stone,
            Element::new(
                "Stone",
                ElementType::Stone,
                vec![Attribute::Immovable, Attribute::Solid],
                ElementColor {
                    r: 40.0 / 255.0,
                    g: 40.0 / 255.0,
                    b: 40.0 / 255.0,
                    rv: 0.01,
                    gv: 0.02,
                    bv: 0.03,
                    dv: 0.1,
                    ..Default::default()
                },
                0.0,
                0.0,
                2.67,
            ),
        );
        m.insert(
            ElementType::Metal,
            Element::new(
                "Metal",
                ElementType::Metal,
                vec![Attribute::Immovable, Attribute::Solid, Attribute::Conductive],
                ElementColor {
                    r: 100.0 / 255.0,
                    g: 100.0 / 255.0,
                    b: 110.0 / 255.0,
                    rv: 0.02,
                    gv: 0.02,
                    bv: 0.03,
                    dv: 0.05,
                    ..Default::default()
                },
                0.0,
                0.0,
                7.8,
            ),
        );
        m.insert(
            ElementType::Steam,
            Element::new(
                "Steam",
                ElementType::Steam,
                vec![
                    Attribute::Gas,
                    Attribute::CanCondensate(ElementType::Water),
                ],
                ElementColor {
                    r: 150.0 / 255.0,
                    g: 220.0 / 255.0,
                    b: 230.0 / 255.0,
                    rv: 0.01,
                    gv: 0.03,
                    bv: 0.07,
                    dv: 0.01,
                    ..Default::default()
                },
                0.0,
                0.0,
                0.08,
            ),
        );
        
        m.insert(
            ElementType::Robustium,
            Element::new(
                "Robustium",
                ElementType::Robustium,
                vec![Attribute::PillarLike(0.08), Attribute::CanFall],
                ElementColor {
                    r: 7.5 / 255.0,
                    g: 25.0 / 255.0,
                    b: 5.5 / 255.0,
                    rv: 0.00,
                    gv: 0.05,
                    bv: 0.00,
                    dv: 0.05,
                    ..Default::default()
                },
                0.0,
                0.0,
                1.7,
            ),
        );
        m.insert(
            ElementType::Bendium,
            Element::new(
                "Bendium",
                ElementType::Bendium,
                vec![
                    Attribute::CanFall,
                    Attribute::Liquid,
                    Attribute::Sparkle,
                ],
                ElementColor {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    rv: 0.06,
                    gv: 0.04,
                    bv: 0.01,
                    dv: 0.0,
                    max_dist: 0.5
                },
                0.0,
                0.0,
                1.05,
            ),
        );
        m
    };
}

#[macro_export]
macro_rules! get_element {
    ($element_type:expr) => {
        crate::client::elements::DEF_ELEMS
            .get(&$element_type)
            .unwrap()
            .create()
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    CanEvaporate(ElementType),
    CanCondensate(ElementType),
    PillarLike(f32),
    Conductive,
    CanFall,
    Solid,
    Liquid,
    Gas,
    Immovable,
    Sparkle,
    Air,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub rv: f32,
    pub gv: f32,
    pub bv: f32,
    pub dv: f32,
    pub max_dist: f32,
}

impl ElementColor {
    pub fn new(r: f32, g: f32, b: f32, rv: f32, gv: f32, bv: f32, dv: f32, max_dist: f32) -> Self {
        ElementColor {
            r,
            g,
            b,
            rv,
            gv,
            bv,
            dv,
            max_dist,
        }
    }
}

impl Default for ElementColor {
    fn default() -> Self {
        ElementColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            rv: 0.0,
            gv: 0.0,
            bv: 0.0,
            dv: 0.0,
            max_dist: 5.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub name: Box<str>,
    pub id: ElementType,
    pub attributes: Vec<Attribute>,
    pub color: ElementColor,
    render_color: RenderColor,
    pub heat: f32,
    pub moisture: f32,
    pub density: f32,
    pub falling: bool,
}

impl Element {
    pub fn new(
        name: &str,
        id: ElementType,
        attributes: Vec<Attribute>,
        color: ElementColor,
        heat: f32,
        moisture: f32,
        density: f32,
    ) -> Self {
        Element {
            name: name.into(),
            id,
            attributes,
            color,
            render_color: RenderColor {
                r: color.r,
                g: color.g,
                b: color.b,
            },
            heat,
            moisture,
            density,
            falling: true,
        }
    }

    pub fn render(&self) -> u32 {
        let r = (self.render_color.r * 255.0) as u32;
        let g = (self.render_color.g * 255.0) as u32;
        let b = (self.render_color.b * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }

    pub fn create(&self) -> Self {
        let mut element = self.clone();
        element.vary_color();
        element
    }

    pub fn vary_color(&mut self) {
        let mut rng = rand::thread_rng();

        let darken_delta = rng.gen_range(-self.color.dv..=self.color.dv);
        let adjust_color = |color: f32, variance: f32| -> f32 {
            let mut rng = rand::thread_rng();
            let delta = rng.gen_range(-variance..=variance);
            let new_color = color + delta + darken_delta;
            new_color.clamp(0.0, 1.0)
        };

        let new_r = adjust_color(self.render_color.r, self.color.rv);
        let new_g = adjust_color(self.render_color.g, self.color.gv);
        let new_b = adjust_color(self.render_color.b, self.color.bv);

        let distance = ((new_r - self.color.r).powi(2)
            + (new_g - self.color.g).powi(2)
            + (new_b - self.color.b).powi(2))
        .sqrt();

        if distance > self.color.max_dist {
            let scale = self.color.max_dist / distance;
            self.render_color.r = self.color.r + (new_r - self.color.r) * scale;
            self.render_color.g = self.color.g + (new_g - self.color.g) * scale;
            self.render_color.b = self.color.b + (new_b - self.color.b) * scale;
        } else {
            self.render_color.r = new_r;
            self.render_color.g = new_g;
            self.render_color.b = new_b;
        }
    }
}