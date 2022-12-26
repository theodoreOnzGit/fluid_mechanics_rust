// using crate brings in the lib
// since i have to manually import files from above directories
use crate::custom_fldk;
use crate::dimensionalisation;

use uom::si::f64::*;
use uom::si::acceleration::meter_per_second_squared;

use super::FluidComponent;

pub trait FluidCustomComponentCalcPressureChange :
FluidCustomComponentCalcPressureLoss + FluidComponent{

    fn fluid_custom_component_calc_pressure_change(
        &mut self,
        fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        absolute_roughness: Length,
        incline_angle: Angle,
        source_pressure: Pressure,
        custom_darcy: &dyn Fn(f64, f64) -> f64,
        custom_k: &dyn Fn(f64) -> f64) -> Pressure {

        unimplemented!();
    }

    fn fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
        &mut self,
        pressure_loss: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        absolute_roughness: Length,
        incline_angle: Angle,
        source_pressure: Pressure,
        custom_darcy: &dyn Fn(f64, f64) -> f64,
        custom_k: &dyn Fn(f64) -> f64) -> MassRate {


        unimplemented!();
    }

}

/// provides generic methods to calculate pressure
/// loss for a custom fluid component (with flow flowing
/// inside it)
/// given a custom darcy friction factor and
/// custom form loss correlation
pub trait FluidCustomComponentCalcPressureLoss {


    /// calculates pressure loss for a component given 
    /// pipe parameter inputs and
    /// custom darcy friction factor and custom form loss
    /// correlations
    fn fluid_custom_component_calc_pressure_loss(
        &mut self,
        fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        absolute_roughness: Length,
        custom_darcy: &dyn Fn(f64, f64) -> f64,
        custom_k: &dyn Fn(f64) -> f64) -> Pressure {

        // first we get our Reynolds number

        let reynolds_number_quantity_object = fluid_mass_flowrate/
            cross_sectional_area*
            hydraulic_diameter/
            fluid_viscosity;

        let reynolds_number_calculated_using_diameter : f64 = 
            dimensionalisation::convert_dimensionless_number_to_float(
                reynolds_number_quantity_object);

        // second we get the darcy factor and custom K
        // note that reverse flow logic should be taken care of in
        // user supplied darcy factor and K, not here

        let roughness_ratio_quantity_object = absolute_roughness/hydraulic_diameter;
        let roughness_ratio : f64 = 
            dimensionalisation::convert_dimensionless_number_to_float(
                roughness_ratio_quantity_object);

        let length_to_diameter_quantity_object = 
            component_length/
            hydraulic_diameter;

        let length_to_diameter = 
            dimensionalisation::convert_dimensionless_number_to_float(
                length_to_diameter_quantity_object);


        // now we have this, we can calculate bejan number

        let bejan_number_calculated_using_diameter = custom_fldk::custom_fLDK_Be_D(
            custom_darcy,
            reynolds_number_calculated_using_diameter,
            roughness_ratio,
            length_to_diameter,
            custom_k);


        // once we get Be, we can get the pressure loss terms
        //
        let pressure_loss = dimensionalisation::CalcBejan::to_pressure(
            bejan_number_calculated_using_diameter,
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity);



        return pressure_loss;
    }

    /// calculates mass flowrate using input parameters
    fn fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
        &mut self,
        pressure_loss: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        absolute_roughness: Length,
        custom_darcy: &dyn Fn(f64, f64) -> f64,
        custom_k: &dyn Fn(f64) -> f64) -> MassRate {


        // first let's get our relevant ratios:
        let roughness_ratio_quantity = absolute_roughness/hydraulic_diameter;

        let roughness_ratio = 
            dimensionalisation::convert_dimensionless_number_to_float(
                roughness_ratio_quantity);

        let length_to_diameter_ratio 
            = dimensionalisation::convert_dimensionless_number_to_float(
                component_length/hydraulic_diameter);

        // then get Bejan number:
        let bejan_number_calculated_using_diameter = 
            dimensionalisation::CalcBejan::from_pressure(
            pressure_loss, 
            hydraulic_diameter, 
            fluid_density, 
            fluid_viscosity);

        // let's get Re
        let reynolds_number_calculated_using_diameter = 
            custom_fldk::getRe(custom_darcy,
                               bejan_number_calculated_using_diameter,
                               roughness_ratio,
                               length_to_diameter_ratio,
                               custom_k);


        // and finally return mass flowrate
        //
        let fluid_mass_flowrate = 
            dimensionalisation::CalcReynolds::to_mass_rate(
                cross_sectional_area,
                reynolds_number_calculated_using_diameter,
                hydraulic_diameter,
                fluid_viscosity);

        return fluid_mass_flowrate;
    }
}


