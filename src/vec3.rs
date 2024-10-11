#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            e: [0f64, 0f64, 0f64],
        }
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn flip_neg_vec(mut self) -> Self {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        self
    }

    pub fn add_vec(mut self, v: Vec3) -> Self {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
        self
    }

    pub fn multiply_vec(mut self, t: f64) -> Self {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
        self
    }

    pub fn divide_vec(self, t: f64) -> Self {
        let fraction = 1f64 / t;
        self.multiply_vec(fraction)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

pub fn make_from_adding_vecs(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
}

pub fn make_from_subtracting_vecs(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
}

pub fn make_from_multiplying_vecs(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
}

pub fn make_from_multiplying_num(t: f64, v: Vec3) -> Vec3 {
    Vec3::new(t * v.e[0], t * v.e[1], t * v.e[2])
}

pub fn make_from_dividing_num(v: Vec3, t: f64) -> Vec3 {
    make_from_multiplying_num(1f64 / t, v)
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    make_from_dividing_num(v, v.length())
}
