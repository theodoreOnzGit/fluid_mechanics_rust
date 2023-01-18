#![warn(missing_docs)]
extern crate peroxide;
extern crate uom;

use uom::si::f64::*;
use uom::typenum::P2;

// uom stands for unit of measure.
//
// this set of functions here is simply to convert to and from
// dimensionless numbers to SI units, and back

/// first and most important function here,
/// this function converts a quantity into float given the correct dimensions
/// if it's not dimensionless, it will throw a compile time error
pub fn convert_dimensionless_number_to_float(dimensionless_number: Ratio) -> f64 {
    return dimensionless_number.value.into();
}


/// struct which contains associated functions to calculate Re
///
/// I might want to turn this into a trait or make a trait for this 
/// in future
pub struct CalcReynolds {}

impl CalcReynolds {
    /// calculates Re = rho * U * D /mu
    #[allow(non_snake_case)]
    pub fn from_velocity(fluidDensity: MassDensity,
                     velocity: Velocity, 
                     hydraulic_diameter: Length,
                     fluidViscosity: DynamicViscosity) -> f64 {

        if fluidViscosity.value <= 0.0 {
            panic!("fluid Viscosity <= 0.0, nonphysical");
        }

        if hydraulic_diameter.value <= 0.0 {
            panic!("hydraulic Diameter <= 0.0, nonphysical");
        }
        if fluidDensity.value <= 0.0 {
            panic!("fluidDensity <= 0.0, nonphysical");
        }

        let reynolds_number = 
            fluidDensity * 
            velocity * 
            hydraulic_diameter / 
            fluidViscosity;



        return convert_dimensionless_number_to_float(reynolds_number);

    }


    #[allow(non_snake_case)]
    /// calculates Re = mass_flow/area * D_H/mu
    pub fn from_mass_rate(fluidMassFlowrate: MassRate,
                        crossSectionalArea: Area,
                        hydraulic_diameter: Length,
                        fluidViscosity: DynamicViscosity) -> f64 {

        if fluidViscosity.value <= 0.0 {
            panic!("fluid Viscosity <= 0.0, nonphysical");
        }

        if hydraulic_diameter.value <= 0.0 {
            panic!("hydraulic Diameter <= 0.0, nonphysical");
        }
        if crossSectionalArea.value <= 0.0 {
            panic!("pipe Area <= 0.0, nonphysical");
        }

        let reynolds_number = fluidMassFlowrate/
            crossSectionalArea*
            hydraulic_diameter/
            fluidViscosity;


        return convert_dimensionless_number_to_float(reynolds_number);
    }

    
    #[allow(non_snake_case)]
    /// converts Re to mass flowrate using
    /// Re = mass_flow/area * D_H/mu
    pub fn to_mass_rate(crossSectionalArea: Area,
                        Re: f64,
                        hydraulicDiameter: Length,
                        fluidViscosity: DynamicViscosity) -> MassRate {

        if fluidViscosity.value <= 0.0 {
            panic!("fluid Viscosity <= 0.0, nonphysical");
        }

        if hydraulicDiameter.value <= 0.0 {
            panic!("hydraulic Diameter <= 0.0, nonphysical");
        }

        if crossSectionalArea.value <= 0.0 {
            panic!("pipe Area <= 0.0, nonphysical");
        }

        let fluidMassFlowrate = fluidViscosity*
            crossSectionalArea/
            hydraulicDiameter*
            Re;

        return fluidMassFlowrate;
    }
}

/// contains assoc functions which help calculate bejan
/// number,
///
/// in our context, it is a form of dimensionless pressure
///
/// i might want to make traits out of this in future
pub struct CalcBejan {}
impl CalcBejan {

    #[allow(non_snake_case)]
    /// calculates Bejan number from pressure
    ///
    /// Be_D = Delta P * rho * D_H^2 / mu^2
    pub fn from_pressure(fluidPressure: Pressure,
              hydraulicDiameter: Length,
              fluidDensity: MassDensity,
              fluidViscosity: DynamicViscosity) -> f64 {


        if fluidViscosity.value <= 0.0 {
            panic!("fluid Viscosity <= 0.0, nonphysical");
        }

        if hydraulicDiameter.value <= 0.0 {
            panic!("hydraulic Diameter <= 0.0, nonphysical");
        }

        if fluidDensity.value <= 0.0 {
            panic!("fluidDensity <= 0.0, nonphysical");
        }

        let Be = fluidPressure*
            fluidDensity *
            hydraulicDiameter.powi(P2::new())/
            fluidViscosity.powi(P2::new());

        return convert_dimensionless_number_to_float(Be);
    }

    #[allow(non_snake_case)]
    /// converts Bejan number to pressure
    /// using:
    ///
    /// Be_D = Delta P * rho * D_H^2 / mu^2
    pub fn to_pressure(Be_D: f64,
                       hydraulicDiameter: Length,
                       fluidDensity: MassDensity,
                       fluidViscosity: DynamicViscosity) -> Pressure {


        if fluidViscosity.value <= 0.0 {
            panic!("fluid Viscosity <= 0.0, nonphysical");
        }

        if hydraulicDiameter.value <= 0.0 {
            panic!("hydraulic Diameter <= 0.0, nonphysical");
        }

        if fluidDensity.value <= 0.0 {
            panic!("fluidDensity <= 0.0, nonphysical");
        }

        let fluidPressure = fluidViscosity.powi(P2::new())*
                        Be_D/
                        hydraulicDiameter.powi(P2::new())/
                        fluidDensity;

        return fluidPressure;
    }


}
