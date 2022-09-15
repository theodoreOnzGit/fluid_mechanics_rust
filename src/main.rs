extern crate uom;
use fluid_mechanics_rust;

// adding units from uom
use uom::si::mass_rate::kilogram_per_second;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::length::{meter,centimeter,foot,inch};
use uom::si::area::square_meter;

use uom::si::f64::*;
use uom::typenum::P2;


fn main() {
    println!("Hello, world!");
    hello2();
    test_friction_factor();
}

fn hello2(){
    println!("hello world!2");
}

fn test_friction_factor(){
    let darcy_friction_factor = 
        fluid_mechanics_rust::darcy(1800.0,0.0015);

    println!("{}", darcy_friction_factor);
    
    let fldk = 
        fluid_mechanics_rust::fldk(
            15000.0,0.00014,10.0,5.0);

    println!("{}", fldk);

    let bejan_d = 
        fluid_mechanics_rust::get_bejan_d(
            0.00000000000001,0.00014,10.0,5.0);

    // i can supply a Re of -5000 to the bejan number
    println!("{}", bejan_d);
    let bejan_d = 
        fluid_mechanics_rust::get_bejan_d(
            -5000.0,0.00014,10.0,5.0);

    println!("{}", bejan_d);

    // and i use the resulting bejan number to see
    // if i can get back the same Re
    //
    // reynolds number from Be
    let reynolds_number = 
        fluid_mechanics_rust::get_reynolds_number(
            bejan_d,0.00014,10.0,5.0);

    println!("{}", reynolds_number);

    fn custom_k_ctah(reynolds_number: f64) -> f64 {
        return 400.0 + 52000.0/reynolds_number;
    }

    fn custom_f_ctah(_reynolds_number: f64,
                     _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    // testing custom K pipe
    // now using some object oriented programming
    // structs with implementations behave a bit like static classes hah
    let custom_fldk = 
        fluid_mechanics_rust::CustomComponent::fldk(
            &custom_f_ctah,
            5000.0,
            0.00014,
            10.0,
            &custom_k_ctah);

    println!("{}", custom_fldk);

    // now testing for bejan number for custom k 
    //
    let expected_bejan_custom_fldk = -0.5*custom_fldk * reynolds_number.powf(2.0);
    println!("expected Bejan custom k pipe: {}", expected_bejan_custom_fldk);

    let actual_bejan_custom_k = fluid_mechanics_rust::CustomComponent::
        get_bejan_custom_fldk(&custom_f_ctah,
                              5000.0,
                              0.00014,
                              10.0,
                              &custom_k_ctah);

    println!("actual_bejan_custom_k: {} \n", actual_bejan_custom_k);
    // manual testing seems to work ok!
    //
    //
    let fluid_massflowrate = MassRate::new::<kilogram_per_second>(0.05);
    let pipe_diameter = Length::new::<meter>(2.79e-2);
    let pipe_xs_area = pipe_diameter.powi(P2::new())*std::f64::consts::PI/4.0;
    let fluid_viscosity = DynamicViscosity::new::<pascal_second>(0.001);

    let reynolds_number = fluid_mechanics_rust::CalcReynolds::from_mass_rate(
        fluid_massflowrate,
        pipe_xs_area,
        pipe_diameter,
        fluid_viscosity);

    println!("Reynolds number: {} \n", reynolds_number);

    let test_fluid_mass_flowrate = fluid_mechanics_rust::CalcReynolds::to_mass_rate(
        pipe_xs_area,
        reynolds_number,
        pipe_diameter,
        fluid_viscosity);

    println!("mass flowrate: {:?} \n", test_fluid_mass_flowrate);



}




