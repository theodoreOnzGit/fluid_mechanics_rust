#[warn(missing_docs)]

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
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

/// function to obtain dowtherm A density
/// given a temperature
#[allow(non_snake_case)]
pub fn getDowthermADensity(
    fluidTemp: ThermodynamicTemperature) -> MassDensity {

    // first we check if fluid temp is between 20-180C (range of validity)
    // panic otherwise
    rangeCheck(fluidTemp);

    //then convert the fluidTemp object into a f64
    // and plug it into the correlation
    let densityValueKgPerM3 = 1078.0 - 0.85*fluidTemp
       .get::<degree_celsius>();

    return MassDensity::new::<kilogram_per_cubic_meter>(densityValueKgPerM3);
}

/// function to obtain dowtherm A viscosity
/// given a temperature
#[allow(non_snake_case)]
pub fn  getDowthermAViscosity(
    fluidTemp: ThermodynamicTemperature) -> DynamicViscosity{

    rangeCheck(fluidTemp);
    let temperatureDegreesCValue = fluidTemp.get::<degree_celsius>();
    let viscosityValuePascalSecond = 0.130/
        temperatureDegreesCValue.powf(1.072);

    return DynamicViscosity::new::<pascal_second>(viscosityValuePascalSecond);
                                
}

/// function to obtain dowtherm A specific heat capacity
/// given a temperature
#[allow(non_snake_case)]
pub fn getDowthermAConstantPressureSpecificHeatCapacity(
    fluidTemp: ThermodynamicTemperature) -> SpecificHeatCapacity{

    rangeCheck(fluidTemp);
    // note, specific entropy and heat capcity are the same unit...
    //
    let cp_value_joule_per_kg = 1518.0 + 2.82*fluidTemp.get::<degree_celsius>();

    return SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg);
}

/// function to obtain dowtherm A thermal conductivity
/// given a temperature
#[allow(non_snake_case)]
pub fn getDowthermAThermalConductivity(
    fluidTemp: ThermodynamicTemperature) -> ThermalConductivity {


    rangeCheck(fluidTemp);
    let thermalConductivityValue = 0.142 - 0.00016* fluidTemp
        .get::<degree_celsius>();

    return ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermalConductivityValue);
}

/// function to obtain dowtherm A enthalpy
/// given a temperature
///
/// 
/// This is done via analytically integrating 
/// the function for specific heat capacity of 
/// dowtherm A
///
/// However,
/// the thing is that with enthalpy
/// we need a reference value
/// i take the reference value to be 0 J/kg enthalpy at 20C
/// integrating heat capacity with respect to T, we get
///
/// cp = 1518 + 2.82*T
///
/// H = 1518*T + 2.82/2.0*T^2 + C
/// at T = 20C, 
/// H = 30924 + C
/// H = 0
/// C = -30924 (i used libre office to calculate this)
///
/// Example use:
/// ```rust
///
/// use uom::si::f64::*;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use fluid_mechanics_rust::therminol_component::
/// dowtherm_a_properties::getDowthermAEnthalpy;
///
/// let temp1 = ThermodynamicTemperature::new::<kelvin>(303_f64);
///
/// let specific_enthalpy_1 = 
/// getDowthermAEnthalpy(temp1);
///
///
/// let expected_enthalpy: f64 = 
/// 1518_f64*30_f64 + 2.82/2.0*30_f64.powf(2_f64) - 30924_f64;
///
/// // the expected value is about 15885 J/kg
///
/// extern crate approx;
/// approx::assert_relative_eq!(expected_enthalpy, specific_enthalpy_1.value, 
/// max_relative=0.02);
/// ```
#[allow(non_snake_case)]
pub fn getDowthermAEnthalpy(
    fluidTemp: ThermodynamicTemperature) -> AvailableEnergy{

    rangeCheck(fluidTemp);
    // note, specific entropy and heat capcity are the same unit...
    //
    // H = 1518*T + 2.82/2.0*T^2 - 30924
    let tempCValue = fluidTemp.get::<degree_celsius>();
    let enthalpy_value_joule_per_kg 
        = 1518.0 * tempCValue 
        + 2.82/2.0 * tempCValue.powf(2.0) -
        30924.0;

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    return AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg);
}

