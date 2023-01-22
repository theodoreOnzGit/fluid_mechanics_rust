// This library was developed for use in my PhD thesis under supervision 
// of Professor Per F. Peterson. It is part of a thermal hydraulics
// library in Rust that is released under the GNU General Public License
// v 3.0. This is partly due to the fact that some of the libraries 
// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
// Public License v3.0.
//
// As such, the entire library is released under GNU GPL v3.0. It is a strong 
// copyleft license which means you cannot use it in proprietary software.
//
//
// License
//    This file is part of fluid_mechanics_rust, a partial library of the
//    thermal hydraulics library written in rust meant to help with the
//    fluid mechanics aspects of the calculations
//     
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    fluid_mechanics_rust is free software; you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    fluid_mechanics_rust is distributed in the hope that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    This library is part of a thermal hydraulics library in rust
//    and contains some code copied from GeN-Foam, and OpenFOAM derivative.
//    This offering is not approved or endorsed by the OpenFOAM Foundation nor
//    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
//    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
//    Nor is it endorsed by the authors and owners of GeN-Foam.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// © All rights reserved. Theodore Kay Chen Ong,
// Singapore Nuclear Research and Safety Initiative,
// Per F. Peterson,
// University of California, Berkeley Thermal Hydraulics Laboratory
//
// Main author of the code: Theodore Kay Chen Ong, supervised by
// Professor Per F. Peterson
extern crate uom;
use fluid_mechanics_rust;

// adding units from uom
use uom::si::mass_rate::kilogram_per_second;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::length::{meter,millimeter,foot,inch};
use uom::si::pressure::pascal;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::area::square_meter;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::thermodynamic_temperature::degree_celsius;

use uom::si::f64::*;
use uom::typenum::P2;

// for timed tests
use std::time::SystemTime;


/// here are tests (manual) that i didn't use assert equal or anything

pub fn factory_test(){

    // the factory test ensures that we do all the hard coding behind the scenes
    //
    let start = SystemTime::now();
    use fluid_mechanics_rust::therminol_component::factory;

    let flowmeter_40 = factory::Flowmeter40::get();

    // now let's have a temperature of 21C and mass flow of 0.15 kg/s
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(21.0);
    let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);

    // now let's use the calc pressure change object
    use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;

    let pressure_change = CalcPressureChange::from_mass_rate(
        &flowmeter_40,
        mass_flow_expected,
        fluid_temp);

    println!("calculated pressure_change: {:?} \n", pressure_change);

    // if you don't want to type all the pressure change stuff out,
    // you can also use this syntax
    let test_mass_flow = flowmeter_40.
        to_mass_rate(pressure_change,
                     fluid_temp);

    println!("expected_mass_rate: {:?}\n", mass_flow_expected);
    println!("actual_mass_rate: {:?} \n", test_mass_flow);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("therminol/dowtherm A factory calculations took {:?}", duration);
}

pub fn test_therminol_fldk_custom_component(){

    let start = SystemTime::now();

    // let's import everything necessary:

    use fluid_mechanics_rust::therminol_component::custom_therminol_component::
        DowthermACustomComponent;


    // now lets define the functions
    //
    fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value = 
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }
    // now we have constructed our fldk functions, let's construct the 
    // component
    use crate::fluid_mechanics_rust::therminol_component::
        StandardCustomComponentProperties;

    let flowmeter_40_14a: DowthermACustomComponent 
        = StandardCustomComponentProperties::new(
        "flowmeter_40_14a".to_string(),
        2.79e-2, // component diameter in meters
        6.11e-4, // component area in square meters
        0.36, // component length in meters
        0.015, // estimated component wall roughness (doesn't matter here,
               // but i need to fill in
        90.0, //incline angle in degrees
        &custom_darcy,
        &custom_k);

    // now let's have a temperature of 21C and mass flow of 0.15 kg/s
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(21.0);
    let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);

    // now let's use the calc pressure change object
    use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;

    let pressure_change = CalcPressureChange::from_mass_rate(
        &flowmeter_40_14a,
        mass_flow_expected,
        fluid_temp);

    println!("calculated pressure_change: {:?} \n", pressure_change);

    // if you don't want to type all the pressure change stuff out,
    // you can also use this syntax
    let test_mass_flow = flowmeter_40_14a.
        to_mass_rate(pressure_change,
                     fluid_temp);

    println!("expected_mass_rate: {:?}\n", mass_flow_expected);
    println!("actual_mass_rate: {:?} \n", test_mass_flow);


    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("\n custom fldk component for dowtherm A / therinol took {:?}", duration);
}

pub fn test_therminol_pipe(){
    let start = SystemTime::now();
    use fluid_mechanics_rust::therminol_component::therminol_pipe::
        DowthermAPipe;

    use crate::fluid_mechanics_rust::therminol_component::
        StandardPipeProperties;


    // it's kind of tedious to construct a new pipe
    // i might consider making a component factory class to instantiate 
    // everything


    let static_mixer_pipe_6a: DowthermAPipe = 
        StandardPipeProperties::new(
            "static_mixer_pipe_6a".to_string(),
            2.79e-02, // pipe diameter 2.79e-02m
            0.1526, // pipe length 0.1526m
            0.015, // wall roughness for stainless steel 0.015mm
            51.526384, // incline angle 51.526384 degrees
            5.05); // form loss k = 5.05 dimensionless

    // now let's have a temperature of 21C and mass flow of 0.15 kg/s
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(21.0);
    let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);

    // now let's use the calc pressure change object
    use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // this is an alternative syntax available to calcualte pressure
    // change, but it may seem confusing to the user
    let pressure_change = static_mixer_pipe_6a.
        from_mass_rate(mass_flow_expected,
                       fluid_temp);

    println!("calculated pressure_change: {:?} \n", pressure_change);

    let test_mass_flow = static_mixer_pipe_6a.
        to_mass_rate(pressure_change,
                     fluid_temp);

    println!("expected_mass_rate: {:?}\n", mass_flow_expected);
    println!("actual_mass_rate: {:?} \n", test_mass_flow);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("therminol/dowtherm A pipe calculations took {:?}", duration);


}


