// raylib already implements vector2's but i already wrote this so
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        return Vector2 {
            x: x,
            y: y
        }
    }

    pub fn get_magnitude(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub fn in_range(self, other: Vector2, range: f32) -> bool {
        return (self - other).get_magnitude() <= range
    }

    pub fn multiply_by_f32(self, rhs: f32) -> Vector2 {
        return Vector2::new(self.x * rhs, self.y * rhs);
    }

    pub fn divide_by_f32(self, rhs: f32) -> Vector2 {
        // no illegal operations on my watch...
        let mut x = self.x;
        let mut y = self.y;
        if x == 0.0 { x = 0.01; }
        if y == 0.0 { y = 0.01; }
        return Vector2::new(x / rhs, y / rhs);
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        return Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        return Vector2::new(self.x * rhs.x, self.y * rhs.y);
    }
}

impl ops::Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        return Vector2::new(self.x / rhs.x, self.y / rhs.y);
    }
}


pub struct Boid {
    pub position: Vector2,
    pub velocity: Vector2,
}