
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
#[cfg(test)]
pub mod fluid_component_collection_test_and_examples {

    use std::f64::consts::PI;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::custom_component_calc::{FluidCustomComponentCalcPressureChange, FluidCustomComponentCalcPressureLoss};
    use crate::fluid_component_calculation::standard_pipe_calc
        ::{FluidPipeCalcPressureLoss, FluidPipeCalcPressureChange};
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal};
    use uom::si::angle::degree;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::fluid_thermophysical_properties::*;


    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component collections
    ///
    ///
    /// [TBD]
    /// First i make a typical fluid component, a set of air pipes
    /// perhaps 10 therminol pipes and i want to put them in series
    ///
    #[test]
    pub fn example_1_therminol_pipe () {

        

        // we will implement a few properties here for our therminol pipe
        // for clarity we will list them in a
        // supertrait
        // This makes it easy to see what traits are being implemented here

        pub trait TherminolPipeTraits<'trait_lifetime> :
            ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'trait_lifetime>
            + FluidComponent
            + FluidPipeCalcPressureChange
            + FluidPipeCalcPressureLoss
        {}
        
        // first we create an therminol pipe struct
        // and start implementing it
        struct TherminolPipe<'pipe_lifetime> {

            therminol_properties_reference: &'pipe_lifetime dyn FluidProperties,
            fluid_temp: ThermodynamicTemperature,
            fluid_mass_flowrate: MassRate,

            internal_pressure: Pressure,
            incline_angle: Angle,
            component_length: Length,
            hydraulic_diameter: Length,

            pressure_loss: Pressure,
            form_loss_k: f64,
            absolute_roughness: Length,

        }

        impl<'pipe_lifetime> 
            TherminolPipeTraits<'pipe_lifetime> for TherminolPipe<'pipe_lifetime> {}

        impl<'pipe_lifetime> 
            FluidPipeCalcPressureChange for TherminolPipe<'pipe_lifetime> {
            }

        impl<'pipe_lifetime> 
            FluidPipeCalcPressureLoss for TherminolPipe<'pipe_lifetime> {

                fn get_pipe_form_loss_k(&mut self) -> f64 {
                    return self.form_loss_k;
                }

                fn get_pipe_form_loss_k_immutable(&self) -> f64 {
                    return self.form_loss_k;
                }

                /// return absolute roughness for pipe
                /// for a typical copper pipe
                /// it is 0.002 mm 
                /// i did a web search
                ///
                fn get_pipe_absolute_roughness(&mut self) -> Length {
                    return self.absolute_roughness;
                }

                fn get_pipe_absolute_roughness_immutable(&self) -> Length {
                    return self.absolute_roughness;
                }

            }

        impl<'pipe_lifetime> 
            FluidComponent for TherminolPipe<'pipe_lifetime>{
            fn get_pressure_loss(&mut self) -> Pressure {


                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let mass_flowrate = self.fluid_mass_flowrate;
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    Self::pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it

                self.pressure_loss = pressure_loss;

                return self.pressure_loss;
            }

            fn get_pressure_loss_immutable(
                &self,
                mass_flowrate: MassRate) -> Pressure {


                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let viscosity = self.get_fluid_viscosity_immutable();
                let density = self.get_fluid_density_immutable();
                let pipe_legnth = self.get_component_length_immutable();


                // calculate the pressure loss

                let pressure_loss = 
                    Self::pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it


                return pressure_loss;
            }
            fn set_pressure_loss(&mut self, pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }

            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.fluid_mass_flowrate = mass_flowrate;
            }

            fn get_mass_flowrate(&mut self) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();
                let pressure_loss = self.pressure_loss;
                let incline_angle = self.get_incline_angle();
                let internal_pressure_source = self.get_internal_pressure_source();

                let pressure_change = 
                    -pressure_loss 
                    + internal_pressure_source 
                    + self.get_hydrostatic_pressure_change();

                let mass_flowrate = 
                    Self::pipe_calculate_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k,
                        incline_angle,
                        internal_pressure_source);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.fluid_mass_flowrate;

            }

            fn get_mass_flowrate_from_pressure_loss_immutable(
                &self,
                pressure_loss: Pressure) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let fluid_viscosity = self.get_fluid_viscosity_immutable();
                let fluid_density = self.get_fluid_density_immutable();
                let pipe_length = self.get_component_length_immutable();
                let incline_angle = self.get_incline_angle_immutable();
                let internal_pressure_source = self.get_internal_pressure_source_immutable();

                let pressure_change = 
                    -pressure_loss 
                    + internal_pressure_source 
                    + <Self as FluidPipeCalcPressureChange>::
                    get_hydrostatic_pressure_change(
                        pipe_length,
                        incline_angle,
                        fluid_density);

                let mass_flowrate = 
                    Self::pipe_calculate_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k,
                        incline_angle,
                        internal_pressure_source);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it


                return mass_flowrate;

            }

                fn get_cross_sectional_area(&mut self) -> Area {
                    return self.get_hydraulic_diameter()*
                        self.get_hydraulic_diameter()*
                        PI/4.0_f64;
                }

                fn get_cross_sectional_area_immutable(&self) -> Area {
                    return self.get_hydraulic_diameter_immutable()*
                        self.get_hydraulic_diameter_immutable()*
                        PI/4.0_f64;
                }

                fn get_hydraulic_diameter(&mut self) -> Length {

                    return self.hydraulic_diameter;

                }

                fn get_hydraulic_diameter_immutable(&self) -> Length {


                    return self.hydraulic_diameter;

                }


                fn get_fluid_viscosity(&mut self) -> DynamicViscosity {

                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get viscosity

                    let fluid_viscosity = 
                        Self::viscosity(fluid_temp, fluid_properties);

                    return fluid_viscosity;
                    

                }

                fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {


                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get viscosity

                    let fluid_viscosity = 
                        Self::viscosity(fluid_temp, fluid_properties);

                    return fluid_viscosity;
                    


                }

                fn get_fluid_density(&mut self) -> MassDensity {

                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get density

                    let fluid_density = 
                        Self::density(fluid_temp, fluid_properties);

                    return fluid_density;
                    

                }

                fn get_fluid_density_immutable(&self) -> MassDensity {


                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get density

                    let fluid_density = 
                        Self::density(fluid_temp, fluid_properties);

                    return fluid_density;
                    


                }

                fn get_component_length(&mut self) -> Length {

                    return self.component_length;
                }

                fn get_component_length_immutable(&self) -> Length {

                    return self.component_length;
                }

                fn get_incline_angle(&mut self) -> Angle {

                    return self.incline_angle;
                }

                fn get_incline_angle_immutable(&self) -> Angle {

                    return self.incline_angle;
                }



                fn get_internal_pressure_source(&mut self) -> Pressure {

                    return self.internal_pressure;
                }

                fn get_internal_pressure_source_immutable(&self) -> Pressure {

                    return self.internal_pressure;
                }

                fn set_internal_pressure_source(&mut self,
                                                internal_pressure: Pressure){

                    self.internal_pressure = internal_pressure;
                }

            }

        impl<'pipe_lifetime> 
            ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'pipe_lifetime>
            for TherminolPipe<'pipe_lifetime>{

                fn get_fluid_properties(&self) -> &'pipe_lifetime dyn FluidProperties {

                    return self.therminol_properties_reference;

                }

                fn set_fluid_properties(&mut self,
                                        fluid_properties: &'pipe_lifetime dyn FluidProperties){

                    self.therminol_properties_reference = fluid_properties;

                }

                fn get_fluid_temp(&self) -> ThermodynamicTemperature {

                    return self.fluid_temp;

                }

                fn set_fluid_temp(&mut self,
                                  fluid_temp: ThermodynamicTemperature){

                    self.fluid_temp = fluid_temp;

                }
            }

        impl<'pipe_lifetime> TherminolPipe<'pipe_lifetime>{

            // let's implement a generic constructor
            fn new(fluid_temp: ThermodynamicTemperature,
                   incline_angle: Angle,
                   component_length: Length,
                   hydraulic_diameter: Length,
                   form_loss_k: f64,
                   absolute_roughness: Length,
                   therminol_properties_reference: &'pipe_lifetime TherminolVP1Properties) -> Self {

                return Self { 
                    therminol_properties_reference: therminol_properties_reference,
                    fluid_temp: fluid_temp, 
                    fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
                    internal_pressure: Pressure::new::<pascal>(0.0), 
                    incline_angle: incline_angle, 
                    component_length: component_length ,
                    hydraulic_diameter: hydraulic_diameter ,
                    pressure_loss: Pressure::new::<pascal>(0.0),
                    form_loss_k: form_loss_k ,
                    absolute_roughness: absolute_roughness,
                };



            }
        }

        // now to use this code, we need to define a few things

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let incline_angle = Angle::new::<degree>(0.0);
        let component_length  = Length::new::<meter>(0.5);
        let hydraulic_diameter = Length::new::<inch>(1.0);
        let form_loss_k: f64 = 5.0;
        let absolute_roughness = Length::new::<millimeter>(0.002);
        let therminol_properties = TherminolVP1Properties::new();

        // let's make a new therminol pipe

        let therminol_pipe = 
            TherminolPipe::new(fluid_temp, 
                               incline_angle, 
                               component_length, 
                               hydraulic_diameter, 
                               form_loss_k, 
                               absolute_roughness, 
                               &therminol_properties);

        // pass 0.2 kg/s of therminol through

        let pressure_change = 
            therminol_pipe.get_pressure_change_immutable(
                MassRate::new::<kilogram_per_second>(0.2));

        // this should be equal to -413 Pa

        approx::assert_relative_eq!(
            -413_f64,
            pressure_change.value,
            max_relative = 0.001);

        // now let's get the mass flowrate

        let mass_flowrate = 
            therminol_pipe.get_mass_flowrate_from_pressure_change_immutable(
                Pressure::new::<pascal>(-413_f64));

        approx::assert_relative_eq!(
            0.2,
            mass_flowrate.value,
            max_relative = 0.001);




    }

    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component collections
    /// 
    ///
    #[test]
    pub fn example_2_therminol_custom_component() {

        

        // we will implement a few properties here for our therminol pipe
        // for clarity we will list them in a
        // supertrait
        // This makes it easy to see what traits are being implemented here

        pub trait TherminolCustomComponentTraits<'trait_lifetime> :
            ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'trait_lifetime>
            + FluidComponent
            + FluidCustomComponentCalcPressureLoss<'trait_lifetime>
            + FluidCustomComponentCalcPressureChange<'trait_lifetime>
        {}
        
        // first we create an therminol pipe struct
        // and start implementing it
        struct TherminolCustomComponent<'pipe_lifetime> {

            therminol_properties_reference: &'pipe_lifetime dyn FluidProperties,
            fluid_temp: ThermodynamicTemperature,
            fluid_mass_flowrate: MassRate,

            internal_pressure: Pressure,
            incline_angle: Angle,
            component_length: Length,
            hydraulic_diameter: Length,

            pressure_loss: Pressure,
            absolute_roughness: Length,
            name: String,

            custom_k: &'pipe_lifetime dyn Fn(f64) -> f64,
            custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) ->f64,

        }

        impl<'pipe_lifetime> 
            TherminolCustomComponentTraits<'pipe_lifetime> for TherminolCustomComponent<'pipe_lifetime> {}

        impl<'pipe_lifetime> 
            FluidCustomComponentCalcPressureChange<'pipe_lifetime> 
                for TherminolCustomComponent<'pipe_lifetime> {
            }

        impl<'pipe_lifetime> 
            FluidCustomComponentCalcPressureLoss<'pipe_lifetime> 
                for TherminolCustomComponent<'pipe_lifetime> {

                    fn get_custom_component_absolute_roughness(
                        &mut self) -> Length {

                        return self.absolute_roughness;
                    }

                    fn get_custom_component_absolute_roughness_immutable(
                        &self) -> Length {

                        return self.absolute_roughness;
                    }

                    fn get_custom_darcy(&mut self) 
                        -> &dyn Fn(f64, f64) -> f64 {

                            return self.custom_darcy.clone();

                        }


                    fn get_custom_darcy_immutable(&self) 
                        -> &dyn Fn(f64, f64) -> f64 {

                            return self.custom_darcy.clone();

                        }

                    fn get_custom_k(&mut self) 
                        -> &dyn Fn(f64) -> f64 {

                            return self.custom_k.clone();

                        }

                    fn get_custom_k_immutable(&self) 
                        -> &dyn Fn(f64) -> f64 {

                            return self.custom_k.clone();

                        }

                    fn set_custom_k(
                        &mut self,
                        custom_k: &'pipe_lifetime dyn Fn(f64) -> f64){

                        self.custom_k = custom_k;

                    }

                    fn set_custom_darcy(
                        &mut self,
                        custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64){

                        self.custom_darcy = custom_darcy;
                    }




            }

        impl<'pipe_lifetime> 
            FluidComponent for TherminolCustomComponent<'pipe_lifetime>{
            fn get_pressure_loss(&mut self) -> Pressure {

                let fluid_mass_flowrate = 
                    self.fluid_mass_flowrate;

                let cross_sectional_area = 
                    self.get_cross_sectional_area();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter();

                let fluid_viscosity = 
                    self.get_fluid_viscosity();

                let fluid_density = 
                    self.get_fluid_density();

                let component_length = 
                    self.get_component_length();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                // i need to make some immutable borrows here...
                let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;

                let pressure_loss =
                    Self::
                    fluid_custom_component_calc_pressure_loss(
                    fluid_mass_flowrate, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    component_length, 
                    absolute_roughness, 
                    custom_darcy, custom_k);

                self.pressure_loss = pressure_loss;

                return pressure_loss;


            }

            fn get_pressure_loss_immutable(
                &self,
                mass_flowrate: MassRate) -> Pressure {

                let fluid_mass_flowrate = 
                    mass_flowrate;

                let cross_sectional_area = 
                    self.get_cross_sectional_area_immutable();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter_immutable();

                let fluid_viscosity = 
                    self.get_fluid_viscosity_immutable();

                let fluid_density = 
                    self.get_fluid_density_immutable();

                let component_length = 
                    self.get_component_length_immutable();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness_immutable();

                // i need to make some immutable borrows here...
                let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;

                let pressure_loss =
                    Self:: fluid_custom_component_calc_pressure_loss(
                        fluid_mass_flowrate, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        component_length, 
                        absolute_roughness, 
                        custom_darcy, custom_k);


                return pressure_loss;

            }
            fn set_pressure_loss(&mut self, pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }

            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.fluid_mass_flowrate = mass_flowrate;
            }

            fn get_mass_flowrate(&mut self) -> MassRate {


                //i'll have to get the pressure change
                //
                // pressure_change = 
                // - pressure_change
                // + hydrostatic pressure change
                // + internal pressure source
                //

                // internal pressure source
                let internal_pressure_source = 
                    self.get_internal_pressure_source();

                // hydrostatic pressure
                let incline_angle = 
                    self.get_incline_angle();

                let hydrostatic_pressure_change =
                    self.get_hydrostatic_pressure_change();

                // pressure_loss term
                //
                //
                let pressure_loss = 
                    self.get_pressure_loss();

                // now we get pressure change

                let pressure_change =
                    - pressure_loss
                    + hydrostatic_pressure_change
                    + internal_pressure_source;

                let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;


                let cross_sectional_area = 
                    self.get_cross_sectional_area();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter();

                let fluid_viscosity = 
                    self.get_fluid_viscosity();

                let fluid_density = 
                    self.get_fluid_density();

                let component_length = 
                    self.get_component_length();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                let source_pressure = 
                    self.get_internal_pressure_source();

                let mass_flowrate =
                    Self::
                    fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        component_length, 
                        absolute_roughness, 
                        incline_angle, 
                        source_pressure, 
                        custom_darcy, 
                        custom_k);

                self.fluid_mass_flowrate = mass_flowrate;

                return mass_flowrate;
            }

            fn get_mass_flowrate_from_pressure_loss_immutable(
                &self,
                pressure_loss: Pressure) -> MassRate {


                //i'll have to get the pressure change
                //
                // pressure_change = 
                // - pressure_change
                // + hydrostatic pressure change
                // + internal pressure source
                //

                // internal pressure source
                let internal_pressure_source = 
                    self.get_internal_pressure_source_immutable();

                // hydrostatic pressure

                let incline_angle = 
                    self.get_incline_angle_immutable();


                let hydrostatic_pressure_change =
                    self.get_hydrostatic_pressure_change_immutable();


                // now we get pressure change

                let pressure_change =
                    - pressure_loss
                    + hydrostatic_pressure_change
                    + internal_pressure_source;

                let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;


                let cross_sectional_area = 
                    self.get_cross_sectional_area_immutable();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter_immutable();

                let fluid_viscosity = 
                    self.get_fluid_viscosity_immutable();

                let fluid_density = 
                    self.get_fluid_density_immutable();

                let component_length = 
                    self.get_component_length_immutable();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness_immutable();

                let source_pressure = 
                    self.get_internal_pressure_source_immutable();

                let mass_flowrate =
                    Self::
                    fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        component_length, 
                        absolute_roughness, 
                        incline_angle, 
                        source_pressure, 
                        custom_darcy, 
                        custom_k);

                return mass_flowrate;
            }

                fn get_cross_sectional_area(&mut self) -> Area {
                    return self.get_hydraulic_diameter()*
                        self.get_hydraulic_diameter()*
                        PI/4.0_f64;
                }

                fn get_cross_sectional_area_immutable(&self) -> Area {
                    return self.get_hydraulic_diameter_immutable()*
                        self.get_hydraulic_diameter_immutable()*
                        PI/4.0_f64;
                }

                fn get_hydraulic_diameter(&mut self) -> Length {

                    return self.hydraulic_diameter;

                }

                fn get_hydraulic_diameter_immutable(&self) -> Length {


                    return self.hydraulic_diameter;

                }


                fn get_fluid_viscosity(&mut self) -> DynamicViscosity {

                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get viscosity

                    let fluid_viscosity = 
                        Self::viscosity(fluid_temp, fluid_properties);

                    return fluid_viscosity;
                    

                }

                fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {


                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get viscosity

                    let fluid_viscosity = 
                        Self::viscosity(fluid_temp, fluid_properties);

                    return fluid_viscosity;
                    


                }

                fn get_fluid_density(&mut self) -> MassDensity {

                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get density

                    let fluid_density = 
                        Self::density(fluid_temp, fluid_properties);

                    return fluid_density;
                    

                }

                fn get_fluid_density_immutable(&self) -> MassDensity {


                    // get fluid temp first
                    let fluid_temp = self.get_fluid_temp();

                    // then the fluid properties

                    let fluid_properties = self.get_fluid_properties();

                    // let's get density

                    let fluid_density = 
                        Self::density(fluid_temp, fluid_properties);

                    return fluid_density;
                    


                }

                fn get_component_length(&mut self) -> Length {

                    return self.component_length;
                }

                fn get_component_length_immutable(&self) -> Length {

                    return self.component_length;
                }

                fn get_incline_angle(&mut self) -> Angle {

                    return self.incline_angle;
                }

                fn get_incline_angle_immutable(&self) -> Angle {

                    return self.incline_angle;
                }



                fn get_internal_pressure_source(&mut self) -> Pressure {

                    return self.internal_pressure;
                }

                fn get_internal_pressure_source_immutable(&self) -> Pressure {

                    return self.internal_pressure;
                }

                fn set_internal_pressure_source(&mut self,
                                                internal_pressure: Pressure){

                    self.internal_pressure = internal_pressure;
                }

            }

        impl<'pipe_lifetime> 
            ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'pipe_lifetime>
            for TherminolCustomComponent<'pipe_lifetime>{

                fn get_fluid_properties(&self) -> &'pipe_lifetime dyn FluidProperties {

                    return self.therminol_properties_reference;

                }

                fn set_fluid_properties(&mut self,
                                        fluid_properties: &'pipe_lifetime dyn FluidProperties){

                    self.therminol_properties_reference = fluid_properties;

                }

                fn get_fluid_temp(&self) -> ThermodynamicTemperature {

                    return self.fluid_temp;

                }

                fn set_fluid_temp(&mut self,
                                  fluid_temp: ThermodynamicTemperature){

                    self.fluid_temp = fluid_temp;

                }
            }

        impl<'pipe_lifetime> TherminolCustomComponent<'pipe_lifetime>{

            // let's implement a generic constructor
            fn new(fluid_temp: ThermodynamicTemperature,
                   incline_angle: Angle,
                   component_length: Length,
                   hydraulic_diameter: Length,
                   absolute_roughness: Length,
                   therminol_properties_reference: &'pipe_lifetime TherminolVP1Properties,
                   custom_k: &'pipe_lifetime dyn Fn(f64)-> f64 ,
                   custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64 ) -> Self {

                return Self { 
                    name: "pipe_1".to_string(),
                    therminol_properties_reference: therminol_properties_reference,
                    fluid_temp: fluid_temp, 
                    fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
                    internal_pressure: Pressure::new::<pascal>(0.0), 
                    incline_angle: incline_angle, 
                    component_length: component_length ,
                    hydraulic_diameter: hydraulic_diameter ,
                    pressure_loss: Pressure::new::<pascal>(0.0),
                    absolute_roughness: absolute_roughness,
                    custom_k: custom_k,
                    custom_darcy: custom_darcy,
                };

            }

            fn get_name(&self) -> &str {
                return &self.name;
            }

            fn set_name(&mut self, name: &str) {

                self.name = name.to_string();
            }

        }

        // now to use this code, we need to define a few things

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let incline_angle = Angle::new::<degree>(0.0);
        let component_length  = Length::new::<meter>(0.5);
        let hydraulic_diameter = Length::new::<inch>(1.0);
        let absolute_roughness = Length::new::<millimeter>(0.002);
        let therminol_properties = TherminolVP1Properties::new();

        fn custom_darcy(_reynolds_number: f64,
                        _roughness_ratio:f64) -> f64 {

            return 0.0;
        }

        fn custom_k(reynolds_number: f64) -> f64 {

            // the correlation is:
            // 18.0 + 93000/Re^1.35
            //

            if reynolds_number > 0.0 {

                return 18.0 + 93000_f64/reynolds_number.powf(1.35);
            }

            if reynolds_number < 0.0 {

                let abs_reynolds_number = reynolds_number.abs();
                let fldk = 18.0 + 93000_f64/abs_reynolds_number.powf(1.35);

                return -fldk;

            }

            // return no fldk = 0 for no flow, doesn't really matter anyway
            // because Be_D = 0.5 fldk * Re^2
            // Re = 0
            // and we expect Be_D = 0
            // so fldk can be 0 and it still makes physical sense
            // ie Be_D = 0 when Re = 0
            return 0.0;

        }

        // let's make a new therminol pipe

        let mut therminol_pipe = 
            TherminolCustomComponent::new(fluid_temp, 
                               incline_angle, 
                               component_length, 
                               hydraulic_diameter, 
                               absolute_roughness, 
                               &therminol_properties,
                               &custom_k,
                               &custom_darcy);

        // pass 0.2 kg/s of therminol through

        let pressure_change = 
            therminol_pipe.get_pressure_change_immutable(
                MassRate::new::<kilogram_per_second>(0.2));

        // this should be equal to -1559 Pa

        approx::assert_relative_eq!(
            -1559_f64,
            pressure_change.value,
            max_relative = 0.001);

        // now let's get the mass flowrate

        let mass_flowrate = 
            therminol_pipe.get_mass_flowrate_from_pressure_change_immutable(
                Pressure::new::<pascal>(-1559_f64));

        approx::assert_relative_eq!(
            0.2,
            mass_flowrate.value,
            max_relative = 0.001);

        // you should also be able to set and get the name of pipes

        // but this is purely up to you

        let name = "my_therminol_pipe";

        therminol_pipe.set_name(name);

        let test_name = therminol_pipe.get_name();

        assert_eq!(name, test_name);



    }
}

