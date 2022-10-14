// Note: //! indicates crate level documentation
//
//! A Library which calculates pressure losses to and from fluid
//! flowrates. 
//!
//! For pipe flow, the library makes use of Churchill's Friction
//! Factor Correlation in order to calculate darcy or fanning
//! friction factor for laminar, turbulent and transitional flow.
#![warn(missing_docs)]
extern crate uom;
mod churchill_friction_factor;
mod custom_fldk;
mod dimensionalisation;
pub mod fluid_component_calculation;
pub mod therminol_component;

use uom::si::f64::*;


/// This function calculates darcy friction factor
/// It takes in a Reynold's number and roughness ratio
///
/// and gives the darcy friction factor for laminar 
/// turbulent, and transition regimes. 
///
/// However, Re = 0 will not work!
/// ```rust
/// let darcy_friction_factor = 
///     fluid_mechanics_rust::darcy(1800.0,0.0015);
///
/// println!("{}", darcy_friction_factor);
/// ```
#[allow(non_snake_case)]
pub fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        darcy(ReynoldsNumber, roughnessRatio);
}

/// This function calculates moody friction factor
/// It takes in a Reynold's number and roughness ratio
///
/// and gives the darcy friction factor for laminar 
/// turbulent, and transition regimes. 
///
/// It's basically the same as darcy friction factor
///
/// However, Re = 0 will not work!
/// ```rust
/// let moody_friction_factor = 
///     fluid_mechanics_rust::moody(1800.0,0.0015);
///
/// println!("{}", moody_friction_factor);
/// ```
#[allow(non_snake_case)]
pub fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        moody(ReynoldsNumber, roughnessRatio);
}

