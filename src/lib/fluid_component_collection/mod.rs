use uom::si::f64::{Pressure, MassRate};
use uom::si::pressure::pascal;

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
pub trait FluidComponentCollection<'trait_lifetime> {


    /// returns a copy of the fluid component vector
    /// containing mutable elements
    ///
    /// 
    fn get_mutable_fluid_component_vector(&mut self) 
        -> Vec<&mut dyn FluidComponent> ;

    /// returns a copy of the fluid component vector
    /// containing immutable elements
    ///
    /// you'll probably need some legwork to create a fresh
    /// object
    fn get_immutable_fluid_component_vector(&mut self) 
        -> Vec<&dyn FluidComponent>;

    /// sets the fluid component vector to a specific value
    fn set_fluid_component_vector(
        &mut self,
        fluid_component_vector: Vec<&mut dyn FluidComponent>);


    /// adds a fluid component to the collection

    fn add_fluid_component(&mut self,
                           fluid_component_vector: Vec<&'trait_lifetime mut dyn FluidComponent>,
                           fluid_component_pointer: &'trait_lifetime mut dyn FluidComponent){

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
                              fluid_component_vector: Vec<&'trait_lifetime mut dyn FluidComponent>,
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
        fluid_component_vector: Vec<&'trait_lifetime mut dyn FluidComponent>,
        fluid_component_pointer: &'trait_lifetime mut dyn FluidComponent){

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

/// contains methods which take a fluid component
/// vector and calculate mass flowrates and pressure changes
/// and losses from it
pub trait FluidComponentCollectionSeriesMethods {

    /// calculates pressure change from mass flowrate
    /// for a given fluid component collection
    /// it needs a vector of mutable references to
    /// any object which implements FluidComponent
    fn calculate_pressure_change_from_mass_flowrate(
        mass_flowrate: MassRate,
        fluid_component_vector: Vec<&mut dyn FluidComponent>) -> Pressure {

        let mut fluid_component_vector_mutable
            = fluid_component_vector;

        // we instantiate a pressure vector to store
        // the values of the pressure changes

        let mut pressure_vector: Vec<Pressure> =
            vec![];

        // the pressure vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_vector_mutable.len();

        let default_pressure_value = 
            Pressure::new::<pascal>(0.0);

        pressure_vector.resize(
            new_vector_length,
            default_pressure_value
            );

        for (index,fluid_component_pointer) in 
            fluid_component_vector_mutable.iter_mut().enumerate() {

            let fluid_component = 
                &mut *fluid_component_pointer;

            fluid_component.set_mass_flowrate(
                mass_flowrate);

            let fluid_component_pressure_change = 
                fluid_component.get_pressure_change();

            pressure_vector[index] = 
                fluid_component_pressure_change;

        }

        let mut final_pressure_change: Pressure =
            default_pressure_value;
        // now we sum everything up

        for pressure_change in pressure_vector {

            final_pressure_change += pressure_change;

        }

        return final_pressure_change;

    }



}
