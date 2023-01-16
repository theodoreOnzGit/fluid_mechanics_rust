use uom::num_traits::ToPrimitive;
use uom::si::f64::{Pressure, MassRate};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;

use crate::fluid_component_collection::FluidComponentCollectionMethods;

// the peroxide crate for root finders

// another crate for root finders, in fact this package specialises in root
// finding
extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;


/// contains associated functions which take a fluid component collection
/// vector and calculate mass flowrates and pressure changes
/// and losses from it
///
/// this assumes that all the components collections in the vector
/// are connected in series
pub trait FluidComponentSuperCollectionSeriesAssociatedFunctions {


    /// calculates pressure change from mass flowrate
    /// for a given fluid component collection
    /// it needs a vector of mutable references to
    /// any object which implements FluidComponentCollectionMethods
    fn calculate_pressure_change_from_mass_flowrate(
        mass_flowrate: MassRate,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> Pressure {


        // we instantiate a pressure vector to store
        // the values of the pressure changes

        let mut pressure_vector: Vec<Pressure> =
            vec![];

        // the pressure vector will have a length
        // equal to the fluid_component_collection vector

        let new_vector_length =
            fluid_component_collection_vector.len();

        let default_pressure_value = 
            Pressure::new::<pascal>(0.0);

        pressure_vector.resize(
            new_vector_length,
            default_pressure_value
            );


        for (index,fluid_component_collection) in 
            fluid_component_collection_vector.iter().enumerate() {
                

                let fluid_component_collection_pressure_change = 
                    fluid_component_collection.get_pressure_change(mass_flowrate);

                pressure_vector[index] = 
                    fluid_component_collection_pressure_change;

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
    /// for a given fluid component super collection
    /// it needs a vector of mutable references to
    /// any object which implements FluidComponent
    ///
    ///
    /// [PENDING EDITS]
    /// 
    fn calculate_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> MassRate {

        // To iteratively find mass flowrate from pressure change,
        // I first need to find an initial guess of mass flowrates with which
        // to iterate
        //
        // [PENDING EDITS]
        //
        // For this, i use the brent method from the roots crate in rust
        // as it seems quite efficient for what it's meant to do
        // it has the speed of secant but falls back to bisection
        // if things go wrong.
        //
        // One of the main issues with brent or bisection method is
        // that you need bounds for the solver to work. One of the ways
        // I derive the bounds is by looking at physical phenomena for 
        // mass flowrate.
        //
        // First, i made this package specifically for industrial use
        // cases where we talk about flow in man made pipes and fluid
        // components.
        //
        // We don't expect that man made flow to exceed a certain 
        // amount, eg. a water fall. Except for a hydroelectric dam.
        //
        // For the maximum possible fluid flowrate, i'm going to look
        // at the Amazon River.
        //
        // The guiness book of world records shows that the amazon
        // river has a flowrate of about 200,000 m3/s
        // https://www.guinnessworldrecords.com/world-records/greatest-river-flow
        //
        // in other words about 200,000,000 kg/s.
        //
        // This would in theory cover all scenarios pertaining to fluid flowrate.
        //
        // However, this may make the solver quite slow as the bounds for fluid
        // flow are so large!
        //  
        // To speed things up, we can use a small conservative bound.
        // Eg. water carrying a thermal power rating of 3GWth
        //
        // the heat capacity for water is approx 4.2 KJ/kg.K
        //
        // For a typical PWR, the water is heated from 275C to 315C
        // https://en.wikipedia.org/wiki/Pressurized_water_reactor
        //
        // This is a temp change of 40K
        //
        // Hence, the heat capacity of water going through this
        // temp transition is about 168 kJ/kg
        // or about 0.168 MJ/kg
        // or about 1.68e-4 GJ/kg
        //
        // For such a calculation, 
        // 3GWTh/0.168 GJ/kg = 17857 kg/s (approx)
        // 
        // Therefore, a more realistic upper bound is about 20,000 kg/s
        // which bounds all possible flows in industrial scenarios
        // 
        // Now of course, a user is free to override these bounds for
        // their application, but i leave it here.
        // 
        // 
        //
        // To speed applications up even faster, i'd like to skip the
        // iteration process for a few special scenarios:
        //
        // (1) zero flow
        //
        // If flow is near zero, then i just want to return the
        // mass flowrate as zero.
        //
        // By near i mean that the pressure reading is within
        // measurement error for a manometer
        // (about 1mm H2O or 9 Pa)
        //
        // 
        //


        let zero_mass_flow: MassRate 
            = MassRate::new::<kilogram_per_second>(0.0);

        let get_pressure_loss_from_pressure_change = |
            pressure_change: Pressure| -> Result<Pressure, String> {

            let zero_mass_flow: MassRate 
                = MassRate::new::<kilogram_per_second>(0.0);

            let pressure_change_0kg_per_second: Pressure 
                = Self::calculate_pressure_change_from_mass_flowrate(
                    zero_mass_flow, 
                    fluid_component_collection_vector);

            let pressure_loss_pascals = 
                -(pressure_change - pressure_change_0kg_per_second).value;

            return Ok(Pressure::new::<pascal>(pressure_loss_pascals));

        };



        // now we will check if the 
        // pressure loss due to flowrate is about 9 Pa
        // from zero flow
        // which is that manometer reading error
        //
        // For a piping system, this pressure change is tiny
        //
        // if that is so, then return mass flowrate = 0
        //
        // Of course, i am also aware that there is sometimes
        // that the function won't work when i introduce check valve
        // behaviour in fluid collections


        let user_specified_pressure_loss_pascals = 
            get_pressure_loss_from_pressure_change(
                pressure_change).unwrap().value;


        if user_specified_pressure_loss_pascals.abs() < 9_f64 {
            return zero_mass_flow;
        }

        // (2) reducing algorithms for small flows
        //
        // If my flow is relatively small, eg, <10kg/s
        // then i'd like to reduce the bounds

        // so first i want to check the pressure loss at a certain
        // mass flowrate value:
        let check_flow_pressure_loss_pascals = |
            mass_flowrate: MassRate| -> Pressure {

                let zero_mass_flow: MassRate 
                    = MassRate::new::<kilogram_per_second>(0.0);

                let pressure_change_0kg_per_second: Pressure 
                    = Self::calculate_pressure_change_from_mass_flowrate(
                        zero_mass_flow, 
                        fluid_component_collection_vector);

                let pressure_change_at_specified_mass_flow: Pressure 
                    = Self::calculate_pressure_change_from_mass_flowrate(
                        mass_flowrate, 
                        fluid_component_collection_vector);

                let pressure_loss_pascals = 
                    -(pressure_change_at_specified_mass_flow - pressure_change_0kg_per_second).value;

                return Pressure::new::<pascal>(pressure_loss_pascals);

            };

        // then i want to compare it agains the specified pressure loss
        // for both forward and backward flow
        //let check_mass_flowrate_smaller_magnitude_than_specified = 
        //    |mass_flowrate: MassRate| -> Result<bool, String> {

        //        let pressure_loss_at_specified_forward_flow
        //            = check_flow_pressure_loss_pascals(mass_flowrate);

        //        let pressure_loss_at_specified_backward_flow
        //            = check_flow_pressure_loss_pascals(-mass_flowrate);

        //        // i'll find the bigger pressure loss of the two

        //        let max_pressure_loss_at_specified_flow = 
        //            f64::max(pressure_loss_at_specified_forward_flow.value.abs(),
        //                     pressure_loss_at_specified_backward_flow.value.abs());


        //        if user_specified_pressure_loss_pascals 
        //                < max_pressure_loss_at_specified_flow {

        //                return Ok(true);

        //            } else {

        //                return Ok(false);

        //            }


        //        // now of course, for check valve behaviour, flow is always
        //        // zero when i introduce a pressure change in the backflow
        //        // in such a case, we will likely get an error by executing the
        //        // above code
        //    };

        // in such a case for check value behaviour, then i'll need to 
        // do something special
        //

        let check_for_check_valve_behaviour = 
            |mass_flowrate: MassRate| -> Result<bool, String> {

                // there are a number of ways one may do check valve behaviour
                //
                // perhaps one is to return a NaN value or f64::max when 
                // mass flowrate is being passed through one or the other side
                //
                let pressure_loss_at_specified_forward_flow
                    = check_flow_pressure_loss_pascals(mass_flowrate);

                let pressure_loss_at_specified_backward_flow
                    = check_flow_pressure_loss_pascals(-mass_flowrate);

                // i'll find the bigger pressure loss of the two

                let max_pressure_loss_at_specified_flow = 
                    f64::max(pressure_loss_at_specified_forward_flow.value.abs(),
                             pressure_loss_at_specified_backward_flow.value.abs());

                let min_pressure_loss_at_specified_flow = 
                    f64::min(pressure_loss_at_specified_forward_flow.value.abs(),
                             pressure_loss_at_specified_backward_flow.value.abs());

                // idk if this algorithm may produce errors in earlier part
                // depending how i implement check valve behaviour
                // but perhaps i'll sort that issue out later

                // but yes now that i have the bigger loss of the two,
                // i can check for max values, infinite values or NaN
                // values... These might indicate check valve behaviour

                if max_pressure_loss_at_specified_flow == f64::NAN {
                    return Ok(true);
                }

                if min_pressure_loss_at_specified_flow == f64::NAN {
                    return Ok(true);
                }

                if max_pressure_loss_at_specified_flow == f64::MAX {
                    return Ok(true);
                }

                if max_pressure_loss_at_specified_flow == f64::INFINITY {
                    return Ok(true);
                }


                // now, i also want to see if the pressure loss of
                // one side is like 1000 times or more of the other
                // side for the same pressure rate, this indicates almost
                // that there is some sort of fluidic diode behaviour

                if max_pressure_loss_at_specified_flow
                    > min_pressure_loss_at_specified_flow*1000.0 {

                        return Ok(true);

                    }

                // if smaller than the above value, we might not think
                // there is check valve behaviour

                if max_pressure_loss_at_specified_flow
                    <= min_pressure_loss_at_specified_flow*1000.0 {

                        return Ok(false);

                    }

                return Err("unable to ascertain if there is check valve \n
                behaviour".to_string());

            };

        // now I can check for check valve behaviour
        // perhaps using a small flowrate, 0.01 kg/s


        let check_valve_behaviour_result = 
            check_for_check_valve_behaviour(
                MassRate::new::<kilogram_per_second>(0.01));

        // now i should try to unwrap this result, if i cannot, then panic

        let check_valve_behaviour_exists: bool = 
            check_valve_behaviour_result.unwrap();

        // if check valve behaviour exists,
        // i want to check if the check valve or flow diode
        // is forwards or backwards biased
        //
        // now if pressure loss is greater than 0, then forward flow is true
        //
        
        let forward_flow_true: bool =
            user_specified_pressure_loss_pascals > 0.0 ;

        // in the case check valve behaviour exists,
        // i check the biasing
        if check_valve_behaviour_exists {

            let check_valve_mass_flow = MassRate::new::<kilogram_per_second>(0.1);

            let pressure_loss_at_specified_forward_flow
                = check_flow_pressure_loss_pascals(check_valve_mass_flow);

            let pressure_loss_at_specified_backward_flow
                = check_flow_pressure_loss_pascals(-check_valve_mass_flow);

            let forward_bias: bool = {
                
                let mut forward_bias = false;

                // the the pressure loss at backward flow is NAN, infinity, max or
                // 1000 times greater than the backflow, we have forward bias
                if pressure_loss_at_specified_backward_flow.value.abs() 
                    == f64::NAN {
                        forward_bias = true;
                    }

                if pressure_loss_at_specified_backward_flow.value.abs() 
                    == f64::MAX {
                        forward_bias = true;
                    }

                if pressure_loss_at_specified_backward_flow.value.abs() 
                    == f64::INFINITY {
                        forward_bias = true;
                    }

                if pressure_loss_at_specified_forward_flow.value.abs() >
                    1000.0 * pressure_loss_at_specified_backward_flow.value.abs() {
                        forward_bias = true;
                    }
                // returns the forward bias boolean
                forward_bias 

            };

            // the only times i want to return zero mass flowrate
            // is if forward bias is true and there is reverse flow
            //

            if forward_bias && !forward_flow_true {
                return zero_mass_flow;
            }

            // or there is forward flow and reverse bias

            if !forward_bias && forward_flow_true {
                return zero_mass_flow;
            }

            // if none of these is true, carry on the solving as per normal

        }


        // Now that i have a baseline pressure loss 

        // if pressure loss is positive, we have forward flow
        // if pressure loss is negative, we have backflow
        //



        // if forward flow is true, then i want to iteratively calculate 
        // pressure changes using mass flowrates until the limit is reached


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
                fluid_component_collection_vector);

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

        let mut convergency = SimpleConvergency { eps:1e-9f64, max_iter:30 };

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


/// contains associated functions which take a fluid component
/// vector and calculate mass flowrates and pressure changes
/// and losses from it
///
/// this assumes that all the components in the vector
/// are connected in parallel
pub trait FluidComponentSuperCollectionParallelAssociatedFunctions {


    /// calculates mass flowrate given a pressure change
    /// across each pipe or component in the parallel
    /// arrangement
    fn calculate_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> MassRate {
        // we instantiate a mass_flowrate vector to store
        // the values of the mass_flowrate changes

        let mut mass_flowrate_vector: Vec<MassRate> =
            vec![];

        // the mass_flowrate vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_collection_vector.len();

        let default_mass_flowrate_value = 
            MassRate::new::<kilogram_per_second>(0.0);

        mass_flowrate_vector.resize(
            new_vector_length,
            default_mass_flowrate_value
            );

        for (index,fluid_component_pointer) in 
            fluid_component_collection_vector.iter().enumerate() {
                
                // first we get an immutable reference from
                // the mutable reference

                let fluid_component = 
                    &*fluid_component_pointer;


                let fluid_component_mass_flowrate = 
                    fluid_component.get_mass_flowrate_from_pressure_change(
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

    /// calculates pressure change given a mass
    /// flowrate through a parallel collection of
    /// fluid pipes or components
    fn calculate_pressure_change_from_mass_flowrate(
        mass_flowrate: MassRate,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> Pressure {

        // for calculating pressure change in a parallel super
        // collection from
        // mass flowrate, 
        // i will need to iteratively guess the pressure change
        // across each pipe to get the specified mass flowrate

        // only thing is how do i do so?
        //
        // First thing first, I will need to guess some bounds for the brent
        // calculator, ie what pressure change bounds are appropriate?
        //
        // There are no standardised pressure change bounds for any of
        // these
        //
        // Nevertheless, they can be calculated,
        //
        // For reference, at zero mass flowrate, each parallel branch would
        // have a default pressure change. This may differ for each
        // branch. 
        //
        // taking the average of these pressure changes at zero flow case
        // i would get a pretty good guess of what the pressure change may
        // be like at zero flow
        //
        // this will then be my starting point and if i bound it by
        // the change between maximum and minimum pressure,
        // i should be able to get my bounds for zero flow
        // this case is simpler
        //
        //
        //
        //
        //
        // And then, when I supply a mass flowrate for each of these branches
        // there would be some pressure losses associated with that
        // mass flowrate
        // Again, the pressure losses expected from each branch would
        // be different
        //
        // since i supply a mass flowrate here already, I can use this
        // combined mass flowrate through all pipes
        //
        // the minimum pressure loss from any one of these branches
        // and subtract that from the maximum pressure loss
        //
        //
        //
        // This will form a pressure bound which i can plus and minus
        // minus from my default pressure change
        // 
        // Lastly, I need to add the difference between the maximum
        // and minimum of the pressure change at zero flow
        // perhaps multiply that by 1.5 to obtain pressure bounds as
        // well
        //
        // In this way, both flows due to pressure changes outside the      
        // parallel branches
        // and changes inside the parallel branches are accounted for
        //
        // in dynamic setting of bounds. 
        // and this should provide decent-ish initial guesses
        //
        
        // if mass flowrate over this series is zero, then we can calculate the bound
        // straightaway

        let user_requested_mass_flowrate = 
            mass_flowrate;

        let zero_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(0.0);

        // if the mass flowrate is almost zero (1e-9 kg/s)
        // we assume flow is zero 
        // this is zero NET flow through the parallel structure
        // the branches themselves may still have flow going 
        // through them
        if user_requested_mass_flowrate.value.abs() < 1e-9_f64 {

            // in this case, the average mass flowrate through each of these
            // loops is very close to zero,
            // therefore zero flowrate is supplied
            // as a guess

            let guess_average_mass_flowrate =
                zero_mass_flowrate;

            return <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                calculate_pressure_change_using_guessed_branch_mass_flowrate(
                    guess_average_mass_flowrate, 
                    user_requested_mass_flowrate, 
                    fluid_component_collection_vector);
        }

        // if flow is non zero, then we will have to deal with 3 bounding cases
        // so that we can guess the bounds of root finding
        //
        // First case is where 
        // the internal circulation effect >> external flow 
        //
        // This will be similar to the zero pressure mass flowrate algorithm
        //
        // in that one can simply apply that mass flowrate
        // to all the branches, 
        //
        // assume that the pressure change will lie somewhere between
        // the pressure changes obtained in the various branches
        //
        // and use the maximum and minimum pressure changes to obtain bounds
        // and the solution to the equation
        //
        // For this to work, we know that the scale of the internal circulation
        // driving force is perhaps (max pressure change - min pressure change)
        //
        // if the maximum pressure loss caused by the flow is within 10% of
        // this driving force, i can say that case 1 applies. This is just
        // a guestimate
        //
        // So let's first get the zero mass flowrate pressure force measured


        // step 1: let's first get the pressure changes at
        // mass flowrate = 0.0
        //


        let zero_flow_pressure_change_est_vector = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_pressure_estimate_vector(
                zero_mass_flowrate, 
                fluid_component_collection_vector);



        let max_pressure_change_at_zero_flow = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_maximum_pressure_from_vector(
                &zero_flow_pressure_change_est_vector);

        let min_pressure_change_at_zero_flow = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_minimum_pressure_from_vector(
                &zero_flow_pressure_change_est_vector);

        let internal_circulation_driving_force_scale = 
            max_pressure_change_at_zero_flow -
            min_pressure_change_at_zero_flow;

        // step 2: now i'll apply the user_specified flowrate to all the branches
        // and calculate pressure loss

        let user_specified_flow_pressure_loss_est_vector = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_pressure_loss_estimate_vector(
                user_requested_mass_flowrate, 
                fluid_component_collection_vector);

        // note that these pressure loss values are likely positive
        // even if not though, what i'm looking for here is the
        // largest magnitude of all these pressure losses

        // to get a sense of the scale, i'm going to look for the average,
        // minimum and maximum pressure drop

        let user_specified_average_pressure_drop =
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_average_pressure_from_vector(
                &user_specified_flow_pressure_loss_est_vector);



        // now i can compare the magnitude of the internal driving force
        // to the user_specified_average_pressure_drop
        //
        // if the average pressure drop is <10% or the internal driving force,
        // then we can consider this a internal circulation dominant case

        let internal_circulation_dominant = 
            internal_circulation_driving_force_scale.value * 10.0 
            > user_specified_average_pressure_drop.value.abs();

        if internal_circulation_dominant {

            // in this case, the average mass flowrate through each of these
            // loops is very close to zero,
            // therefore zero flowrate is supplied
            // as a guess

            let guess_average_mass_flowrate =
                zero_mass_flowrate;

            return <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                calculate_pressure_change_using_guessed_branch_mass_flowrate(
                    guess_average_mass_flowrate, 
                    user_requested_mass_flowrate, 
                    fluid_component_collection_vector);
        }

        // next we can go to the other extreme, where external flowrate is 
        // dominant,
        //
        // in such a case, the internal circulation driving force (at zero flow)
        // is much smaller <10% of the external pressure driving force
        // which can be specified by the user specified average pressure drop
        // value
        //


        let external_circulation_dominant = 
            internal_circulation_driving_force_scale.value * 10.0 
            < user_specified_average_pressure_drop.value.abs();

        if external_circulation_dominant {

            // in such a case, the average guessed flowrate should be 
            // the total mass flowrate divided by the number of branches

            let number_of_branches: f64 =
                fluid_component_collection_vector.len() as f64;

            let guess_average_mass_flowrate =
                user_requested_mass_flowrate
                /number_of_branches;


            return <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                calculate_pressure_change_using_guessed_branch_mass_flowrate(
                    guess_average_mass_flowrate, 
                    user_requested_mass_flowrate, 
                    fluid_component_collection_vector);

        }

        // now that we've covered both of the extreme cases, we can check the third
        // case where the internal circulation force and external circulation force
        // both cannot be neglected
        //
        // in such a case, we expect the pressure change to be large enough
        // to be able to block flow in any one of the tubes
        
        // so it may be likely that flow in any one of those tubes is zero or
        // close to zero because some of the flow in those tubes are blocked by
        // the external pressure drop
        //
        // if scales are similar (and non zero, because we already handled the
        // zero case)
        //
        // we can take the internal driving force as a reference scale
        // calculate then 

        let pressure_deviation_percentage_from_internal_driving_force =
            (internal_circulation_driving_force_scale - 
             user_specified_average_pressure_drop).value.abs()
            /internal_circulation_driving_force_scale.value.abs()
            *100.0_f64;

        // if the deviation percentage is less than 80%, we can say they are quite
        // in the same order of magnitude or similar

        if pressure_deviation_percentage_from_internal_driving_force < 80.0 {


            // in this case, the guessed mass flowrate through each of these
            // loops can be very close to zero,
            // therefore zero flowrate is supplied
            // as a guess
            // the algorithm is similar to the internal pressure dominant
            // case,
            // but the reasoning is different

            let guess_average_mass_flowrate =
                zero_mass_flowrate;

            return <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                calculate_pressure_change_using_guessed_branch_mass_flowrate(
                    guess_average_mass_flowrate, 
                    user_requested_mass_flowrate, 
                    fluid_component_collection_vector);


        }

        // now if all of the cases are exhausted, we will just resort to a generic
        // method where the guessed flowrate for each branch is the 
        // user supplied mass flowrate/number of branches
        //
        // hopefully this will supply the correct pressure bounds to
        // guess the pressure change

        let number_of_branches: f64 =
            fluid_component_collection_vector.len() as f64;

        let guess_average_mass_flowrate =
            user_requested_mass_flowrate
            /number_of_branches;

        return <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            calculate_pressure_change_using_guessed_branch_mass_flowrate(
                guess_average_mass_flowrate, 
                user_requested_mass_flowrate, 
                fluid_component_collection_vector);

    }


    /// calculates pressure change at user specified mass flowrate
    /// given a guessed flowrate through each branch
    /// and user specified flowrate
    ///
    #[inline]
    fn calculate_pressure_change_using_guessed_branch_mass_flowrate(
        guess_average_mass_flowrate: MassRate,
        user_specified_mass_flowrate: MassRate,
        fluid_component_collection_vector: &Vec<&dyn FluidComponentCollectionMethods>) -> Pressure {


        // first i am applying the average gussed flowrate through all branches
        // this is the trivial solution
        //

        let pressure_change_est_vector = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_pressure_estimate_vector(
                guess_average_mass_flowrate, 
                fluid_component_collection_vector);

        let average_pressure_at_guessed_average_flow = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_average_pressure_from_vector(
                &pressure_change_est_vector);


        let max_pressure_change_at_guessed_average_flow = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_maximum_pressure_from_vector(
                &pressure_change_est_vector);

        let min_pressure_change_at_guessed_average_flow = 
            <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_minimum_pressure_from_vector(
                &pressure_change_est_vector);

        let pressure_diff_at_guessed_average_flow = 
            max_pressure_change_at_guessed_average_flow -
            min_pressure_change_at_guessed_average_flow;



        let static_pressure_variation_estimate = 
            pressure_diff_at_guessed_average_flow;


        // with my upper and lower bounds
        // i can now define the root function for pressure
        // we are iterating pressure across each branch


        // this is for use in the roots library
        let pressure_change_from_mass_flowrate_root = 
            |branch_pressure_change_pascals: f64| -> f64 {

                // we obtain an iterated branch pressure change
                // obtain a mass flowrate from it, by applying it to each branch
                //
                // then compare it to the user supplied mass flowrate
                //

                let iterated_pressure = 
                    Pressure::new::<pascal>(branch_pressure_change_pascals);

                let iterated_mass_flowrate =
                    <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                    calculate_mass_flowrate_from_pressure_change(
                        iterated_pressure, 
                        fluid_component_collection_vector);

                let mass_flowrate_error = 
                    iterated_mass_flowrate -
                    user_specified_mass_flowrate;

                return mass_flowrate_error.value;

        };

        // now we use the guessed average flowrates to decide upper
        // and lower bounds for the pressure loss
        //
        

        let mut user_specified_pressure_upper_bound = 
            average_pressure_at_guessed_average_flow 
            + static_pressure_variation_estimate;

        let mut user_specified_pressure_lower_bound =
            average_pressure_at_guessed_average_flow 
            - static_pressure_variation_estimate;

        // now if the upper and lower bounds are the same,
        // then we will add a 5 Pa difference to them
        //

        if user_specified_pressure_lower_bound.value ==
            user_specified_pressure_upper_bound.value {

                user_specified_pressure_lower_bound.value -= 5.0;
                user_specified_pressure_upper_bound.value += 5.0;


            }

        // i was using panic macros to debug during development
        // may wanna delete later
        //panic!("{:?}", user_specified_pressure_upper_bound);

        // i can't use a convergency value too strict, perhaps 1e-9 will do!
        //
        let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

        let pressure_change_pascals_result_user_specified_flow
            = find_root_brent(
                user_specified_pressure_upper_bound.value,
                user_specified_pressure_lower_bound.value,
                &pressure_change_from_mass_flowrate_root,
                &mut convergency);

        let pressure_change_pascals_user_specified_flow: f64 = 
            pressure_change_pascals_result_user_specified_flow.unwrap();

        return Pressure::new::<pascal>(pressure_change_pascals_user_specified_flow);
    }

    /// This function takes a mass flowrate and applies it to each
    /// branch of the pipes in parallel
    ///
    /// The resulting pressure change in each pipe is returned
    /// as a vector
    #[inline]
    fn obtain_pressure_estimate_vector(
        mass_flowrate: MassRate,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> Vec<Pressure> {

        // first we obtain pressure changes at zero mass flow
        // over each branch
        //
        // we instantiate a pressure vector to store
        // the values of the pressure changes

        let mut pressure_vector: Vec<Pressure> =
            vec![];

        // the pressure vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_collection_vector.len();

        let default_pressure_value = 
            Pressure::new::<pascal>(0.0);

        pressure_vector.resize(
            new_vector_length,
            default_pressure_value
            );



        for (index,fluid_component_collection_pointer) in 
            fluid_component_collection_vector.iter().enumerate() {
                
                // first we get an immutable reference from
                // the mutable reference

                let fluid_component_collection = 
                    &*fluid_component_collection_pointer;


                let fluid_component_collection_pressure_change = 
                    fluid_component_collection.get_pressure_change(mass_flowrate);

                pressure_vector[index] = 
                    fluid_component_collection_pressure_change;

            }

        return pressure_vector;

    }

    /// This function takes a mass flowrate and applies it to each
    /// branch of the pipes in parallel
    ///
    /// The resulting pressure loss in each pipe is returned
    /// as a vector
    #[inline]
    fn obtain_pressure_loss_estimate_vector(
        mass_flowrate: MassRate,
        fluid_component_collection_vector: 
        &Vec<&dyn FluidComponentCollectionMethods>) -> Vec<Pressure> {

        // first we obtain pressure changes at zero mass flow
        // over each branch
        //
        // we instantiate a pressure vector to store
        // the values of the pressure changes

        let mut pressure_vector: Vec<Pressure> =
            vec![];

        // the pressure vector will have a length
        // equal to the fluid_component vector

        let new_vector_length =
            fluid_component_collection_vector.len();

        let default_pressure_value = 
            Pressure::new::<pascal>(0.0);

        pressure_vector.resize(
            new_vector_length,
            default_pressure_value
            );



        for (index,fluid_component_pointer) in 
            fluid_component_collection_vector.iter().enumerate() {
                
                // first we get an immutable reference from
                // the mutable reference

                let fluid_component = 
                    &*fluid_component_pointer;


                let fluid_component_pressure_loss = 
                    fluid_component.get_pressure_loss(mass_flowrate);

                pressure_vector[index] = 
                    fluid_component_pressure_loss;

            }

        return pressure_vector;

    }

    /// this function returns the maximum pressure change within 
    /// a pressure vector
    #[inline]
    fn obtain_maximum_pressure_from_vector(
        pressure_vector: &Vec<Pressure>) -> Pressure {

        // let's get an f64 vector from the pressure vector

        let mut f64_vector: Vec<f64> =vec![];

        f64_vector.resize(
            pressure_vector.len(), 
            0.0);

        for (index,pressure_obj_pointer) in 
            pressure_vector.iter().enumerate() {

                f64_vector[index] = 
                    pressure_obj_pointer.value;

            }
        
        // now we have obtained a f64 vector from the pressure vector,
        // we can then use the max values from it
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max

        let maximum_pressure_value_pascals = 
            f64_vector
            .into_iter()
            .reduce(f64::max)
            .unwrap();


        return Pressure::new::<pascal>(maximum_pressure_value_pascals);

    }

    /// this function returns the minimum pressure change within
    /// a pressure vector
    #[inline]
    fn obtain_minimum_pressure_from_vector(
        pressure_vector: &Vec<Pressure>) -> Pressure {

        // let's get an f64 vector from the pressure vector

        let mut f64_vector: Vec<f64> =vec![];

        f64_vector.resize(
            pressure_vector.len(), 
            0.0);

        for (index,pressure_obj_pointer) in 
            pressure_vector.iter().enumerate() {

                f64_vector[index] = 
                    pressure_obj_pointer.value;

            }
        
        // now we have obtained a f64 vector from the pressure vector,
        // we can then use the max values from it
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max

        let minimum_pressure_value_pascals = 
            f64_vector
            .into_iter()
            .reduce(f64::min)
            .unwrap();


        return Pressure::new::<pascal>(minimum_pressure_value_pascals);

    }

    /// this function returns the minimum pressure change within
    /// a pressure vector
    #[inline]
    fn obtain_average_pressure_from_vector(
        pressure_vector: &Vec<Pressure>) -> Pressure {

        // let's get an f64 vector from the pressure vector

        let mut f64_vector: Vec<f64> =vec![];

        f64_vector.resize(
            pressure_vector.len(), 
            0.0);

        for (index,pressure_obj_pointer) in 
            pressure_vector.iter().enumerate() {

                f64_vector[index] = 
                    pressure_obj_pointer.value;

            }
        
        // now we have obtained a f64 vector from the pressure vector,
        // we can then use the max values from it
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max

        let sum_pressure_value_pascals: f64 = 
            f64_vector
            .into_iter()
            .sum();

        let average_pressure_value_pascals = 
            sum_pressure_value_pascals
            /pressure_vector.len().to_f64().unwrap();




        return Pressure::new::<pascal>(average_pressure_value_pascals);

    }

}