/// This function calculates the fldk
///
/// this is the
///
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Re
/// and roughness ratio
///
/// However, Re = 0 will not work!
/// ```rust
///    let fldk = 
///        fluid_mechanics_rust::fldk(
///            15000.0,0.00014,10.0,5.0);
///
///    println!("{}", fldk);
/// ```
#[allow(non_snake_case)]
pub fn fldk(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64{
    return churchill_friction_factor::
        fLDK(ReynoldsNumber,
             roughnessRatio,
             lengthToDiameterRatio,
             K);
}


/// This function calculates the bejan number
///
/// this is the
///
///
/// Be = (P * D^2)/(mu * nu)
/// 
/// P is pressure loss
/// D is hydraulic diameter
/// mu is dynamic viscosity
/// nu is kinematic viscosity
///
/// Be is the bejan number which is dimensionless
///
/// It is calculated using:
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Re
/// and roughness ratio
///
/// Re = 0  and Re < 0 is supported,
/// this assumes that the component is symmetrical
/// in terms of pressure loss, which may usually
/// be the case for pipes anyhow
///
/// 
///
/// ```rust
/// let bejan_d = 
///     fluid_mechanics_rust::get_bejan_d(
///         0.00000000000001,0.00014,10.0,5.0);
///
/// println!("{}", bejan_d);
///
/// let bejan_d = 
///     fluid_mechanics_rust::get_bejan_d(
///         -5000.0,0.00014,10.0,5.0);
///
/// println!("{}", bejan_d);
///
/// let bejan_d = 
///     fluid_mechanics_rust::get_bejan_d(
///         0.0,0.00014,10.0,5.0);
///
/// println!("{}", bejan_d);
/// ```
#[allow(non_snake_case)]
pub fn get_bejan_d(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64 {
    return churchill_friction_factor::
        getBe(ReynoldsNumber, roughnessRatio,
              lengthToDiameterRatio, K);
}


/// This function calculates the Reynolds number given
/// a Bejan number.
///
/// Remember Bejan number is dimensionless pressure 
/// drop
///
/// Be = (P * D^2)/(mu * nu)
/// 
/// P is pressure loss
/// D is hydraulic diameter
/// mu is dynamic viscosity
/// nu is kinematic viscosity
///
/// We implicitly solve for Re using:
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Be
/// and roughness ratio
///
/// Re = 0  and Re < 0 is supported,
/// this assumes that the component is symmetrical
/// in terms of pressure loss, which may usually
/// be the case for pipes anyhow
///
/// 
/// In the following example, we get a bejan number calculated
/// first with Re = 5000.0
/// and then using that bejan number, we try and find the Re again
/// which should be about 5000.0
///
/// we use the approx package and ensure that the numbers are similar
/// to within 0.001 or 0.1% of each other
///
/// ```rust
///
/// extern crate approx;
/// let bejan_d = 
///     fluid_mechanics_rust::get_bejan_d(
///         5000.0,0.00014,10.0,5.0);
///
/// println!("{}", bejan_d);
///
/// let reynolds_number = 
///     fluid_mechanics_rust::get_reynolds_number(
///         bejan_d,0.00014,10.0,5.0);
///
/// approx::assert_relative_eq!(reynolds_number, 5000.0,
/// max_relative = 0.001);
/// ```
///
///
/// Note: why can't we just find Reynold's number from friction factor?
///
/// Note that in the laminar and turbulent region, a single Reynold's
/// number can have two different friction factor values.
/// Even in the transition region, there's probably a range of friction
/// factors where Re can have a third or fourth value
/// That's not good
///
/// Hence Reynold's number is not a function of friction factor unless
/// you restrict Re to a certain range
///
/// To get around this, we assume that pressure losses are a function
/// of Re and vice versa, 
///
/// meaning to say each pressure loss value maps to a single Re
/// and therefore dimensionless pressure losses (Be) should also
/// map to a single Re.
///
/// Therefore, we must supply a Bejan number to get an Re value.
///
#[allow(non_snake_case)]
pub fn get_reynolds_number(Be_D: f64,
             roughnessRatio: f64,
             lengthToDiameter: f64,
             formLossK: f64) -> f64 {
    return churchill_friction_factor::
        getRe(Be_D, roughnessRatio,
              lengthToDiameter, formLossK);

}


/// Custom Component is a struct or class
/// which contains functions to allow users
/// to define their own custom fluid components
/// should the regular fldk pipe structure not suffice
///
/// usually if one wants to use empirical correlations,
/// the CustomComponent struct would be suitable for this
pub struct CustomComponent {
}

#[allow(non_snake_case)]
impl CustomComponent {


    /// Here I allow users to implement custom functions for
    /// fldk. Recall:
    ///
    /// Be = 0.5 * Re^2 * (f * (L/D) + K)
    ///
    /// the f is darcy friction factor
    /// and the term in the brackets is fldk
    ///
    /// Here, I allow the user to specify the darcy
    /// friction factor using the generic function input types
    /// with any two floating point number (f64)
    ///
    /// and also to specify the form loss K with another
    /// function.
    ///
    /// The darcy friction factor will necesarily
    /// be multiplied by L/D while the 
    /// custom K will be added on into the fldk term
    ///
    /// The following example shows what happens if we want
    /// fldk = 400 + 52,000/Re
    ///
    /// In this example, we first define a custom K 
    /// and custom f function
    ///
    /// the custom f function will always return 0 since
    /// we don't want any dependence on L/D
    /// 
    /// While the custom K function will just return
    /// 400+52,000/Re
    ///
    /// Now, we must ensure that reverse flow scenarios 
    /// are properly taken care of, so there are if statements
    /// that check if Re < 0.0
    ///
    /// if so, then the negative K value is returned
    ///
    /// After that, we test whether negative values and zero
    /// values of Re are okay.
    ///
    /// When it comes to custom f and custom K values,
    /// the reverse flow logic (Re<0.0) is up to you to
    /// decide. 
    /// By default if Re = 0.0, Be = 0.0, so you needn't worry about
    /// that
    /// 
    ///
    ///```rust
    ///fn custom_k(mut reynolds_number: f64) -> f64 {
    ///    let mut reverse_flow = false;
    ///    if reynolds_number < 0.0 {
    ///        reverse_flow = true;
    ///        reynolds_number = reynolds_number * -1.0;
    ///    }
    ///    let custom_k_value =  400.0 + 52000.0/reynolds_number;
    ///    if reverse_flow == true {
    ///        return -custom_k_value;
    ///    }
    ///    return custom_k_value;
    ///}
    ///fn custom_f(_reynolds_number: f64,
    ///                 _roughness_ratio: f64) -> f64 {
    ///    return 0.0;
    ///}
    ///let custom_fldk = 
    ///    fluid_mechanics_rust::CustomComponent::fldk(
    ///        &custom_f,
    ///        -5000.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_fldk);
    ///
    ///let custom_fldk = 
    ///    fluid_mechanics_rust::CustomComponent::fldk(
    ///        &custom_f,
    ///        0.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_fldk);
    ///```
    ///
    /// 
    pub fn fldk(customDarcy: &dyn Fn(f64, f64) -> f64,
    ReynoldsNumber: f64,
    roughnessRatio: f64,
    lengthToDiameterRatio: f64,
    customK: &dyn Fn(f64) -> f64) -> f64{
    // i allow users to define their own fldk
    // basically i allow the user to define 
    // the darcy(Re, roughnessRatio) 
    // and the 
    // formLossK(Re)
    //
    // fLDK is calculated by:
    // darcy*(L/D) + K
        return custom_fldk::custom_fLDK(customDarcy,
                                        ReynoldsNumber,
                                        roughnessRatio,
                                        lengthToDiameterRatio,
                                        customK);

    }

    /// Calculate fldk based on a custom K value
    ///
    /// I included this function in case the user
    /// wanted to just specify a custom K value
    /// that depends on Re or some other factor
    /// but the friction factor was left at the darcy
    /// value
    ///
    /// Recall that fldK is defined:
    /// Be = 0.5 * Re^2 * (f * (L/D) + K)
    ///
    /// the f is darcy friction factor
    /// and the term in the brackets is fldk
    ///
    /// Here, I allow the user to specify the form 
    /// loss K with another
    /// function.
    ///
    /// For this function, Reverse flow behaviour is not 
    /// defined, nor is Re = 0.0
    ///
    /// So for Re <= 0.0, this code will panic
    ///
    ///
    ///```rust
    ///fn custom_k(mut reynolds_number: f64) -> f64 {
    ///    let mut reverse_flow = false;
    ///    if reynolds_number < 0.0 {
    ///        reverse_flow = true;
    ///        reynolds_number = reynolds_number * -1.0;
    ///    }
    ///    let custom_k_value =  400.0 + 52000.0/reynolds_number;
    ///    if reverse_flow == true {
    ///        return -custom_k_value;
    ///    }
    ///    return custom_k_value;
    ///}
    ///
    ///
    ///let custom_fldk = 
    ///    fluid_mechanics_rust::CustomComponent::fldk_pipe(
    ///        1000.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_fldk);
    ///```
    ///
    pub fn fldk_pipe(ReynoldsNumber: f64,
                         roughnessRatio: f64,
                         lengthToDiameterRatio: f64,
                         customK: &dyn Fn(f64) -> f64) -> f64{
    // if the user only wants to change K to be a custom value
    // then fldk_pipe is more appropriate

        return custom_fldk::custom_Kpipe(ReynoldsNumber,
                                         roughnessRatio,
                                         lengthToDiameterRatio,
                                         customK);
    }

    /// Calculate Bejan number based on a custom K value and
    /// standard pipe darcy friction factor correlations.
    ///
    /// I included this function in case the user
    /// wanted to just specify a custom K value
    /// that depends on Re or some other factor
    /// but the friction factor was left at the darcy
    /// value
    ///
    /// Recall that fldK is defined:
    /// Be = 0.5 * Re^2 * (f * (L/D) + K)
    ///
    /// the f is darcy friction factor
    /// and the term in the brackets is fldk
    ///
    ///
    /// Here, I allow the user to specify the form 
    /// loss K with another
    /// function.
    ///
    /// For this function, Reverse flow behaviour is not 
    /// defined, nor is Re = 0.0
    ///
    /// So for Re <= 0.0, this code will panic
    ///
    ///
    ///```rust
    ///fn custom_k(mut reynolds_number: f64) -> f64 {
    ///    let mut reverse_flow = false;
    ///    if reynolds_number < 0.0 {
    ///        reverse_flow = true;
    ///        reynolds_number = reynolds_number * -1.0;
    ///    }
    ///    let custom_k_value =  400.0 + 52000.0/reynolds_number;
    ///    if reverse_flow == true {
    ///        return -custom_k_value;
    ///    }
    ///    return custom_k_value;
    ///}
    ///
    ///
    ///let custom_bejan = 
    ///    fluid_mechanics_rust::CustomComponent::
    ///    get_bejan_custom_k_pipe(
    ///        1000.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_bejan);
    ///```
    ///
    pub fn get_bejan_custom_k_pipe( ReynoldsNumber: f64, 
                                    roughnessRatio: f64,
                                    lengthToDiameterRatio: f64,
                                    customK: &dyn Fn(f64) -> f64) -> f64{

        // now suppose we have a custom K type pipe, we can just return the
        // bejan number
        return custom_fldk::custom_Kpipe_Be_D(ReynoldsNumber,
                                              roughnessRatio,
                                              lengthToDiameterRatio,
                                              customK);

    }


    /// Calculates Bejan number based on user defined fldk
    ///
    /// Be = 0.5 * Re^2 * (f * (L/D) + K)
    ///
    /// the f is darcy friction factor
    /// and the term in the brackets is fldk
    ///
    /// Here, I allow the user to specify the darcy
    /// friction factor using the generic function input types
    /// with any two floating point number (f64)
    ///
    /// and also to specify the form loss K with another
    /// function.
    ///
    /// The darcy friction factor will necesarily
    /// be multiplied by L/D while the 
    /// custom K will be added on into the fldk term
    ///
    /// The following example shows what happens if we want
    /// fldk = 400 + 52,000/Re
    ///
    /// In this example, we first define a custom K 
    /// and custom f function
    ///
    /// the custom f function will always return 0 since
    /// we don't want any dependence on L/D
    /// 
    /// While the custom K function will just return
    /// 400+52,000/Re
    ///
    /// Now, we must ensure that reverse flow scenarios 
    /// are properly taken care of, so there are if statements
    /// that check if Re < 0.0
    ///
    /// if so, then the negative K value is returned
    ///
    /// After that, we test whether negative values and zero
    /// values of Re are okay.
    ///
    /// When it comes to custom f and custom K values,
    /// the reverse flow logic (Re<0.0) is up to you to
    /// decide. 
    /// By default if Re = 0.0, Be = 0.0, so you needn't worry about
    /// that
    /// 
    ///
    ///```rust
    ///fn custom_k(mut reynolds_number: f64) -> f64 {
    ///    let mut reverse_flow = false;
    ///    if reynolds_number < 0.0 {
    ///        reverse_flow = true;
    ///        reynolds_number = reynolds_number * -1.0;
    ///    }
    ///    let custom_k_value =  400.0 + 52000.0/reynolds_number;
    ///    if reverse_flow == true {
    ///        return -custom_k_value;
    ///    }
    ///    return custom_k_value;
    ///}
    ///fn custom_f(_reynolds_number: f64,
    ///                 _roughness_ratio: f64) -> f64 {
    ///    return 0.0;
    ///}
    ///let custom_bejan = 
    ///    fluid_mechanics_rust::CustomComponent::get_bejan_custom_fldk(
    ///        &custom_f,
    ///        -5000.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_bejan);
    ///
    ///let custom_bejan = 
    ///    fluid_mechanics_rust::CustomComponent::get_bejan_custom_fldk(
    ///        &custom_f,
    ///        0.0,
    ///        0.00014,
    ///        10.0,
    ///        &custom_k);
    ///println!("{}", custom_bejan);
    ///```
    ///
    /// 
    pub fn get_bejan_custom_fldk(customDarcy: &dyn Fn(f64, f64) -> f64,
                    ReynoldsNumber: f64,
                    roughnessRatio: f64,
                    lengthToDiameterRatio: f64,
                    customK: &dyn Fn(f64) -> f64) -> f64{

        // and now do the same for a generic fldk component of any form
        // i allow users to define their own fldk
        // basically i allow the user to define 
        // the darcy(Re, roughnessRatio) 
        // and the 
        // formLossK(Re)
        //
        // fLDK is calculated by:
        // darcy*(L/D) + K
        // the bejan number is calculated by:
        // Be_D = 0.5*fLDK*Re^2
        //
        return custom_fldk::custom_fLDK_Be_D(customDarcy,
                                             ReynoldsNumber,
                                             roughnessRatio,
                                             lengthToDiameterRatio,
                                             customK);

    }

}

/// Contains functions which Calculate Re from mass flow rate 
/// and vice versa
pub struct CalcReynolds {}
#[allow(non_snake_case)]
impl CalcReynolds {

    /// Calculates Re from mass flowrate
    ///
    /// Note that you must use the uom (units of measure)
    /// crate here. That ensures that you are calculating in
    /// a unit safe fashion
    ///
    /// In this example, i define the fluid mass flowrate, pipe diameter
    /// and viscosity
    /// the cross sectional area is calculated using
    /// A_xs = pi * D^2/4
    ///```rust
    ///
    ///use uom::si::mass_rate::kilogram_per_second;
    ///use uom::si::dynamic_viscosity::pascal_second;
    ///use uom::si::length::{meter,millimeter,foot,inch};
    ///
    ///use uom::si::f64::*;
    ///use uom::typenum::P2;
    ///
    ///let fluid_massflowrate = MassRate::new::<kilogram_per_second>(0.05);
    ///let pipe_diameter = Length::new::<meter>(2.79e-2);
    ///let pipe_xs_area = pipe_diameter.powi(P2::new())*std::f64::consts::PI/4.0;
    ///let fluid_viscosity = DynamicViscosity::new::<pascal_second>(0.001);
    ///
    ///let reynolds_number = fluid_mechanics_rust::CalcReynolds::from_mass_rate(
    ///    fluid_massflowrate,
    ///    pipe_xs_area,
    ///    pipe_diameter,
    ///    fluid_viscosity);
    ///
    ///
    ///println!("Reynolds number: {} \n", reynolds_number);
    ///```
    #[allow(non_snake_case)]
    pub fn from_mass_rate(fluidMassFlowrate: MassRate,
                        crossSectionalArea: Area,
                        hydraulic_diameter: Length,
                        fluidViscosity: DynamicViscosity) -> f64 {



        return dimensionalisation::CalcReynolds::from_mass_rate(
            fluidMassFlowrate,
            crossSectionalArea,
            hydraulic_diameter,
            fluidViscosity);
    }

    /// Calculates Re from velocity
    ///
    /// Note that you must use the uom (units of measure)
    /// crate here. That ensures that you are calculating in
    /// a unit safe fashion
    ///
    /// In this example, i define the fluid mass flowrate, pipe diameter
    /// and viscosity
    /// the cross sectional area is calculated using
    /// A_xs = pi * D^2/4
    /// 
    /// As of the time of writing this doc, Oct 2022, I didn't 
    /// implement calculating velocity from Re, as it wasn't necessary
    /// for my project
    /// 
    ///```rust
    ///
    ///use uom::si::dynamic_viscosity::pascal_second;
    ///use uom::si::length::{meter,millimeter,foot,inch};
    ///use uom::si::mass_density::kilogram_per_cubic_meter;
    ///use uom::si::velocity::meter_per_second;
    ///
    ///use uom::si::f64::*;
    ///
    ///let fluid_velocity = Velocity::new::<meter_per_second>(0.05);
    ///let pipe_diameter = Length::new::<meter>(2.79e-2);
    ///let fluid_viscosity = DynamicViscosity::new::<pascal_second>(0.001);
    ///let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    ///
    ///let reynolds_number = fluid_mechanics_rust::CalcReynolds::from_velocity(
    ///    fluid_density,
    ///    fluid_velocity,
    ///    pipe_diameter,
    ///    fluid_viscosity);
    ///
    ///
    ///println!("Reynolds number: {} \n", reynolds_number);
    ///```
    pub fn from_velocity(fluidDensity: MassDensity,
                     velocity: Velocity, 
                     hydraulic_diameter: Length,
                     fluidViscosity: DynamicViscosity) -> f64 {

        return dimensionalisation::CalcReynolds::from_velocity(
            fluidDensity,
            velocity,
            hydraulic_diameter,
            fluidViscosity);

    }

    /// Calculates mass flowrate from Re
    ///
    /// Note that you must use the uom (units of measure)
    /// crate here. That ensures that you are calculating in
    /// a unit safe fashion
    ///
    /// Here I define a reynolds number of 4000 (as f64 type)
    /// I define pipe diameter
    /// the cross sectional area is calculated using
    /// A_xs = pi * D^2/4
    ///
    /// and also define fluid viscosity
    /// and then i calculate mass flowrate from these parameters
    ///
    ///```rust
    ///
    ///use uom::si::mass_rate::kilogram_per_second;
    ///use uom::si::dynamic_viscosity::pascal_second;
    ///use uom::si::length::{meter,millimeter,foot,inch};
    ///
    ///use uom::si::f64::*;
    ///use uom::typenum::P2;
    ///
    ///let reynolds_number = 4000_f64;
    ///let pipe_diameter = Length::new::<meter>(2.79e-2);
    ///let pipe_xs_area = pipe_diameter.powi(P2::new())*std::f64::consts::PI/4.0;
    ///let fluid_viscosity = DynamicViscosity::new::<pascal_second>(0.001);
    ///
    ///
    ///println!("Reynolds number: {} \n", reynolds_number);
    ///let test_fluid_mass_flowrate = fluid_mechanics_rust::CalcReynolds::to_mass_rate(
    ///    pipe_xs_area,
    ///    reynolds_number,
    ///    pipe_diameter,
    ///    fluid_viscosity);
    ///    
    ///println!("mass flowrate: {:?} \n", test_fluid_mass_flowrate);
    ///
    ///```
    #[allow(non_snake_case)]
    pub fn to_mass_rate(crossSectionalArea: Area,
                        Re: f64,
                        hydraulicDiameter: Length,
                        fluidViscosity: DynamicViscosity) -> MassRate {

        return dimensionalisation::CalcReynolds::to_mass_rate(
            crossSectionalArea,
            Re,
            hydraulicDiameter,
            fluidViscosity);
    }
}

/// Contains functions to nondimensionalise and dimensionalise
/// pressure loss
pub struct CalcBejan {}
impl CalcBejan {


    /// Calculates Bejan number from pressure loss
    ///
    /// Bejan number is defined here as:
    /// Be = (P * D^2)/(mu * nu)
    ///
    /// But for this code, i usually take:
    /// Be = (P * D^2 * rho)/(mu * mu)
    ///
    ///
    ///
    ///
    #[allow(non_snake_case)]
    pub fn from_pressure(fluidPressure: Pressure,
              hydraulicDiameter: Length,
              fluidDensity: MassDensity,
              fluidViscosity: DynamicViscosity) -> f64 {

        return dimensionalisation::CalcBejan::from_pressure(
            fluidPressure,
            hydraulicDiameter,
            fluidDensity,
            fluidViscosity);
    }

    #[allow(non_snake_case)]
    pub fn to_pressure(Be_D: f64,
                       hydraulicDiameter: Length,
                       fluidDensity: MassDensity,
                       fluidViscosity: DynamicViscosity) -> Pressure {
        return dimensionalisation::CalcBejan::to_pressure(
            Be_D,
            hydraulicDiameter,
            fluidDensity,
            fluidViscosity);

    }


}
