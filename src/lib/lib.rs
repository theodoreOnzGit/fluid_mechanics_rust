extern crate uom;
mod churchill_friction_factor;
mod custom_fldk;
mod dimensionalisation;
pub mod fluid_component_calculation;

use uom::si::f64::*;

#[allow(non_snake_case)]
pub fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        darcy(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
pub fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        moody(ReynoldsNumber, roughnessRatio);
}

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


#[allow(non_snake_case)]
pub fn get_bejan_d(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64 {
    return churchill_friction_factor::
        getBe(ReynoldsNumber, roughnessRatio,
              lengthToDiameterRatio, K);
}

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
