
#[derive(Default)]
#[derive(Debug)]
#[allow(unused)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[allow(unused)]
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3{x, y, z};
    }

    pub fn list(&mut self) -> [f32; 3] {
        return [self.x, self.y, self.z];
    }

    pub fn length(&mut self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn add_vector(&mut self, u: Vec3) -> Vec3 {
        return Vec3::new(self.x + u.x, self.y + u.y, self.z + u.z)
    }

    pub fn divide(&mut self, div: f32) -> Vec3 {
        return Vec3::new(self.x / div, self.y / div, self.z / div);
    }

    pub fn normalize(&mut self) -> Vec3 {
        let length = self.length();
        return self.divide(length);
    }

    pub fn multiply(&mut self, multiplier: f32) -> Vec3 {
        return Vec3::new(self.x * multiplier, self.y * multiplier, self.z * multiplier);
    }

    pub fn scale(&mut self, length: f32) -> Vec3 {
        let mut norm = self.normalize();
        return norm.multiply(length);
    }
    pub fn copy(&mut self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z);
    }


}

#[derive(Default)]
#[derive(Debug)]
#[allow(unused)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[allow(unused)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        return Vec2{x, y};
    }

    pub fn list(&mut self) -> [f32; 2] {
        return [self.x, self.y];
    }

    pub fn add(&mut self, x: f32, y: f32) -> Vec2 {
        return Vec2::new(self.x + x, self.y + y);
    }

    pub fn add_vector(&mut self, u: Vec2) -> Vec2 {
        return Vec2::new(self.x + u.x, self.y + u.y);
    }

    pub fn length(&mut self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }

    pub fn divide(&mut self, div: f32) -> Vec2 {
        return Vec2::new(self.x / div, self.y / div);
    }

    pub fn multiply(&mut self, multiplier: f32) -> Vec2 {
        return Vec2::new(self.x * multiplier, self.y * multiplier);
    }

    pub fn normalize(&mut self) -> Vec2 {
        let length = self.length();
        return self.divide(length);
    }

    pub fn scale(&mut self, length: f32) -> Vec2 {
        return self.normalize().multiply(length);
    }

    pub fn copy(&mut self) -> Vec2 {
        return Vec2::new(self.x, self.y);
    }
}