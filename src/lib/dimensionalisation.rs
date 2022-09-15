extern crate peroxide;
extern crate uom;

use uom::si::mass_rate::kilogram_per_second;
use uom::si::area::square_meter;
use uom::si::length::{meter,centimeter,foot,inch};
use uom::si::f64::*;
use uom::typenum::P2;

// uom stands for unit of measure.
//
// this set of functions here is simply to convert to and from
// dimensionless numbers to SI units, and back

pub struct CalcReynolds {}
impl CalcReynolds {
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

        return reynolds_number.value.into();

    }


    #[allow(non_snake_case)]
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


        return reynolds_number.value.into();
    }

    
    #[allow(non_snake_case)]
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

struct CalcBejan {}
impl CalcBejan {

    #[allow(non_snake_case)]
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

        return Be.value.into();
    }

    #[allow(non_snake_case)]
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
