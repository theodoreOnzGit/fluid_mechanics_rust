use crate::fluid_thermophysical_properties::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;
extern crate peroxide;
use peroxide::prelude::*;

/// Contains thermophysical property information for 
/// Therminol VP 1 or Dowtherm A based on Zweibaum 2015 PhD dissertation
/// the correlations range from 20C to 180C
pub struct TherminolVP1Properties {

}

impl FluidProperties for TherminolVP1Properties {
    /// fluid density based on temperature,
    fn density(&self,
               fluid_temp: ThermodynamicTemperature) -> MassDensity{


        // first we check if fluid temp is between 20-180C (range of validity)
        // panic otherwise
        Self::therminol_vp_1_range_check(fluid_temp);

        //then convert the fluidTemp object into a f64
        // and plug it into the correlation
        let density_value_kg_per_m3 = 1078.0 - 0.85*fluid_temp
            .get::<degree_celsius>();

        return MassDensity::new::<kilogram_per_cubic_meter>(density_value_kg_per_m3);
    }

    /// fluid dynamic viscosity based on temperature,
    fn viscosity(&self,
                 fluid_temp: ThermodynamicTemperature) -> DynamicViscosity{


        Self::therminol_vp_1_range_check(fluid_temp);
        let temp_celsius_value = fluid_temp.get::<degree_celsius>();
        let viscosity_value_pa_s = 0.130/
            temp_celsius_value.powf(1.072);

        return DynamicViscosity::new::<pascal_second>(viscosity_value_pa_s);
    }


    /// function to obtain dowtherm A enthalpy
    /// given a temperature
    ///
    /// 
    /// This is done via analytically integrating 
    /// the function for specific heat capacity of 
    /// dowtherm A
    ///
    /// However,
    /// the thing is that with enthalpy
    /// we need a reference value
    /// i take the reference value to be 0 J/kg enthalpy at 20C
    /// integrating heat capacity with respect to T, we get
    ///
    /// cp = 1518 + 2.82*T
    ///
    /// H = 1518*T + 2.82/2.0*T^2 + C
    /// at T = 20C, 
    /// H = 30924 + C
    /// H = 0
    /// C = -30924 (i used libre office to calculate this)
    ///
    ///
    ///
    /// Example use:
    /// ```rust
    ///
    /// use uom::si::f64::*;
    /// use uom::si::thermodynamic_temperature::kelvin;
    /// use fluid_mechanics_rust::fluid_thermophysical_properties::property_library::*;
    /// use fluid_mechanics_rust::fluid_thermophysical_properties::FluidProperties;
    ///
    /// let temp1 = ThermodynamicTemperature::new::<kelvin>(303_f64);
    ///
    /// let therminol_properties = TherminolVP1Properties::new();
    ///
    /// let specific_enthalpy_1 = therminol_properties.enthalpy(temp1);
    ///
    ///
    /// let expected_enthalpy: f64 = 
    /// 1518_f64*30_f64 + 2.82/2.0*30_f64.powf(2_f64) - 30924_f64;
    ///
    /// // the expected value is about 15885 J/kg
    ///
    /// extern crate approx;
    /// approx::assert_relative_eq!(expected_enthalpy, specific_enthalpy_1.value, 
    /// max_relative=0.02);
    /// ```
    /// fluid enthalpy  based on temperature,
    fn enthalpy(&self,
                fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{


        Self::therminol_vp_1_range_check(fluid_temp);
        // note, specific entropy and heat capcity are the same unit...
        //
        // H = 1518*T + 2.82/2.0*T^2 - 30924
        let temp_celsius_value = fluid_temp.get::<degree_celsius>();
        let enthalpy_value_joule_per_kg 
            = 1518.0 * temp_celsius_value 
            + 2.82/2.0 * temp_celsius_value.powf(2.0) -
            30924.0;

        // the closest unit available is AvailableEnergy which is
        // joule per kg 

        return AvailableEnergy::new::<joule_per_kilogram>(
            enthalpy_value_joule_per_kg);
    }

    /// fluid specific heat capacity  based on temperature,
    fn specific_heat_capacity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity{

        Self::therminol_vp_1_range_check(fluid_temp);
        // note, specific entropy and heat capcity are the same unit...
        //
        let cp_value_joule_per_kg = 1518.0 + 2.82*fluid_temp.get::<degree_celsius>();

        return SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
            cp_value_joule_per_kg);
    }

    /// fluid thermal conductivity based on temperature,
    fn thermal_conductivity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity{

        Self::therminol_vp_1_range_check(fluid_temp);
        let thermal_conductivity_value = 0.142 - 0.00016* fluid_temp
            .get::<degree_celsius>();

        return ThermalConductivity::new::<watt_per_meter_kelvin>(
            thermal_conductivity_value);
    }

