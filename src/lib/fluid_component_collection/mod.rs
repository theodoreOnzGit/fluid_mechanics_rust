use peroxide::fuga::RootFind::Newton;
use uom::num_traits::Float;
use uom::si::f64::{Pressure, MassRate};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;

use crate::fluid_component_calculation::FluidComponent;


extern crate peroxide;
use peroxide::prelude::*;


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
///
/// this assumes that all the components in the vector
/// are connected in series
pub trait FluidComponentCollectionSeriesMethods {

    /// calculates pressure change from mass flowrate
    /// for a given fluid component collection
    /// it needs a vector of mutable references to
    /// any object which implements FluidComponent
    fn calculate_pressure_change_from_mass_flowrate(
        mass_flowrate: MassRate,
        fluid_component_vector: &Vec<&dyn FluidComponent>) -> Pressure {


        // we instantiate a pressure vector to store
        // the values of the pressure changes

        let mut pressure_vector: Vec<Pressure> =
            vec![];

        // the pressure vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_vector.len();

        let default_pressure_value = 
            Pressure::new::<pascal>(0.0);

        pressure_vector.resize(
            new_vector_length,
            default_pressure_value
            );

        for (index,fluid_component_pointer) in 
            fluid_component_vector.iter().enumerate() {

            let fluid_component = 
                &*fluid_component_pointer;


            let fluid_component_pressure_change = 
                fluid_component.get_pressure_change_immutable(mass_flowrate);

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

    /// calculates mass flowrate from pressure change
    /// for a given fluid component collection
    /// it needs a vector of mutable references to
    /// any object which implements FluidComponent
    fn calculate_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        fluid_component_vector: &Vec<&dyn FluidComponent>) -> MassRate {

        // a few key issues here:
        //
        // the method i'm going to use here is iteration
        //
        // which means I have to guess a mass flowrate
        // and obtain pressure change until the
        // pressure change matches the desired pressure change
        //
        // How then can I guess it intelligently?
        // without having the user set bounds?
        // 
        // First, we can get a baseline pressure change
        // ie when mass flowrate = 0 
        // 
        // We can then set the mass flowrate > 0  to some amount
        // and mass flowrate < 0 to some amount and 
        // take a look at the trends
        //
        // for newtonian fluid flow, we should infer that
        // higher pressure loss means higher flowrate all else equal
        //
        // for the most part, we don't have osciallting functions
        // or inflexion points for pressure loss vs reynolds number
        //
        //
        // Hence, Newton Raphson should be quite stable in theory
        // 
        //
        // The other method should be bisection, if all else fails
        // I could use mass flowrate = 0 as one bound
        //
        // and an initial bound of mass flowrate = 1kg/s
        //
        // if i find that mass flowrate is more than 1kg/s (unlikely)
        //
        // increase bound by 10
        // and then check again
        //
        // then use 1kg/s as the lower bound and 10 kg/s as the upper bound
        // and then perform bisection (this is a fallback and may
        // tend to be slow)
        //
        // The last issue is how much error to tolerate in terms of
        // pressure change should the pressure change be zero
        //
        // my take is that it should be an absolute value
        // based on a real error scale
        //
        // it should be 1 mm h2o at room temp because
        // this is usually absolute the manotmeter error
        // This is about 9.8 pascals or 10 pascals
        //
        // Therefore, my absolute tolerance should be within 
        // 7 Pa


        // first let's find the pressure change at zero, 1 kg/s
        // and -1 kg/s


        let zero_mass_flow: MassRate 
            = MassRate::new::<kilogram_per_second>(0.0);



        let pressure_change_0kg_per_second: Pressure 
            = Self::calculate_pressure_change_from_mass_flowrate(
                zero_mass_flow, 
                fluid_component_vector);

        // now we will check if the difference is about 9 Pa
        // from zero flow
        // which is that manometer reading error
        //
        // if that is so, then return mass flowrate = 0

        let pressure_loss_pascals = 
            -(pressure_change - pressure_change_0kg_per_second).value;

        if pressure_loss_pascals.abs() < 10_f64 {
            return zero_mass_flow;
        }


        // present issue: 
        // trait objects can be moved (ie used once)
        // but after using, they are finished...
        //
        // i cannot exactly clone them because this is not object
        // safe. Ie, the cloning process cannot know the size
        // of the struct at compile time 
        // traits aren't exactly well suited for 
        // methods which take in the mutable state
        //
        // nevertheless
        //
        // I can extract the state of an object and convert that
        // into a vector with size known at compile time
        //
        // However, with many potential trait objects bearing the same
        // kind of method with different size, and different required
        // data
        //
        // eg. 3 pipes and 1 flowmeter  or variations of these
        //
        // i cannot really know the size of the trait object at compile
        // time, or the required properties they contain
        //
        // The solution then is to use mutable borrows of
        // these objects rather than the actual object itself 
        // which then becomes deleted
        //
        // So then parallelism with trait objects becomes QUITE
        // challenging due to the mutability requirement
        //
        // I just hope they are not really needed =(
        //
        // However, if the functions required do NOT need a mutable
        // reference to self or anything, then we are in good shape
        //
        // Doing so however, we then do not have our usual OOP paradigms
        // where we change object state before invoking a get()
        // function

        let one_kg_per_second_mass_flow: MassRate
            = MassRate::new::<kilogram_per_second>(1.0);

        let pressure_change_1kg_per_second: Pressure 
            = Self::calculate_pressure_change_from_mass_flowrate(
                zero_mass_flow, 
                fluid_component_vector);

        let pressure_loss_1kg_per_second: Pressure = 
            -(pressure_change_1kg_per_second - pressure_change_0kg_per_second);

        // now we want to check if pressure change is positive or negative
        //
        // to do so, i compare it against a baseline pressure change
        // where mass flowrate is zero
        // if pressure loss is positive, we have forward flow
        // if pressure loss is negative, we have backflow
        //

        let forward_flow_true: bool =
            pressure_loss_pascals > 0.0 ;

        // if the flow is forward flow, then we can check we can then check if
        // it is in bounds, (between 0.0 kg/s and 1.0 kg/s)
        // if out of bounds, we can panic or increase the bound
        //
        // to check this, i want to see if the magnitude of
        // pressure loss at 1kg per second
        // is greater than the magnitude of pressure loss 
        // caused by the flow


        let positive_flow_above_1kg_per_second: bool = 
            pressure_loss_1kg_per_second.value.abs() >
            pressure_loss_pascals.abs();

        if positive_flow_above_1kg_per_second {
            // if the flow is above 1kg/s, then temporarily
            // panic
            unimplemented!();

        }

        if forward_flow_true != true {

            // if the flow is below 0kg/s
            // temporarily 
            // panic
            unimplemented!();

        }

        // if forward flow is true, then i want to iteratively calculate 
        // pressure changes using mass flowrates until the limit is reached

        // i'm going to use the peroxide library 
        //

        let mass_flow_from_pressure_chg_root = 
            |mass_flow_kg_per_s: AD| -> AD {

            let mass_flow_kg_per_s_double: f64 =
                mass_flow_kg_per_s.x();

            let mass_rate = 
                MassRate::new::<kilogram_per_second>(
                    mass_flow_kg_per_s_double);


            let pressure_change = 
                Self::calculate_pressure_change_from_mass_flowrate(
                mass_rate, 
                fluid_component_vector);

            // now i've obtained the pressure change, i convert it to f64

            let pressure_change_pascals_f64 = 
                pressure_change.value;

            // since we are finding root, then we must also
            // subtract it from our pressure change value


            let pressure_change_error: f64 =
                pressure_change_pascals_f64 - 
                pressure_change.value;

            return AD0(pressure_change_error);

        };

        // note: this function mutates the value of fluid_component_vector,
        // and is thus incompatible with peroxide libraries...
        // I'll need to rewrite the libraries in terms of immutable functions
        let mass_flowrate_result 
            = newton(
                mass_flow_from_pressure_chg_root,
                0.5, // initial guess 0.5 kg/s
                100,
                1e-8);








        unimplemented!();
    }

}