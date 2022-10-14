// using crate brings in the lib
// since i have to manually import files from above directories
use crate::churchill_friction_factor;
use crate::dimensionalisation;

use uom::si::f64::*;
/// Contains functions or methods to calculate pressure loss
/// from mass flowrate or to mass flowrate
pub struct CalcPressureLoss {}
impl CalcPressureLoss {
    /// calculates pressure loss in a pipe from mass flowrate
    ///
    /// Example:
    ///
    /// ```rust
    /// extern crate uom;
    /// use uom::si::mass_rate::kilogram_per_second;
    /// use uom::si::dynamic_viscosity::pascal_second;
    /// use uom::si::length::{meter,millimeter,foot,inch};
    /// use uom::si::pressure::pascal;
    /// use uom::si::mass_density::kilogram_per_cubic_meter;
    /// use uom::si::area::square_meter;
    /// use uom::si::thermodynamic_temperature::kelvin;
    /// use uom::si::thermodynamic_temperature::degree_celsius;
    /// 
    /// use uom::si::f64::*;
    /// use uom::typenum::P2;
    ///
    /// let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.015);
    /// let cross_sectional_area= Area::new::<square_meter>(4e-5);
    /// let hydraulic_diameter= Length::new::<inch>(3.0);
    /// let fluid_viscosity= DynamicViscosity::new::<pascal_second>(0.001);
    /// let fluid_density= MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    /// let pipe_length= Length::new::<foot>(6.0);
    /// let absolute_roughness= Length::new::<millimeter>(0.001);
    /// let form_loss_k= 5.0;
    /// // first import crate for CalcPressureLoss functions
    /// use crate::fluid_mechanics_rust::
    ///     fluid_component_calculation::
    ///     standard_pipe_calc::CalcPressureLoss;
    ///
    ///
    /// let pressure_loss = CalcPressureLoss::from_mass_rate(
    ///         fluid_mass_flowrate,
    ///         cross_sectional_area,
    ///         hydraulic_diameter,
    ///         fluid_viscosity,
    ///         fluid_density,
    ///         pipe_length,
    ///         absolute_roughness,
    ///         form_loss_k);
    ///
    /// println!("pressure loss : {:?} (Pascals) ", pressure_loss);
    /// ```
    #[allow(non_snake_case)]
    pub fn from_mass_rate(mut fluidMassFlowrate: MassRate,
                          crossSectionalArea: Area,
                          hydraulicDiameter: Length,
                          fluidViscosity: DynamicViscosity,
                          fluidDensity: MassDensity,
                          pipeLength: Length,
                          absolute_roughness: Length,
                          formLossK: f64) -> Pressure {
        // first let's calculate roughness ratio

        let roughnessRatioQuantity = absolute_roughness/hydraulicDiameter;

        let roughnessRatio = 
            dimensionalisation::convert_dimensionless_number_to_float(
                roughnessRatioQuantity);

        // second i want to take care of reverse flow

        let mut reverseFlow = false;
        if fluidMassFlowrate.value < 0.0 {
            reverseFlow = true;
        }

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

    /// this calculates mass flowrate in a pipe given pressure losses
    ///
    /// Example:
    ///
    /// ```rust
    /// extern crate uom;
    /// use uom::si::mass_rate::kilogram_per_second;
    /// use uom::si::dynamic_viscosity::pascal_second;
    /// use uom::si::length::{meter,millimeter,foot,inch};
    /// use uom::si::pressure::pascal;
    /// use uom::si::mass_density::kilogram_per_cubic_meter;
    /// use uom::si::area::square_meter;
    /// use uom::si::thermodynamic_temperature::kelvin;
    /// use uom::si::thermodynamic_temperature::degree_celsius;
    /// 
    /// use uom::si::f64::*;
    /// use uom::typenum::P2;
    ///
    /// let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.015);
    /// let cross_sectional_area= Area::new::<square_meter>(4e-5);
    /// let hydraulic_diameter= Length::new::<inch>(3.0);
    /// let fluid_viscosity= DynamicViscosity::new::<pascal_second>(0.001);
    /// let fluid_density= MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    /// let pipe_length= Length::new::<foot>(6.0);
    /// let absolute_roughness= Length::new::<millimeter>(0.001);
    /// let form_loss_k= 5.0;
    ///
    /// // first import crate for CalcPressureLoss functions
    /// use crate::fluid_mechanics_rust::
    ///     fluid_component_calculation::
    ///     standard_pipe_calc::CalcPressureLoss;
    ///
    ///
    ///
    /// let pressure_loss = Pressure::new::<pascal>(500.0);
    ///
    /// println!("pressure loss : {:?} (Pascals) ", pressure_loss);
    /// let test_mass_rate = CalcPressureLoss::to_mass_rate(
    ///     pressure_loss,
    ///     cross_sectional_area,
    ///     hydraulic_diameter,
    ///     fluid_viscosity,
    ///     fluid_density,
    ///     pipe_length,
    ///     absolute_roughness,
    ///     form_loss_k);
    ///
    /// println!("reference mass flowrate : {:?}  ", fluid_mass_flowrate);
    /// ```
    #[allow(non_snake_case)]
    pub fn to_mass_rate(pressureLoss: Pressure,
                        crossSectionalArea: Area,
                        hydraulicDiameter: Length,
                        fluidViscosity: DynamicViscosity,
                        fluidDensity: MassDensity,
                        pipeLength: Length,
                        absolute_roughness: Length,
                        formLossK: f64) -> MassRate {

        // first let's get our relevant ratios:
        let roughnessRatioQuantity = absolute_roughness/hydraulicDiameter;

        let roughnessRatio = 
            dimensionalisation::convert_dimensionless_number_to_float(
                roughnessRatioQuantity);

        let lengthToDiameterRatio 
            = dimensionalisation::convert_dimensionless_number_to_float(
                pipeLength/hydraulicDiameter);

        // then get Bejan number:

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


