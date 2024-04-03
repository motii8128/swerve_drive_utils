use crate::wheel_unit;
use wheel_unit::{WheelUnit, NONE};

use std::net::UdpSocket;
pub struct SwerveDriverUDP
{
    pub front_left:WheelUnit,
    pub front_right:WheelUnit,
    pub rear_left:WheelUnit,
    pub rear_right:WheelUnit,
    sock_:UdpSocket
}

impl SwerveDriverUDP {
    pub fn new(local_addr:&str, wheel_diameter:f64)->SwerveDriverUDP
    {
        let fl = WheelUnit::new(wheel_diameter,NONE.to_string());
        let fr = WheelUnit::new(wheel_diameter,NONE.to_string());
        let rl = WheelUnit::new(wheel_diameter,NONE.to_string());
        let rr = WheelUnit::new(wheel_diameter,NONE.to_string());

        let sock = UdpSocket::bind(local_addr).unwrap();

        SwerveDriverUDP{front_left:fl, front_right:fr, rear_left:rl, rear_right:rr, sock_:sock}
    }

    
}