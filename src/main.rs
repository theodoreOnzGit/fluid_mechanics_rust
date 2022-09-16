extern crate uom;
use fluid_mechanics_rust;
mod manual_tests;

use crate::manual_tests::*;


fn main() {
    println!("Hello, world!");
    test_friction_factor();
    test_dimensionless_number();
    test_standard_pipe_calc();
    test_custom_fldk_component();
    test_temperature_conversion();
    test_therminol_pipe();
    test_therminol_fldk_custom_component();
}


fn test_friction_factor(){
    let darcy_friction_factor = 
        fluid_mechanics_rust::darcy(1800.0,0.0015);

    println!("{}", darcy_friction_factor);
    
    let fldk = 
        fluid_mechanics_rust::fldk(
            15000.0,0.00014,10.0,5.0);

    println!("{}", fldk);

}

