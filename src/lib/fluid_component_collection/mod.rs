use uom::si::f64::{Pressure, MassRate};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;

use crate::fluid_component_calculation::FluidComponent;

// the peroxide crate for root finders

// another crate for root finders, in fact this package specialises in root
// finding
extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;


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
pub trait FluidComponentCollectionMethods {

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

/// contains associated functions which take a fluid component
/// vector and calculate mass flowrates and pressure changes
/// and losses from it
///
/// this assumes that all the components in the vector
/// are connected in parallel
pub trait FluidComponentCollectionParallelAssociatedFunctions {


    /// calculates mass flowrate given a pressure change
    /// across each pipe or component in the parallel
    /// arrangement
    fn calculate_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        fluid_component_vector: &Vec<&dyn FluidComponent>) -> MassRate {
        // we instantiate a mass_flowrate vector to store
        // the values of the mass_flowrate changes

        let mut mass_flowrate_vector: Vec<MassRate> =
            vec![];

        // the mass_flowrate vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_vector.len();

        let default_mass_flowrate_value = 
            MassRate::new::<kilogram_per_second>(0.0);

        mass_flowrate_vector.resize(
            new_vector_length,
            default_mass_flowrate_value
            );

        for (index,fluid_component_pointer) in 
            fluid_component_vector.iter().enumerate() {
                
                // first we get an immutable reference from
                // the mutable reference

                let fluid_component = 
                    &*fluid_component_pointer;


                let fluid_component_mass_flowrate = 
                    fluid_component.get_mass_flowrate_from_pressure_change_immutable(
                        pressure_change);

                mass_flowrate_vector[index] = 
                    fluid_component_mass_flowrate;

            }

        let mut final_mass_flowrate = 
            default_mass_flowrate_value;

        for mass_flowrate in mass_flowrate_vector {

            final_mass_flowrate += mass_flowrate;

        }

        return final_mass_flowrate;
    }
}

/// contains associated functions which take a fluid component
/// vector and calculate mass flowrates and pressure changes
/// and losses from it
///
/// this assumes that all the components in the vector
/// are connected in series
pub trait FluidComponentCollectionSeriesAssociatedFunctions {


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
                
                // first we get an immutable reference from
                // the mutable reference

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

