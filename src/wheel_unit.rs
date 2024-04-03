pub const NONE:&str = "NNNN";

pub struct WheelUnit
{
    pub yaw:f64,
    pub diam:f64,
    pub addr:String
}

impl WheelUnit {
    pub fn new(diameter:f64, mcu_addr:String)->WheelUnit
    {
        WheelUnit{yaw:0.0, diam:diameter, addr:mcu_addr}
    }
}