pub fn test_dimensionless_number(){
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

    fn custom_k(mut reynolds_number: f64) -> f64 {

        let mut reverse_flow = false;
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }
        let fldk =  400.0 + 52000.0/reynolds_number;

        if reverse_flow == true {
            return -fldk;
        }
        return fldk;
    }

    fn custom_f(_reynolds_number: f64,
                     _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    // testing custom K pipe
    // now using some object oriented programming
    // structs with implementations behave a bit like static classes hah
    let custom_fldk = 
        fluid_mechanics_rust::CustomComponent::fldk(
            &custom_f,
            -5000.0,
            0.00014,
            10.0,
            &custom_k);

    println!("{}", custom_fldk);

    // now testing for bejan number for custom k 
    // and we are testing for reverse flow
    let expected_bejan_custom_fldk = 0.5*custom_fldk * reynolds_number.powf(2.0);
    println!("expected Bejan custom k pipe: {}", expected_bejan_custom_fldk);

    let actual_bejan_custom_k = fluid_mechanics_rust::CustomComponent::
        get_bejan_custom_fldk(&custom_f,
                              -5000.0,
                              0.00014,
                              10.0,
                              &custom_k);

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

    // here is some testing for bejan number

    let fluid_pressure = Pressure::new::<pascal>(500.0);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let bejan_number = fluid_mechanics_rust::CalcBejan::from_pressure(
        fluid_pressure,
        pipe_diameter,
        fluid_density,
        fluid_viscosity);


    println!("Bejan number: {} \n", bejan_number);

    let test_fluid_pressure = fluid_mechanics_rust::CalcBejan::to_pressure(
        bejan_number,
        pipe_diameter,
        fluid_density,
        fluid_viscosity);

    println!("reference pressure : {:?} ", fluid_pressure);
    println!("test fluid pressure : {:?} \n", test_fluid_pressure);
}

pub fn test_standard_pipe_calc() {

    let start = SystemTime::now();

    let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.015);
    let cross_sectional_area= Area::new::<square_meter>(4e-5);
    let hydraulic_diameter= Length::new::<inch>(3.0);
    let fluid_viscosity= DynamicViscosity::new::<pascal_second>(0.001);
    let fluid_density= MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    let pipe_length= Length::new::<foot>(6.0);
    let absolute_roughness= Length::new::<millimeter>(0.001);
    let form_loss_k= 5.0;

    // first import crate for CalcPressureLoss functions
    use crate::fluid_mechanics_rust::
        fluid_component_calculation::
        standard_pipe_calc::CalcPressureLoss;

    let pressure_loss = CalcPressureLoss::from_mass_rate(
            fluid_mass_flowrate,
            cross_sectional_area,
            hydraulic_diameter,
            fluid_viscosity,
            fluid_density,
            pipe_length,
            absolute_roughness,
            form_loss_k);

    println!("reference pressure loss : {:?} (Pascals) ", pressure_loss);


    let test_mass_rate = CalcPressureLoss::to_mass_rate(
        pressure_loss,
        cross_sectional_area,
        hydraulic_diameter,
        fluid_viscosity,
        fluid_density,
        pipe_length,
        absolute_roughness,
        form_loss_k);

    println!("reference mass flowrate : {:?}  ", fluid_mass_flowrate);
    println!("test mass flowrate : {:?}  ", test_mass_rate);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    println!("bisection numerical solution for pipe took {:?}", duration);

}


pub fn test_custom_fldk_component(){

    fn custom_k(mut reynolds_number: f64) -> f64 {

        let mut reverse_flow = false;
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }
        let fldk =  400.0 + 52000.0/reynolds_number;

        if reverse_flow == true {
            return -fldk;
        }
        return fldk;
    }

    fn custom_f(_reynolds_number: f64,
                     _roughness_ratio: f64) -> f64 {
        return 0.0;
    }
    let start = SystemTime::now();

    let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let cross_sectional_area= Area::new::<square_meter>(6.11e-4);
    let hydraulic_diameter= Length::new::<meter>(2.79e-2);
    let fluid_viscosity= DynamicViscosity::new::<pascal_second>(0.0044);
    let fluid_density= MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    let pipe_length= Length::new::<foot>(6.0);
    let absolute_roughness= Length::new::<millimeter>(0.001);

    // first import crate for CalcPressureLoss functions
    use crate::fluid_mechanics_rust::
        fluid_component_calculation::
        custom_component_calc::CalcPressureLoss;

    let reynolds_number = fluid_mass_flowrate/
        cross_sectional_area*
        hydraulic_diameter/
        fluid_viscosity;

    println!("\n reynolds_number = {:?}", reynolds_number);

    let pressure_loss = 
        CalcPressureLoss::from_mass_rate(fluid_mass_flowrate,
                                         cross_sectional_area,
                                         hydraulic_diameter,
                                         fluid_viscosity,
                                         fluid_density,
                                         pipe_length,
                                         absolute_roughness,
                                         &custom_f,
                                         &custom_k);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();



    println!("pressure loss calculated as {:?}", pressure_loss);

    println!("custom component calc pressure loss took {:?}", duration);
}

pub fn test_temperature_conversion(){

    let kelvin_temp = ThermodynamicTemperature::new::<kelvin>(293.0);
    let celsius_temp = kelvin_temp.get::<degree_celsius>();

    println!("\n {:?} = {:?} C", kelvin_temp, celsius_temp);

}