        if pressure_loss_pascals.abs() < 9_f64 {
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

        // if pressure loss is positive, we have forward flow
        // if pressure loss is negative, we have backflow
        //

        let forward_flow_true: bool =
            pressure_loss_pascals > 0.0 ;


        // if forward flow is true, then i want to iteratively calculate 
        // pressure changes using mass flowrates until the limit is reached

        // i'm going to use the peroxide library 
        //


        // this is for use in the roots library
        let mass_flow_from_pressure_chg_root = 
            |mass_flow_kg_per_s: f64| -> f64 {

            let mass_flow_kg_per_s_double = mass_flow_kg_per_s; 

            let mass_rate = 
                MassRate::new::<kilogram_per_second>(
                    mass_flow_kg_per_s_double);


            let pressure_change_tested = 
                Self::calculate_pressure_change_from_mass_flowrate(
                mass_rate, 
                fluid_component_vector);

            // now i've obtained the pressure change, i convert it to f64

            let pressure_change_user_stipulated_pascals_f64 = 
                pressure_change.value;

            // since we are finding root, then we must also
            // subtract it from our pressure change value


            let pressure_change_error: f64 =
                pressure_change_user_stipulated_pascals_f64 - 
                pressure_change_tested.value;

            return pressure_change_error;

        };

        // note: this function mutates the value of fluid_component_vector,
        // and is thus incompatible with peroxide libraries...
        // I'll need to rewrite the libraries in terms of immutable functions
        //
        // But having done so, I want to use the newton raphson method to
        // try and converge this result, hopefully within 30 iterations

        let mut convergency = SimpleConvergency { eps:1e-15f64, max_iter:30 };

        let mut mass_flowrate_result =
            if forward_flow_true != true {

                // i will search between -10 and 0 for the bracketing
                let mass_flowrate_result 
                    = find_root_brent(
                        -10_f64,
                        -0_f64,
                        &mass_flow_from_pressure_chg_root,
                        &mut convergency);


                // if loop returns this mass flowrate result
                mass_flowrate_result

            } else {
                let mass_flowrate_result 
                    = find_root_brent(
                        10_f64,
                        0_f64,
                        &mass_flow_from_pressure_chg_root,
                        &mut convergency);
                
                // if loop returns this mass flowrate result
                mass_flowrate_result
            };

        // the above results only work for ranges of 15 kg/s and
        // -15 kg/s

        // now if the newton raphson method does not converge within the
        // set number of iterations, I want it to use bisection
        // which should fall back to bisection
        // This function is not meant ot be used by the end user
        // but is instead called by another function
        //
        //
        // this function is expected to take in an automatic
        // differentiation function
        //
        // which takes the following form
        //
        // function(mass_flowrate_kg_per_second) -> pressure_pascals
        //
        // both mass flowrate and pressure are f64 but in SI units
        // as i'm trying to solve pipe network problems and flow in series
        // i consider the highest volume of flow possible for such a system
        //
        // the pressure actually measures the difference between the
        // guessed pressure loss in the iteration
        // and the actual pressure loss specified by the user
        //
        // The guiness book of world records shows that the amazon
        // river has a flowrate of about 200,000 m3/s
        // https://www.guinnessworldrecords.com/world-records/greatest-river-flow
        //
        // in other words about 200,000,000 kg/s
        //
        // We never expect man made 
        // piping systems to have this much flow 
        //
        // But this would be a good upper bound for bisection solver.
        //
        //
        //
        // If we cannot find a root in this range,
        // then it's likely there is no possible root at all
        //
        // the inline thingy here is just to help the code
        // speed up a bit
        //
        // However, I don't want to go to such an upper limit so
        // quickly,
        //
        // I'll do 10,000 kg/s in each flow branch first
        // then 200,000,000


        mass_flowrate_result = 
            match mass_flowrate_result {
                Ok(_mass_flowrate) => 
                    return MassRate::new::<kilogram_per_second>(_mass_flowrate),
                Err(_error_msg) => {

                    mass_flowrate_result 
                        = find_root_brent(
                            10_000_f64,
                            -10_000_f64,
                            &mass_flow_from_pressure_chg_root,
                            &mut convergency);

                    mass_flowrate_result
                }
            };

        mass_flowrate_result = 
            match mass_flowrate_result {
                Ok(_mass_flowrate) => 
                    return MassRate::new::<kilogram_per_second>(_mass_flowrate),
                Err(_error_msg) => {

                    mass_flowrate_result 
                        = find_root_brent(
                            20_000_000_f64,
                            -20_000_000_f64,
                            &mass_flow_from_pressure_chg_root,
                            &mut convergency);

                    mass_flowrate_result
                }
            };
        //return mass_flowrate.unwrap();
        return MassRate::new::<kilogram_per_second>(mass_flowrate_result.unwrap());
    }

}

#[cfg(test)]
pub mod fluid_component_collection_test_and_examples {

