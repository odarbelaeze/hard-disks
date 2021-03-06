use std::ops::Sub;


#[derive(Debug, Clone)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a, 'b> Sub<&'a Vector2> for &'b Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {x: x, y: y}
    }

    pub fn comps(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn zero() -> Vector2 {
        Vector2 {x: 0.0, y: 0.0}
    }

    pub fn distance_to(&self, other: &Vector2) -> f64 {
        (self - other).norm()
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalise(&self) -> Option<Vector2> {
        let norm = self.norm();
        match norm {
            0.0 => None,
            _ => Some( Vector2 {
                x: self.x / norm,
                y: self.y / norm,
            })
        }
    }
}


pub struct Box2 {
    ll: Vector2,
    ur: Vector2,
}


impl Box2 {
    pub fn new(ll: &Vector2, ur: &Vector2) -> Box2 {
        if ll.comps() > ur.comps() {
            panic!("Lower left should be lesser than upper right");
        }
        Box2 {
            ll: ll.clone(),
            ur: ur.clone(),
        }
    }

    pub fn rectangle(w: f64, h: f64) -> Box2 {
        if w < 0.0 || h < 0.0 {
            panic!("Negative width or height is something to panic at");
        }
        Box2 {
            ll: Vector2::zero(),
            ur: Vector2::new(w, h),
        }
    }

    pub fn contains(&self, vec: &Vector2) -> bool {
        vec.comps() >= self.ll.comps() && vec.comps() <= self.ur.comps()
    }
}
