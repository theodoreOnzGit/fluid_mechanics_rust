extern crate peroxide;
extern crate uom;

use uom::si::mass_rate::kilogram_per_second;
use uom::si::area::square_meter;
use uom::si::length::{meter,centimeter,foot,inch};
use uom::si::f64::*;
// uom stands for unit of measure.
//
// this set of functions here is simply to convert to and from
// dimensionless numbers to SI units, and back

struct CalcReynolds {}
impl CalcReynolds {
    #[allow(non_snake_case)]
    pub fn from_velocity(rho: MassDensity,
                     velocity: Velocity, 
                     hydraulic_diameter: Length,
                     mu: DynamicViscosity) -> f64 {

        let reynolds_number = 
            rho * 
            velocity * 
            hydraulic_diameter / 
            mu;

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
}
