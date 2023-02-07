/*
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
*/

use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (idx, elem)in vec.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", idx, elem)?;
        }

        write!(f, "]")
    }
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let cmpx = Complex{
        real: 3.3,
        imag: 7.2
    };

    println!("{}", cmpx);
    println!("{:?}", cmpx);

    let mut vec = Vec::new();
    for i in 0..=5 {
        vec.push(i);
    }

    let list = List(vec);

    println!("{}", list);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}