use std::ops::{Add, Sub, Mul};

pub struct Vec3f {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

pub struct Vec3i {
    pub x:i64,
    pub y:i64,
    pub z:i64
}

pub struct Vec2i {
    pub x:i64,
    pub y:i64
}

impl Vec3f {
    pub fn zero() -> Vec3f {
        Vec3f { x:0.0, y:0.0, z:0.0 }
    }

    pub fn cross(&self, other:&Vec3f) -> Vec3f {
        Vec3f { x: self.y * other.z - self.z * other.y,
                  y: self.z * other.x - self.x * other.z,
                  z: self.x * other.y - self.y * other.x }
    }

    pub fn dot(&self, other:&Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Vec3f {
        let size = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3f{ x: self.x / size, y: self.y / size, z: self.z / size }
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other:Vec3f) -> Self::Output {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other:Vec3f) -> Self::Output {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub for &Vec3f {
    type Output = Vec3f;

    fn sub(self, other:&Vec3f) -> Self::Output {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;

    fn mul(self, other:f64) -> Self::Output {
        Vec3f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Vec3i {
    pub fn zero() -> Vec3i {
        Vec3i { x:0, y:0, z:0 }
    }

    pub fn cross(&self, other:&Vec3i) -> Vec3i {
        Vec3i { x: self.y * other.z - self.z * other.y,
                  y: self.z * other.x - self.x * other.z,
                  z: self.x * other.y - self.y * other.x }
    }

    pub fn dot(&self, other:&Vec3i) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec3i {
    type Output = Vec3i;

    fn add(self, other:Vec3i) -> Self::Output {
        Vec3i {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3i {
    type Output = Vec3i;

    fn sub(self, other:Vec3i) -> Self::Output {
        Vec3i {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<i64> for Vec3i {
    type Output = Vec3i;

    fn mul(self, other:i64) -> Self::Output {
        Vec3i {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Vec2i {
    pub fn zero() -> Vec2i {
        Vec2i { x:0, y:0 }
    }
}

impl Add for Vec2i {
    type Output = Vec2i;

    fn add(self, other:Vec2i) -> Self::Output {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, other:Vec2i) -> Self::Output {
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<i64> for Vec2i {
    type Output = Vec2i;

    fn mul(self, other:i64) -> Self::Output {
        Vec2i {
            x: self.x * other,
            y: self.y * other
        }
    }
}