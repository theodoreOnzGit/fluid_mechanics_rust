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
#[allow(non_snake_case)]
pub fn get_reynolds_number(Be_D: f64,
             roughnessRatio: f64,
             lengthToDiameter: f64,
             formLossK: f64) -> f64 {
    return churchill_friction_factor::
        getRe(Be_D, roughnessRatio,
              lengthToDiameter, formLossK);

}

pub struct CustomComponent {
}

#[allow(non_snake_case)]
impl CustomComponent {

    // i allow users to define their own fldk
    // basically i allow the user to define 
    // the darcy(Re, roughnessRatio) 
    // and the 
    // formLossK(Re)
    //
    // fLDK is calculated by:
    // darcy*(L/D) + K
    pub fn fldk(customDarcy: &dyn Fn(f64, f64) -> f64,
    ReynoldsNumber: f64,
    roughnessRatio: f64,
    lengthToDiameterRatio: f64,
    customK: &dyn Fn(f64) -> f64) -> f64{
        return custom_fldk::custom_fLDK(customDarcy,
                                        ReynoldsNumber,
                                        roughnessRatio,
                                        lengthToDiameterRatio,
                                        customK);

    }

    // if the user only wants to change K to be a custom value
    // then fldk_pipe is more appropriate
    pub fn fldk_pipe(ReynoldsNumber: f64,
                         roughnessRatio: f64,
                         lengthToDiameterRatio: f64,
                         customK: &dyn Fn(f64) -> f64) -> f64{

        return custom_fldk::custom_Kpipe(ReynoldsNumber,
                                         roughnessRatio,
                                         lengthToDiameterRatio,
                                         customK);
    }

    // now suppose we have a custom K type pipe, we can just return the
    // bejan number
    pub fn get_bejan_custom_k_pipe( ReynoldsNumber: f64, 
                                    roughnessRatio: f64,
                                    lengthToDiameterRatio: f64,
                                    customK: &dyn Fn(f64) -> f64) -> f64{

        return custom_fldk::custom_Kpipe_Be_D(ReynoldsNumber,
                                              roughnessRatio,
                                              lengthToDiameterRatio,
                                              customK);

    }

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

    pub fn get_bejan_custom_fldk(customDarcy: &dyn Fn(f64, f64) -> f64,
                    ReynoldsNumber: f64,
                    roughnessRatio: f64,
                    lengthToDiameterRatio: f64,
                    customK: &dyn Fn(f64) -> f64) -> f64{

        return custom_fldk::custom_fLDK_Be_D(customDarcy,
                                             ReynoldsNumber,
                                             roughnessRatio,
                                             lengthToDiameterRatio,
                                             customK);

    }

}

pub struct CalcReynolds {}
#[allow(non_snake_case)]
impl CalcReynolds {

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

pub struct CalcBejan {}
impl CalcBejan {


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
