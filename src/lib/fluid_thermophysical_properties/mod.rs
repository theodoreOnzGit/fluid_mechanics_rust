
use uom::si::f64::*;

/// This is a library containing all the properties which one can
/// choose for the ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions
pub mod property_library;
pub use property_library::*;

/// contains tests and examples to use the fluid thermophysical properties
pub mod tests_and_examples;


// ideally i'd want an easy way to make a selection of which fluid i want
// to use, perhaps via an enum or something,
// then the fluid properties are automatically loaded
// perhaps for convenience i can make a trait object of sorts
// the trait object will be a dependency 
// injected into another trait along with
// the temperature at the constructor
//
// This trait will then be "inherited" and available for immediate use
// I can call it a FluidPropertyAssociatedFunctions kind of trait
//
// Strictly speaking, you DON'T need to use
// 
// ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions
// trait to write code,
//
// but it may make it easier as you don't have to keep writing those
// thermophysical properties down

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

/// This trait makes it easier to set the fluid properties
///
/// The basic idea is that you just make an object that implements
/// the FluidProperties trait,
/// use that object as an argument to the function
/// and then get the desired fluid property.
///
/// You one can pick this out from a library or something
/// and basically you're all set
pub trait ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions
<'trait_lifetime>{

    /// fluid density based on temperature,
    /// it uses a static dispatch impl rather than &dyn because
    /// one fluid property should be used for a 
    /// fluid with constant composition and single phase
    ///
    /// it must have a solid implementation though, otherwise it won't work
    fn density(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> MassDensity {

        return fluid_properties.density(fluid_temp);

    }
    /// fluid  viscosity based on temperature
    fn viscosity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> DynamicViscosity{

        return fluid_properties.viscosity(fluid_temp);

    }

    /// fluid specific enthalpy based on temperature
    fn enthalpy(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> AvailableEnergy{

        return fluid_properties.enthalpy(fluid_temp);

    }

    /// fluid  specific_heat_capacity based on temperature
    fn specific_heat_capacity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> SpecificHeatCapacity{

        return fluid_properties.specific_heat_capacity(fluid_temp);

    }

    /// fluid thermal conductivity based on temperature
    fn thermal_conductivity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> ThermalConductivity{

        return fluid_properties.thermal_conductivity(fluid_temp);

    }

    /// fluid temperature based on enthalpy
    fn get_temperature_from_enthalpy(fluid_enthalpy: AvailableEnergy,
               fluid_properties: &dyn FluidProperties) 
        -> ThermodynamicTemperature{

        return fluid_properties.get_temperature_from_enthalpy(fluid_enthalpy);

    }

    /// Prandtl number calculated by
    /// mu cp / k
    fn prandtl_number(fluid_temp: ThermodynamicTemperature,
                      fluid_properties: &dyn FluidProperties) -> f64 {
        // mu * cp/k

        let prandtl = fluid_properties.viscosity(fluid_temp)
            *fluid_properties.specific_heat_capacity(fluid_temp)
            /fluid_properties.thermal_conductivity(fluid_temp);

        return prandtl.value;
    }

    /// get fluid temperature 
    /// this is a get function which forces the user
    /// to remember to have a fluid temperature property

    fn get_fluid_temp(&self) -> ThermodynamicTemperature;

    /// set fluid temperature
    /// this is a get function which forces the user
    /// to remember to have a fluid temperature property

    fn set_fluid_temp(&mut self, fluid_temp: ThermodynamicTemperature);

    /// a function to return a set FluidProperties Object
    fn get_fluid_properties(&self) -> &'trait_lifetime dyn FluidProperties;

    /// a function to set a FluidProperties Object
    fn set_fluid_properties(&mut self,
                            fluid_properties: &'trait_lifetime dyn FluidProperties);

}

/// A trait (or interface) for getting fluid properties from
/// temperature or enthalpy
///
/// here is a trait (or interface for C# or java people), 
/// however, traits only deal with methods, not properties.
/// note that traits are also types
///
/// This trait (or interface for methods) ensures
/// that density, dynamic viscosity,
/// fluid enthalpy,
/// specific heat capacity,
/// thermal conductivity 
///
/// can be evaluated from temperature
///
/// Also, we can get temperature given fluid enthalpy
/// this is because we may want in future to quickly
/// obtain fluid temperature after energy balance
///
///
/// Now the enthalpy, specific heat capacity and thermal
/// conductivity may not be used in fluid mechanics, but they are useful
/// for heat transfer
///
/// this is updated from the FluidProperties trait in therminol component,
/// because i want it to be made into objects. Therefore, it takes
/// an immutable reference to self
///
pub trait FluidProperties {
    /// fluid density based on temperature,
    fn density(&self,
               fluid_temp: ThermodynamicTemperature) -> MassDensity;

    /// fluid dynamic viscosity based on temperature,
    fn viscosity(&self,
                 fluid_temp: ThermodynamicTemperature) -> DynamicViscosity;

    /// fluid enthalpy  based on temperature,
    fn enthalpy(&self,
                fluid_temp: ThermodynamicTemperature) -> AvailableEnergy;

    /// fluid specific heat capacity  based on temperature,
    fn specific_heat_capacity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity;

    /// fluid thermal conductivity based on temperature,
    fn thermal_conductivity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity;

    /// fluid temperature based on fluid enthalpy
    fn get_temperature_from_enthalpy(
        &self,
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature;
}


