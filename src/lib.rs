use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Add;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Polygon {
    vertices: Vec<Vec<i32>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Directions> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Directions) -> Self {
        match rhs {
            Directions::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Directions::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Directions::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Directions::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Directions::None => self,
        }
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self: Self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Vec2 {
    pub fn to_vec(self: Self) -> Vec<i32> {
        vec![self.x, self.y]
    }
}

#[wasm_bindgen]
impl Polygon {
    pub fn from_enclosed_squares_string(squares: String, scaling: Option<i32>) -> Self {
        let scaling = scaling.unwrap_or(1);
        let squares = squares
            .split(&['|', ','][..])
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .chunks(2)
            .map(|p| Vec2 { x: p[0], y: p[1] })
            .collect::<Vec<Vec2>>();

        if squares.len() == 0 {
            return Polygon { vertices: vec![] };
        };

        let inside: HashSet<Vec2> = HashSet::from_iter(squares.clone());

        let initial_pos = squares.get(0).unwrap().clone();

        let mut p = Polygon {
            vertices: vec![initial_pos.to_vec()],
        };

        let mut cur_dir = Directions::Down;
        let mut prev_dir = Directions::None;

        let mut cur_pos = initial_pos;

        while cur_pos != initial_pos || prev_dir == Directions::None {
            prev_dir = cur_dir;
            cur_pos = cur_pos + prev_dir;
            match prev_dir {
                Directions::Down => {
                    if inside.contains(&(cur_pos + Directions::Left)) {
                        cur_dir = Directions::Left;
                    } else if !inside.contains(&cur_pos) {
                        cur_dir = Directions::Right;
                    }
                }
                Directions::Up => {
                    if inside.contains(&(cur_pos + Directions::Up)) {
                        cur_dir = Directions::Right;
                    } else if !inside.contains(&(cur_pos + Directions::Left + Directions::Up)) {
                        cur_dir = Directions::Left;
                    }
                }
                Directions::Left => {
                    if inside.contains(&(cur_pos + Directions::Left + Directions::Up)) {
                        cur_dir = Directions::Up;
                    } else if !inside.contains(&(cur_pos + Directions::Left)) {
                        cur_dir = Directions::Down;
                    }
                }
                Directions::Right => {
                    if inside.contains(&cur_pos) {
                        cur_dir = Directions::Down;
                    } else if !inside.contains(&(cur_pos + Directions::Up)) {
                        cur_dir = Directions::Up;
                    }
                }
                Directions::None => {}
            }

            if prev_dir != cur_dir {
                p.vertices.push((scaling * cur_pos).to_vec());
            }
        }
        return p;
    }
}
