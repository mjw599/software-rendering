use std::ops::{Add, Sub, Mul};

pub struct Vec3f {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

pub struct Vec3i {
    x:i64,
    y:i64,
    z:i64
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