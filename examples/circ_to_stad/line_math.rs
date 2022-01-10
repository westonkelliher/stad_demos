#![allow(dead_code)]

// Point
use ggez::mint::Point2;


pub fn distance(a: Point2<f32>, b: Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}
pub fn distance_ref(a: Point2<f32>, b: Point2<f32>) -> f32 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).powf(0.5)
}
pub fn length(a: Point2<f32>) -> f32 {
    distance(a, Point2{x: 0.0, y: 0.0})
}

pub fn plus(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
	x: a.x + b.x,
	y: a.y + b.y,
    }
}

pub fn minus(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2 {
	x: a.x - b.x,
	y: a.y - b.y,
    }
}

pub fn scaled(a: Point2<f32>, s: f32) -> Point2<f32> {
    Point2 {
	x: a.x * s,
	y: a.y * s,
    }
}

pub fn normalized(a: Point2<f32>) -> Point2<f32> {
    scaled(a, 1.0/length(a))
}


pub fn point_is_above_line(p: Point2<f32>, l: Line) -> bool {
    let x_on = l.x(p.y);
    p.x > x_on
}


// Line
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
// `ax + by + c = 0`
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Line {
    
    // the line that passes through points p1 and p2
    pub fn new_pp(p1: Point2<f32>, p2: Point2<f32>) -> Self {
        let diff = normalized(minus(p1, p2));
        Line {
            a: diff.y,
            b: -diff.x,
            c: -diff.y*p1.x + diff.x*p1.y,
        }
    }

    // the line with slope s that passes through point p
    pub fn new_sp(s: f32, p: Point2<f32>) -> Self {
        let niff = normalized(Point2::<f32>{x: 1.0, y:s});
        Line {
            a: niff.y,
            b: -niff.x,
            c: -niff.y*p.x + niff.x*p.y,
        }
    }

    pub fn slope(&self) -> f32 {
        -self.a/self.b
    }

    pub fn y(&self, x: f32) -> f32 {
        // y = -(ax + c)/b
        -(self.a*x + self.c)/self.b
    }

    pub fn x(&self, y: f32) -> f32 {
        // x = -(by + c)/a
        -(self.b*y + self.c)/self.a
    }

    pub fn intersection(&self, other: &Self) -> Point2<f32> {
        // `Ax + By + C = ax + by + c`
        // 
        // `y = (ax - Ax + c - C) / (B - b)
        // 
        // 'Ax + B[(ax - Ax + c - C) / (B - b)] + C = ax + b[(ax - Ax + c - C) / (B - b)] + c`
        // 
        // 'Ax + Bax/(B-b) - BAx/(B-b) - ax - bax/(B-b) + bAx/(B-b) =
        //     bc/(B-b) - bC/(B-b) + c - Bc/(B-b) + BC/(B-b) - C`
        // 
        // 'Ax(B-b) + Bax - BAx - ax(B-b) - bax + bAx = bc - bC + c(B-b) - Bc + BC - C(B-b)`
        // 
        // `x = [bc - bC + c(B-b) - Bc + BC - C(B-b)] / [A(B-b) + Ba - BA - a(B-b) - ba + bA]`
        // 
        // `x = [ - b(C-c) + c(B-b) + B(C-c) - C(B-b)] / [A(B-b) - B(A-a) - a(B-b) + b(A-a)]`
        // 
        // `x = [(B-b)(C-c) - (C-c)(B-b)] / [(A-a)(B-b) - (B-b)(A-a)]`
        // 
        // `x = 0 / 0`
        // 
        // `fuck`
        
        // From googling Cramer's Rule:
        let x = ( self.b*other.c - other.b*self.c) / (self.a*other.b - other.a*self.b);
        let y = (-self.a*other.c + other.a*self.c) / (self.a*other.b - other.a*self.b);
        Point2::<f32>{x: x, y: y}
    }
}
