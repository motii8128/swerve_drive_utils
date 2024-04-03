mod wheel_unit;
pub mod driver;

pub struct Position
{
    x_:f64,
    y_:f64,
    z_:f64
}

impl Position {
    pub fn new(x:f64, y:f64, z:f64)->Position
    {
        Position{x_:x, y_:y, z_:z}
    }
}