/// Contains functions to calculate pressure loss from
/// mass flowrate or to mass flowrate
/// for user specified components with custom fldk
pub struct CalcPressureLoss {}
impl CalcPressureLoss {
    // this calculates pressure loss in a pipe from mass flowrate
    #[allow(non_snake_case)]
    /// calculates pressure loss in a user specified
    /// component from mass flowrate
    ///
    /// Note that it is the user's responsibility to
    /// code in the behaviour for reverse flow
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
    /// // here are our custom f and custom k functions
    ///
    /// fn custom_k(mut reynolds_number: f64) -> f64 {
    ///
    ///     let mut reverse_flow = false;
    ///     if reynolds_number < 0.0 {
    ///         reverse_flow = true;
    ///         reynolds_number = reynolds_number * -1.0;
    ///     }
    ///     let fldk =  400.0 + 52000.0/reynolds_number;
    ///
    ///     if reverse_flow == true {
    ///         return -fldk;
    ///     }
    ///     return fldk;
    /// }
    ///
    /// fn custom_f(_reynolds_number: f64,
    ///                  _roughness_ratio: f64) -> f64 {
    ///     return 0.0;
    /// }
    ///
    /// // first import crate for CalcPressureLoss functions
    /// use crate::fluid_mechanics_rust::
    ///     fluid_component_calculation::
    ///     custom_component_calc::CalcPressureLoss;
    ///
    ///
    /// let pressure_loss = 
    ///     CalcPressureLoss::from_mass_rate(fluid_mass_flowrate,
    ///                                      cross_sectional_area,
    ///                                      hydraulic_diameter,
    ///                                      fluid_viscosity,
    ///                                      fluid_density,
    ///                                      pipe_length,
    ///                                      absolute_roughness,
    ///                                      &custom_f,
    ///                                      &custom_k);
    ///
    ///
    /// 
    /// println!("pressure loss calculated as {:?}", pressure_loss);
    ///
    /// ```
    pub fn from_mass_rate(fluidMassFlowrate: MassRate,
                          crossSectionalArea: Area,
                          hydraulicDiameter: Length,
                          fluidViscosity: DynamicViscosity,
                          fluidDensity: MassDensity,
                          pipeLength: Length,
                          absolute_roughness: Length,
                          customDarcy: &dyn Fn(f64, f64) -> f64,
                          customK: &dyn Fn(f64) -> f64) -> Pressure {

        // first we get our Reynolds number

        let ReQuantity = fluidMassFlowrate/
            crossSectionalArea*
            hydraulicDiameter/
            fluidViscosity;

        let Re = dimensionalisation::convert_dimensionless_number_to_float(
            ReQuantity);

        // second we get the darcy factor and custom K
        // note that reverse flow logic should be taken care of in
        // user supplied darcy factor and K, not here

        let roughnessRatioQuantity = absolute_roughness/hydraulicDiameter;
        let roughnessRatio = 
            dimensionalisation::convert_dimensionless_number_to_float(
            roughnessRatioQuantity);

        let lengthToDiameterQuantity = 
            pipeLength/
            hydraulicDiameter;

        let lengthToDiameter = 
            dimensionalisation::convert_dimensionless_number_to_float(
                lengthToDiameterQuantity);


        // now we have this, we can calculate bejan number

        let Be_D = custom_fldk::custom_fLDK_Be_D(
            customDarcy,
            Re,
            roughnessRatio,
            lengthToDiameter,
            customK);


        // once we get Be, we can get the pressure loss terms
        //
        let pressureLoss = dimensionalisation::CalcBejan::to_pressure(
            Be_D,
            hydraulicDiameter,
            fluidDensity,
            fluidViscosity);



        return pressureLoss;
    }

    /// calculates mass flowrate in a user specified 
    /// component from pressure loss
    ///
    /// Note that it is the user's responsibility to
    /// code in the behaviour for reverse flow
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
    /// // here are our custom f and custom k functions
    ///
    /// fn custom_k(mut reynolds_number: f64) -> f64 {
    ///
    ///     let mut reverse_flow = false;
    ///     if reynolds_number < 0.0 {
    ///         reverse_flow = true;
    ///         reynolds_number = reynolds_number * -1.0;
    ///     }
    ///     let fldk =  400.0 + 52000.0/reynolds_number;
    ///
    ///     if reverse_flow == true {
    ///         return -fldk;
    ///     }
    ///     return fldk;
    /// }
    ///
    /// fn custom_f(_reynolds_number: f64,
    ///                  _roughness_ratio: f64) -> f64 {
    ///     return 0.0;
    /// }
    ///
    /// // first import crate for CalcPressureLoss functions
    /// use crate::fluid_mechanics_rust::
    ///     fluid_component_calculation::
    ///     custom_component_calc::CalcPressureLoss;
    ///
    ///
    /// let pressure_loss = Pressure::new::<pascal>(500.0);
    ///
    ///
    /// let mass_rate = CalcPressureLoss::to_mass_rate(pressure_loss,
    ///                                      cross_sectional_area,
    ///                                      hydraulic_diameter,
    ///                                      fluid_viscosity,
    ///                                      fluid_density,
    ///                                      pipe_length,
    ///                                      absolute_roughness,
    ///                                      &custom_f,
    ///                                      &custom_k);
    /// 
    /// 
    /// println!("mass rate calculated as {:?}", mass_rate);
    ///
    /// ```
    #[allow(non_snake_case)]
    pub fn to_mass_rate(pressureLoss: Pressure,
                        crossSectionalArea: Area,
                        hydraulicDiameter: Length,
                        fluidViscosity: DynamicViscosity,
                        fluidDensity: MassDensity,
                        pipeLength: Length,
                        absolute_roughness: Length,
                        customDarcy: &dyn Fn(f64, f64) -> f64,
                        customK: &dyn Fn(f64) -> f64) -> MassRate {


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
        let Re_D = custom_fldk::getRe(customDarcy,
                                      Be_D,
                                      roughnessRatio,
                                      lengthToDiameterRatio,
                                      customK);


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


