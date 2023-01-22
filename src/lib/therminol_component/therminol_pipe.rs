use crate::fluid_component_calculation::standard_pipe_calc;
use crate::therminol_component::*;
use dowtherm_a_properties;

use uom::si::length::{meter,millimeter};
use uom::si::pressure::pascal;
use uom::si::angle::degree;
use uom::si::acceleration::meter_per_second_squared;
use uom::typenum::P2;

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
/// This structure contains methods to calculate 
/// pressure losses for a dowtherm A pipe
///
/// The user only needs to follow this code example
/// and use the constructor (or the "new" method)
/// in order to create a pipe to user specifications:
/// 
/// ```rust
/// // first you need to use the DowthermAPipe
/// // struct and the 
/// // StandardPipeProperties struct
///
/// use fluid_mechanics_rust::therminol_component::therminol_pipe::
///     DowthermAPipe;
///
/// use crate::fluid_mechanics_rust::therminol_component::
///     StandardPipeProperties;
///
/// // to make a new pipe object, in this case
/// // the static_mixer_pipe_6a object,
/// // you need to make sure it is of the type
/// // DowthermAPipe, using the type annotation
///
/// // after that, use the new method within
/// // StandardPipeProperties
/// // to construct it according to the template
/// // make sure the name is of the type string
/// // using the to_string() method
///
/// let static_mixer_pipe_6a: DowthermAPipe = 
///     StandardPipeProperties::new(
///         "static_mixer_pipe_6a".to_string(),
///         2.79e-02, // pipe diameter 2.79e-02m
///         0.1526, // pipe length 0.1526m
///         0.015, // wall roughness for stainless steel 0.015mm
///         51.526384, // incline angle 51.526384 degrees
///         5.05); // form loss k = 5.05 dimensionless
///
/// 
/// // now let's have a temperature of 21C and mass flow of 0.15 kg/s
/// // that's if you want to calculate pressure change
/// // to and from mass flowrate:
///
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::thermodynamic_temperature::degree_celsius;
/// 
/// use uom::si::f64::*;
/// use uom::typenum::P2;
/// use uom::si::mass_rate::kilogram_per_second;
///
/// let fluid_temp = ThermodynamicTemperature::new::<
///     degree_celsius>(21.0);
/// let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);
///
/// // now let's use the calc pressure change object
/// use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;
///
/// // (1) so if you want to calculate pressure change:
///
/// let pressure_change = CalcPressureChange::from_mass_rate(
///                                     &static_mixer_pipe_6a,
///                                     mass_flow_expected,
///                                     fluid_temp);
///
/// println!("calculated pressure_change: {:?} \n", pressure_change);
///
/// use uom::si::pressure::pascal;
/// // (2) if you want to calculate mass flowrate
///
/// let pressure_change = Pressure::new::<pascal>(15900_f64);
///
/// let test_mass_flow = CalcPressureChange::to_mass_rate(
///                                     &static_mixer_pipe_6a,
///                                     pressure_change,
///                                     fluid_temp);
///
/// println!("actual_mass_rate: {:?} \n", test_mass_flow);
///
///
/// ```
pub struct DowthermAPipe {
// now i will have dowtherm A pipe
// this is simliar in composition to therminol VP 1 so 
// i class them as the same

    /// the dowtherm_pipe_properties object
    /// or struct instance
    /// will have pipe length, hydraulic diameter,
    /// angle of inclilne, 
    /// form losses and
    /// roughness
    ///
    /// all are user specified through the constructor
    /// 
    pub dowtherm_pipe_properties: PipeProperties,
}
/// dowtherm A pipe has methods to obtain
/// thermophysical properties using the Dowtherm a correlations
///
impl FluidProperties for DowthermAPipe {
    fn density(fluid_temp: ThermodynamicTemperature) -> MassDensity {
        return dowtherm_a_properties::getDowthermADensity(fluid_temp);
    }

    fn viscosity(
        fluid_temp: ThermodynamicTemperature) -> DynamicViscosity{
        return dowtherm_a_properties::getDowthermAViscosity(fluid_temp);
    }

    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{
        return dowtherm_a_properties::getDowthermAEnthalpy(fluid_temp);
    }

    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity{
        return dowtherm_a_properties::
            getDowthermAConstantPressureSpecificHeatCapacity(
            fluid_temp);
    }

    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity{
        return dowtherm_a_properties::
            getDowthermAThermalConductivity(fluid_temp);
    }

    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature{
        return dowtherm_a_properties::
            get_temperature_from_enthalpy(fluid_enthalpy);
    }

}