    /// function to obtain dowtherm A temperature 
    /// given a enthalpy
    ///
    /// 
    /// This is done via analytically integrating 
    /// the function for specific heat capacity of 
    /// dowtherm A
    ///
    /// However,
    /// the thing is that with enthalpy
    /// we need a reference value
    /// i take the reference value to be 0 J/kg enthalpy at 20C
    /// integrating heat capacity with respect to T, we get
    ///
    /// cp = 1518 + 2.82*T
    ///
    /// H = 1518*T + 2.82/2.0*T^2 + C
    /// at T = 20C, 
    /// H = 30924 + C
    /// H = 0
    /// C = -30924 (i used libre office to calculate this)
    ///
    /// Once i have this correlation, i will use
    /// an iterative root finding method to find the temperature
    ///
    /// As of Oct 2022, it is bisection
    ///
    /// Example: 
    ///
    /// ```rust
    /// use uom::si::f64::*;
    /// use uom::si::thermodynamic_temperature::kelvin;
    /// use uom::si::available_energy::joule_per_kilogram;
    /// use fluid_mechanics_rust::fluid_thermophysical_properties::*;
    ///
    ///
    /// let specific_enthalpy_1 = AvailableEnergy::new::
    /// <joule_per_kilogram>(15885.0);
    ///
    /// let temp_expected = ThermodynamicTemperature::new::
    /// <kelvin>(303_f64);
    ///
    /// let therminol_properties = TherminolVP1Properties::new();
    /// 
    /// let temp_acutal = therminol_properties.get_temperature_from_enthalpy(
    /// specific_enthalpy_1);
    ///
    ///
    /// extern crate approx;
    /// approx::assert_relative_eq!(temp_expected.value, 
    /// temp_acutal.value, 
    /// max_relative=0.01);
    ///
    ///
    /// ```
    /// fluid temperature based on fluid enthalpy
    fn get_temperature_from_enthalpy(
        &self,
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature{

        if fluid_enthalpy.value < 0_f64 {
            panic!("dowtherm A : get_temperature_from_enthalpy \n
               enthalpy < 0.0 , out of correlation range");
        }

        // first let's convert enthalpy to a double (f64)
        let enthalpy_value_joule_per_kg = 
            fluid_enthalpy.get::<joule_per_kilogram>();

        // second let's define a function 
        // or actually a closure or anonymous function that
        // is aware of the variables declared
        // enthalpy value = 1518*T +2.82/2.0 T^2 - 30924
        // LHS is actual enthalpy value

        let enthalpy_root = |temp_degrees_c_value : AD| -> AD {
            let lhs_value = enthalpy_value_joule_per_kg;
            // convert AD type into double
            let temp_degrees_c_value_double = temp_degrees_c_value.x();

            let fluid_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    temp_degrees_c_value_double);
            let rhs = self.enthalpy(fluid_temperature);
            let rhs_value = rhs.get::<joule_per_kilogram>();

            return AD0(lhs_value-rhs_value);
        };

        // now solve using bisection

        let fluid_temperature_degrees_cresult 
            = bisection(enthalpy_root,
                        (20.0,180.0),
                        100,
                        1e-8);

        let fluid_temperature_degrees_c = fluid_temperature_degrees_cresult.unwrap();

        return ThermodynamicTemperature::
            new::<degree_celsius>(fluid_temperature_degrees_c);
    }

}

impl TherminolVP1Properties {


    /// constructor

    pub fn new() -> Self {
        return Self {  };
    }
    /// the correlation has temperature range from 20 C to 180 C,
    /// anything outside this and the code throws a panic
    ///
    pub fn therminol_vp_1_range_check(fluid_temp: ThermodynamicTemperature) -> bool{

        // first i convert the fluid_temp object into a degree 
        // celsius
        let temp_value_celsius = 
            fluid_temp.get::<degree_celsius>();

        if temp_value_celsius < 20.0 {
            let error_msg = "Your fluid temperature \n";
            let error_msg1 = "is too low :";
            let error_msg3 = "C \n";
            let error_msg4 = "\n the minimum is 20C";


            panic!("{}{}{:?}{}{}",
                   error_msg,
                   error_msg1,
                   fluid_temp,
                   error_msg3,
                   error_msg4);
        }


        if temp_value_celsius > 180.0 {
            let error_msg = "Your fluid temperature \n";
            let error_msg1 = "is too high :";
            let error_msg3 = "C \n";
            let error_msg4 = "\n the max is 180C";

            panic!("{}{}{:?}{}{}",
                   error_msg,
                   error_msg1,
                   fluid_temp,
                   error_msg3,
                   error_msg4);
        }

        return true;

    }
}