/// function to obtain dowtherm A temperature 
/// given a enthalpy
///
/// 
/// This is done via analytically integrating 
/// the function for specific heat capacity of 
/// dowtherm A
///
/// However,
/// the thing is that with enthalpy
/// we need a reference value
/// i take the reference value to be 0 J/kg enthalpy at 20C
/// integrating heat capacity with respect to T, we get
///
/// cp = 1518 + 2.82*T
///
/// H = 1518*T + 2.82/2.0*T^2 + C
/// at T = 20C, 
/// H = 30924 + C
/// H = 0
/// C = -30924 (i used libre office to calculate this)
///
/// Once i have this correlation, i will use
/// an iterative root finding method to find the temperature
///
/// As of Oct 2022, it is bisection
///
/// Example: 
///
/// ```rust
/// use uom::si::f64::*;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::available_energy::joule_per_kilogram;
/// use fluid_mechanics_rust::therminol_component::
/// dowtherm_a_properties::get_temperature_from_enthalpy;
///
///
/// let specific_enthalpy_1 = AvailableEnergy::new::
/// <joule_per_kilogram>(15885.0);
///
/// let temp_expected = ThermodynamicTemperature::new::
/// <kelvin>(303_f64);
/// 
/// let temp_acutal = get_temperature_from_enthalpy(
/// specific_enthalpy_1);
///
///
/// extern crate approx;
/// approx::assert_relative_eq!(temp_expected.value, 
/// temp_acutal.value, 
/// max_relative=0.01);
///
///
/// ```
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature {

    if fluid_enthalpy.value < 0_f64 {
        panic!("dowtherm A : get_temperature_from_enthalpy \n
               enthalpy < 0.0 , out of correlation range");
    }

    // first let's convert enthalpy to a double (f64)
    let enthalpy_value_joule_per_kg = 
        fluid_enthalpy.get::<joule_per_kilogram>();

    // second let's define a function 
    // or actually a closure or anonymous function that
    // is aware of the variables declared
    // enthalpy value = 1518*T +2.82/2.0 T^2 - 30924
    // LHS is actual enthalpy value

    let enthalpy_root = |temp_degrees_c_value : AD| -> AD {
        let lhs_value = enthalpy_value_joule_per_kg;
        // convert AD type into double
        let temp_degrees_c_value_double = temp_degrees_c_value.x();

        let fluid_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                temp_degrees_c_value_double);
        let rhs = getDowthermAEnthalpy(fluid_temperature);
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return AD0(lhs_value-rhs_value);
    };
    
    // now solve using bisection
    
    let fluid_temperature_degrees_cresult 
        = bisection(enthalpy_root,
                    (20.0,180.0),
                    100,
                    1e-8);

    let fluid_temperature_degrees_c = fluid_temperature_degrees_cresult.unwrap();

    return ThermodynamicTemperature::
        new::<degree_celsius>(fluid_temperature_degrees_c);

}


/// function checks if a fluid temperature falls in a range (20-180C)
///
/// If it falls outside this range, it will panic
/// or throw an error, and the program will not run
#[allow(non_snake_case)]
pub fn rangeCheck(fluidTemp: ThermodynamicTemperature) -> bool{

    // first i convert the fluidTemp object into a degree 
    // celsius
    let tempvalueCelsius = 
        fluidTemp.get::<degree_celsius>();

    if tempvalueCelsius < 20.0 {
        let errorMsg = "Your fluid temperature \n";
        let errorMsg1 = "is too low :";
        let errorMsg3 = "C \n";
        let errorMsg4 = "\n the minimum is 20C";


        panic!("{}{}{:?}{}{}",
               errorMsg,
               errorMsg1,
               fluidTemp,
               errorMsg3,
               errorMsg4);
    }


    if tempvalueCelsius > 180.0 {
        let errorMsg = "Your fluid temperature \n";
        let errorMsg1 = "is too high :";
        let errorMsg3 = "C \n";
        let errorMsg4 = "\n the max is 180C";

        panic!("{}{}{:?}{}{}",
               errorMsg,
               errorMsg1,
               fluidTemp,
               errorMsg3,
               errorMsg4);
    }

    return true;

}