impl StandardPipeProperties for DowthermAPipe {
    // constructor
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           form_loss_k: f64) -> Self{

        let calculated_hydraulic_diameter = Length::new::<meter>(
            hydraulic_diameter_meters);
        let calculated_component_length = Length::new::<meter>(
            component_length_meters);
        let calculated_absolute_roughness = Length::new::<millimeter>(
            absolute_roughness_millimeters);
        let calculated_incline_angle = Angle::new::<degree>(
            incline_angle_degrees);
        let calculated_internal_pressure = Pressure::new::<pascal>(
            0.0);

        let custom_pipe_properties = PipeProperties {
            _name: name,
            hydraulic_diameter: calculated_hydraulic_diameter,
            component_length: calculated_component_length,
            absolute_roughness: calculated_absolute_roughness,
            incline_angle: calculated_incline_angle,
            form_loss_k: form_loss_k,
            internal_pressure: calculated_internal_pressure,
        };

        return Self { dowtherm_pipe_properties : custom_pipe_properties };
    }

    fn get_cross_sectional_area(&self) -> Area {
        let pipe_diameter = self.dowtherm_pipe_properties.hydraulic_diameter;
        let pipe_xs_area = 
            pipe_diameter.powi(P2::new())*
            std::f64::consts::PI/
            4.0;
        

        return pipe_xs_area;
    }


    fn get_internal_pressure_term(&self) -> Pressure {
        return self.dowtherm_pipe_properties.internal_pressure;
    }

    fn set_internal_pressure_term(&mut self, pressure_pascals: f64) {
        self.dowtherm_pipe_properties.internal_pressure =
            Pressure::new::<pascal>(pressure_pascals);
    }

    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure {

        let pipe_length = self.dowtherm_pipe_properties.component_length;
        let incline_angle = self.dowtherm_pipe_properties.incline_angle;
        let fluid_density = DowthermAPipe::density(fluid_temp);

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }
}

impl CalcPressureChange for DowthermAPipe {

    fn from_mass_rate(&self, fluid_mass_flowrate: MassRate,
                      fluid_temp: ThermodynamicTemperature) -> Pressure {

        // first let's get all the relevant properties...
        let hydraulic_diameter = self.dowtherm_pipe_properties.hydraulic_diameter;
        let pipe_length = self.dowtherm_pipe_properties.component_length;
        let absolute_roughness = self.dowtherm_pipe_properties.absolute_roughness;
        let xs_area = self.get_cross_sectional_area();

        let fluid_viscosity = DowthermAPipe::viscosity(fluid_temp);
        let fluid_density = DowthermAPipe::density(fluid_temp);

        let form_loss_k = self.dowtherm_pipe_properties.form_loss_k;

        // second let's get pressure loss from mass rate
        // by using this function or method, we assume that
        // the pipe behaves symmetrically in reverse flow
        let pressure_loss = standard_pipe_calc::CalcPressureLoss::
            from_mass_rate(
                fluid_mass_flowrate,
                xs_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                form_loss_k);
        // now to calculate pressure change
        // we note this equation
        //
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        //
        // for hydrostatic pressure gain
        // g is earth gravity at 9.81
        // delta H is positive upwards

        let hydrostatic_pressure_increase: Pressure =
            self.get_hydrostatic_pressure_change(
                fluid_temp);
        // last but not least we need our source pressure

        let source_pressure: Pressure = self.dowtherm_pipe_properties.
            internal_pressure;

        // now we can calculate pressure change

        let pressure_change = 
            -pressure_loss +
            hydrostatic_pressure_increase +
            source_pressure;

        return pressure_change;
    }

    fn to_mass_rate(&self, pressure_change: Pressure,
                    fluid_temp: ThermodynamicTemperature) -> MassRate {
        // first let's get all the relevant properties...
        let hydraulic_diameter = self.dowtherm_pipe_properties.hydraulic_diameter;
        let pipe_length = self.dowtherm_pipe_properties.component_length;
        let absolute_roughness = self.dowtherm_pipe_properties.absolute_roughness;
        let xs_area = self.get_cross_sectional_area();

        let fluid_viscosity = DowthermAPipe::viscosity(fluid_temp);
        let fluid_density = DowthermAPipe::density(fluid_temp);

        let form_loss_k = self.dowtherm_pipe_properties.form_loss_k;

        // now we need to calculate a pressure loss term
        // we use:
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        // so we just add pressure loss to both sides and subtract pressure
        // change to both sides
        // pressure loss  = - pressure change + hydrostatic pressure +
        // source pressure

        // for hydrostatic pressure gain
        // g is earth gravity at 9.81
        // delta H is positive upwards
        let hydrostatic_pressure_increase: Pressure =
            self.get_hydrostatic_pressure_change(
                fluid_temp);
        // last but not least we need our source pressure

        let source_pressure: Pressure = self.dowtherm_pipe_properties.
            internal_pressure;

        // now calculate pressure loss
        let pressure_loss = 
            -pressure_change +
            hydrostatic_pressure_increase +
            source_pressure;

        let mass_rate = standard_pipe_calc::CalcPressureLoss::
            to_mass_rate(
                pressure_loss,
                xs_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                form_loss_k);

        return mass_rate;
    }
}



