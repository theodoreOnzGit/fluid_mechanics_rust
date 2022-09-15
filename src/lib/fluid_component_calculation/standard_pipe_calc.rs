// using crate brings in the lib
// since i have to manually import files from above directories
use crate::churchill_friction_factor;
use crate::dimensionalisation;

use uom::si::f64::*;
pub struct CalcPressureLoss {}
impl CalcPressureLoss {
    // this calculates pressure loss in a pipe from mass flowrate
    #[allow(non_snake_case)]
    pub fn from_mass_rate(mut fluidMassFlowrate: MassRate,
                          crossSectionalArea: Area,
                          hydraulicDiameter: Length,
                          fluidViscosity: DynamicViscosity,
                          fluidDensity: MassDensity,
                          pipeLength: Length,
                          roughnessRatio: f64,
                          formLossK: f64) -> Pressure {

        // let's first initiate the nondimensionalPipeObj
        // and also our objects to nondimensionalise fluid mass flowrate

        // now before i calculate Re, i want to make sure that
        // reverse flow is accounted for
        // this is true when fluidMassFlowrate is less than 0
        let mut reverseFlow = false;
        if fluidMassFlowrate.value < 0.0 {
            reverseFlow = true;
        }

        // so if i have reverse flow, i will make the fluidMassFlowrate
        // positive
        // and return the negative value
        // of pressureLoss
        if reverseFlow {
            fluidMassFlowrate = fluidMassFlowrate * -1.0;
        }

        // and let's get the Re and L/D
        let Re = dimensionalisation::CalcReynolds::from_mass_rate(
            fluidMassFlowrate,
            crossSectionalArea,
            hydraulicDiameter,
            fluidViscosity);

        let lengthToDiameterRatio 
            = dimensionalisation::convert_dimensionless_number_to_float(
                pipeLength/hydraulicDiameter);

        // then let's obtain the pipe Bejan Number
        // given the Re

        let Be = churchill_friction_factor::getBe(
            Re,
            roughnessRatio,
            lengthToDiameterRatio,
            formLossK);

        // once we get Be, we can get the pressure loss terms
        //
        let pressureLoss = dimensionalisation::CalcBejan::to_pressure(
            Be,
            hydraulicDiameter,
            fluidDensity,
            fluidViscosity);


        // now before i exit, i want to make sure reverse flow is taken care
        // of
        if reverseFlow {
            return pressureLoss * -1.0;
        }

        return pressureLoss;
    }

    // this calculates mass flowrate in a pipe given pressure losses
    #[allow(non_snake_case)]
    pub fn to_mass_rate(pressureLoss: Pressure,
                        crossSectionalArea: Area,
                        hydraulicDiameter: Length,
                        fluidViscosity: DynamicViscosity,
                        fluidDensity: MassDensity,
                        pipeLength: Length,
                        roughnessRatio: f64,
                        formLossK: f64) -> MassRate {
        // let's first initiate the nondimensionalPipeObj
        // and also our objects to nondimensionalise fluid mass flowrate
        // and let's get the Be_D and L/D


        let lengthToDiameterRatio 
            = dimensionalisation::convert_dimensionless_number_to_float(
                pipeLength/hydraulicDiameter);

        let Be_D = dimensionalisation::CalcBejan::from_pressure(
            pressureLoss, hydraulicDiameter, 
            fluidDensity, fluidViscosity);

        // let's get Re
        let Re_D = churchill_friction_factor::getRe(Be_D,
                                                   roughnessRatio,
                                                   lengthToDiameterRatio,
                                                   formLossK);


        // and finally return mass flowrate
        //
        let fluidMassFlowrate = 
            dimensionalisation::CalcReynolds::to_mass_rate(crossSectionalArea,
                                                           Re_D,
                                                           hydraulicDiameter,
                                                           fluidViscosity);

        return fluidMassFlowrate;

    }
}


