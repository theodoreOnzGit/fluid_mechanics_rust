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
use uom::si::f64::{Pressure, MassRate};
use uom::si::mass_rate::kilogram_per_second;

use crate::fluid_component_calculation::FluidComponent;

/// a fluid component collection,
/// which contains fluid components stored into a vector
/// and should contain some methods for CRUD operations
///
/// Create
/// Read
/// Update
/// Delete
///
pub trait FluidComponentCollection<'trait_lifetime> : FluidComponentCollectionMethods{



    /// returns a copy of the fluid component vector
    /// containing immutable elements
    ///
    /// you'll probably need some legwork to create a fresh
    /// object
    fn get_immutable_fluid_component_vector(&self) 
        -> &Vec<&'trait_lifetime dyn FluidComponent>;

    /// sets the fluid component vector to a specific value
    fn set_fluid_component_vector(
        &mut self,
        fluid_component_vector: Vec<&'trait_lifetime dyn FluidComponent>);


    /// adds a fluid component to the collection

    fn add_fluid_component(
        &mut self,
        fluid_component_vector: Vec<&'trait_lifetime dyn FluidComponent>,
        fluid_component_pointer: &'trait_lifetime dyn FluidComponent){

        // first i make a mutable copy of the component vector
        let mut fluid_component_vector_mutable =
            fluid_component_vector;

        // then i push the pointer to this mutable copy
        fluid_component_vector_mutable.push(fluid_component_pointer);

        // next i set the fluid component vector
        self.set_fluid_component_vector(fluid_component_vector_mutable);

    }


    /// removes a fluid component by index from the collection

    fn remove_fluid_component(&mut self,
                              fluid_component_vector: Vec<&'trait_lifetime dyn FluidComponent>,
                              component_index: usize){

        // first i make a mutable copy of the component vector
        let mut fluid_component_vector_mutable =
            fluid_component_vector;

        // i remove the index from the vector 
        // (note that there may be a case where the vector is smaller than
        // the given index),
        // however, the remove method already has a panic if the 
        // vector is shorter than the given index

        fluid_component_vector_mutable.remove(component_index);

        // next i set the fluid component vector
        self.set_fluid_component_vector(fluid_component_vector_mutable);
    }

    /// returns read only a pointer of the fluid component 
    /// given an index

    fn get_fluid_component(
        &'trait_lifetime mut self,
        component_index: usize) -> &'trait_lifetime dyn FluidComponent {

        // first let's access the fluid component

        let fluid_component_vector =
            self.get_immutable_fluid_component_vector();

        let fluid_component_pointer = 
            fluid_component_vector[component_index];

        return fluid_component_pointer;

    }

    /// updates the fluid component at the specified
    /// index with a fluid component supplied by the user

    fn update_fluid_component(
        &mut self,
        component_index: usize,
        fluid_component_vector: Vec<&'trait_lifetime dyn FluidComponent>,
        fluid_component_pointer: &'trait_lifetime dyn FluidComponent){

        // first i make a mutable copy of the component vector
        let mut fluid_component_vector_mutable =
            fluid_component_vector;

        // then i change the pointer in this mutable copy
        fluid_component_vector_mutable[component_index]
            = fluid_component_pointer;

        // next i set the fluid component vector
        self.set_fluid_component_vector(fluid_component_vector_mutable);
    }


}

/// contains methods to get pressure loss 
/// and pressure change and mass flowrate based on 
/// current state of the fluid component collection
pub trait FluidComponentCollectionMethods{

    /// calculates pressure loss when given a mass flowrate
    fn get_pressure_loss(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure {

        // for pressure losses, we compare the pressure change at
        // zero mass flowrate to pressure change at the desired
        // mass flowrate
        // noting that 
        //
        // pressure_change = - pressure_loss + hydrostatic pressure +
        // internal pressure


        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let current_pressure_change = 
            self.get_pressure_change(fluid_mass_flowrate);

        let pressure_change_due_to_losses = 
            current_pressure_change - reference_pressure_change;

        let pressure_loss = -pressure_change_due_to_losses;

        return pressure_loss;

    }

    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure;

    /// calculates mass flowrate from pressure change

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate;

    /// calculates mass flowrate from pressure loss
    
    fn get_mass_flowrate_from_pressure_loss(
        &self,
        pressure_loss: Pressure) -> MassRate {

        // for this, the default implementation is
        // to obtain pressure change
        //
        // pressure_change = -pressure_loss +
        // hydrostatic pressure
        // + internal pressure
        //
        // to get the latter two terms, i can obtain
        // pressure change when mass flowrate is zero
        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let pressure_change = 
            -pressure_loss + reference_pressure_change;

        // now let's calculate the mass flowrate

        return self.get_mass_flowrate_from_pressure_change(pressure_change);
    }


}