    use std::f64::consts::PI;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::standard_pipe_calc
        ::{FluidPipeCalcPressureLoss};
    use crate::fluid_component_collection::{
        FluidComponentCollection, FluidComponentCollectionMethods,
        FluidComponentCollectionSeriesAssociatedFunctions};
    use uom::si::dynamic_viscosity::{millipascal_second};
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal};
    use uom::si::angle::degree;


    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component collections
    ///
    /// First i make a typical fluid component, a set of air pipes
    /// perhaps 10 air pipes and i want to put them in series
    #[test]
    pub fn simple_fluid_collection_example_1 () {

        
        // first we create an air pipe struct
        //
        struct AirPipe {
            mass_flowrate: MassRate,
            pressure_loss: Pressure,
        }

        // we implement get and set methods for each of the 
        // properties, you can set these properties in the constructor
        // or you can simply return the appropriate values in the 
        // functions
        // 
        // likewise, when you get the mass flowrate
        // or density, you can invoke calculation methods straightaway
        // 
        // but for calculation methods, you can "inherit" the default
        // trait implementations for a generic fluid pipe
        impl FluidComponent for AirPipe {

            /// gets the mass flowrate of the component
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

                let mass_flowrate = 
                    AirPipe::pipe_calc_mass_flowrate(
                        pressure_loss, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.mass_flowrate;
            }

            /// gets the mass flowrate of the component
            /// with immutable instance of self
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

                let mass_flowrate = 
                    AirPipe::pipe_calc_mass_flowrate(
                        pressure_loss, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                return mass_flowrate;
            }

            /// sets the mass flowrate of the component
            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.mass_flowrate = mass_flowrate;
            }


            /// pressure change is accounts for total pressure
            /// differential between start and end point of the pipe,
            /// including hydrostatic pressure and any sources
            /// which may contribute to the pressure, eg. pumps
            /// 
            /// pressure change = -pressure loss + hydrostatic pressure
            fn get_pressure_change(&mut self) -> Pressure {

                // for this, i have
                // pressure change = -pressure loss + hydrostatic pressure
                // + internal pressure
                return -self.get_pressure_loss();
            }


            fn set_pressure_change(&mut self, pressure_change:Pressure) {
                self.set_pressure_loss(-pressure_change);
            }

            /// gets pressure loss
            /// i calculate pressure loss when i invoke this method
            /// and the method comes from the 
            /// FluidPipeCalcPressureLoss trait 
            fn get_pressure_loss(&mut self) -> Pressure {

                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let mass_flowrate = self.mass_flowrate;
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    AirPipe::pipe_calc_pressure_loss(
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
                &self, mass_flowrate: MassRate) -> Pressure {

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
                    AirPipe::pipe_calc_pressure_loss(
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

            /// sets the pressure loss of the component
            fn set_pressure_loss(&mut self, pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }


            /// gets cross sectional area
            /// the inner diameter is 2 in
            /// and the area is Pi*d^2/4
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

            /// gets hydraulic diamter
            /// im giving this pipe a two inch inner diameter 
            fn get_hydraulic_diameter(&mut self) -> Length {
                return Length::new::<inch>(2.0);
            }

            fn get_hydraulic_diameter_immutable(&self) -> Length {
                return Length::new::<inch>(2.0);
            }

            /// gets fluid viscosity
            /// air has a dynamic viscosity of about 18.6 millipascal
            /// seconds
            fn get_fluid_viscosity(&mut self) -> DynamicViscosity{ 
                return DynamicViscosity::new::<millipascal_second>(18.6);
            }

            fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity{ 
                return DynamicViscosity::new::<millipascal_second>(18.6);
            }


            /// gets fluid density
            /// air density is about 1kg/m3
            fn get_fluid_density(&mut self) -> MassDensity {
                return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
            }

            fn get_fluid_density_immutable(&self) -> MassDensity {
                return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
            }

            /// gets the component length
            /// you can set the component length here
            fn get_component_length(&mut self) -> Length {
                return Length::new::<meter>(1.0);
            }

            fn get_component_length_immutable(&self) -> Length {
                return Length::new::<meter>(1.0);
            }

            /// i'm manually fixing the incline angle at zero
            /// meaning that this pipe is horizontal
            fn get_incline_angle(&mut self) -> Angle {
                return Angle::new::<degree>(0.0);
            }
            
            fn get_incline_angle_immutable(&self) -> Angle {
                return Angle::new::<degree>(0.0);
            }

            /// For the air pipe, there should be no internal source

            fn get_internal_pressure_source(&mut self) -> Pressure {
                return Pressure::new::<pascal>(0.0);
            }

            fn get_internal_pressure_source_immutable(&self) -> Pressure {
                return Pressure::new::<pascal>(0.0);
            }

            fn set_internal_pressure_source(
                &mut self, 
                _internal_pressure_source: Pressure
                ){
                // doesn't actually do anything,
                // i refuse to let it set anything
                //
                // rather i have it panic a special kind of panic
                // called unimplemented

                unimplemented!();

            }


        }


        // we can "inherit" methods for the pipe pressure loss
        // and mass flowrate calculations 
        //
        // all you need to do is set form loss K
        // and absolute roughness

        impl FluidPipeCalcPressureLoss for AirPipe {

            /// return form loss K for the pipe
            /// i set it at 5
            ///
            fn get_pipe_form_loss_k(&mut self) -> f64 {
                return 5.0;
            }

            fn get_pipe_form_loss_k_immutable(&self) -> f64 {
                return 5.0;
            }

            /// return absolute roughness for pipe
            /// for a typical copper pipe
            /// it is 0.002 mm 
            /// i did a web search
            ///
            fn get_pipe_absolute_roughness(&mut self) -> Length {
                return Length::new::<millimeter>(0.002);
            }

            fn get_pipe_absolute_roughness_immutable(&self) -> Length {
                return Length::new::<millimeter>(0.002);
            }
        }

        // finally you can implement a constructor

        impl AirPipe {
            pub fn new() -> Self {
                let default_mass_flowrate = 
                    MassRate::new::<kilogram_per_second>(0.0);

                let default_pressure_loss = 
                    Pressure::new::<pascal>(0.0);

                return Self { 
                    mass_flowrate: default_mass_flowrate, 
                    pressure_loss: default_pressure_loss
                }
            }
        }


        // with the AirPipe struct setup, you can caluclate
        // the pressure loss easily


        let air_pipe_1 = AirPipe::new();
        let air_pipe_2 = AirPipe::new();
        let air_pipe_3 = AirPipe::new();
        let air_pipe_4 = AirPipe::new();
        let air_pipe_5 = AirPipe::new();
        let air_pipe_6 = AirPipe::new();
        let air_pipe_7 = AirPipe::new();
        let air_pipe_8 = AirPipe::new();
        let air_pipe_9 = AirPipe::new();
        let air_pipe_10 = AirPipe::new();

        // next i make a struct of 
        struct AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fluid_component_vector_immutable: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>
        }

        impl<'air_pipe_collection_lifetime> 
            FluidComponentCollection<'air_pipe_collection_lifetime>
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {


            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'air_pipe_collection_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

        }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionMethods for
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
                fn get_pressure_change(
                    &self,
                    fluid_mass_flowrate: MassRate) -> Pressure {

                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let pressure_change = 
                        Self::calculate_pressure_change_from_mass_flowrate(
                            fluid_mass_flowrate, immutable_vector_ref);

                    return pressure_change;
                }

                fn get_mass_flowrate_from_pressure_change(
                    &self,
                    pressure_change: Pressure) -> MassRate {


                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let mass_flowrate = 
                        Self::calculate_mass_flowrate_from_pressure_change(
                            pressure_change, immutable_vector_ref);

                    return mass_flowrate;

                }


            }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionSeriesAssociatedFunctions
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {}

        // constructor is here

        impl<'air_pipe_collection_lifetime>
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fn new() -> Self {
                return Self { 
                    fluid_component_vector_immutable:  vec![]
                };
            }
        }

        
        let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];

        air_pipe_vec.push(&air_pipe_1);
        air_pipe_vec.push(&air_pipe_2);
        air_pipe_vec.push(&air_pipe_3);
        air_pipe_vec.push(&air_pipe_4);
        air_pipe_vec.push(&air_pipe_5);
        air_pipe_vec.push(&air_pipe_6);
        air_pipe_vec.push(&air_pipe_7);
        air_pipe_vec.push(&air_pipe_8);
        air_pipe_vec.push(&air_pipe_9);
        air_pipe_vec.push(&air_pipe_10);

        // now i've made my air pipe vector, i can push it into the air pipe collection
        let mut air_pipe_series = 
            AirPipeCollectionSeries::new();

        air_pipe_series.set_fluid_component_vector(air_pipe_vec);

        // now let's push a 0.1kg/s airflow through this pipe series
        //
        let pipe_airflow = MassRate::new::<kilogram_per_second>(0.1);

        // and then let's get the pressure change

        let pipe_pressure_change = air_pipe_series.
            get_pressure_change(pipe_airflow);

        // the pressure losses are about -1144 Pa
        approx::assert_relative_eq!(
            pipe_pressure_change.value,
            -174650.0,
            max_relative=0.001);

        // i will also test the get pressure loss function

        let pipe_pressure_loss = air_pipe_series.
            get_pressure_loss(pipe_airflow);

        // in this case, there is no elevation or internal
        // pressure source, so the pipe pressure losses should
        // be the same as the inverse of the pressure change
        assert_eq!(-pipe_pressure_change,
                   pipe_pressure_loss);

        // all right, so now we want to check if the same pressure loss
        // will yield us 0.001 kg/s

        let test_pressure_loss = 
            Pressure::new::<pascal>(174650.0);

        let pipe_test_air_mass_flowrate = 
            air_pipe_series.get_mass_flowrate_from_pressure_change(
                -test_pressure_loss);

        approx::assert_relative_eq!(
            pipe_airflow.value,
            pipe_test_air_mass_flowrate.value,
            max_relative=0.001);


        // the last thing to assert is whether the pressure loss of 1 pipe
        // is equal to 1/10 of the pipes in series

        let pressure_loss_1_pipe =
            air_pipe_1.get_pressure_change_immutable(pipe_airflow);

        let pressure_loss_10_pipe_series =
            air_pipe_series.get_pressure_change(pipe_airflow);

        approx::assert_relative_eq!(
            pressure_loss_1_pipe.value,
            pressure_loss_10_pipe_series.value/10.0,
            max_relative=1e-3);



        return;

    }
}

