use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

#[allow(non_snake_case)]
pub fn getDowthermADensity(
    fluidTemp: ThermodynamicTemperature) -> MassDensity {

    // first we check if fluid temp is between 20-180C (range of validity)
    // panic otherwise
    rangeCheck(fluidTemp);

    //then convert the fluidTemp object into a f64
    // and plug it into the correlation
    let densityValueKgPerM3 = 1078.0 - 0.85*fluidTemp
       .get::<degree_celsius>();

    return MassDensity::new::<kilogram_per_cubic_meter>(densityValueKgPerM3);
}

#[allow(non_snake_case)]
pub fn  getDowthermAViscosity(
    fluidTemp: ThermodynamicTemperature) -> DynamicViscosity{

    rangeCheck(fluidTemp);
    let temperatureDegreesCValue = fluidTemp.get::<degree_celsius>();
    let viscosityValuePascalSecond = 0.130/
        temperatureDegreesCValue.powf(1.072);

    return DynamicViscosity::new::<pascal_second>(viscosityValuePascalSecond);
                                
}

#[allow(non_snake_case)]
pub fn getDowthermAConstantPressureSpecificHeatCapacity(
    fluidTemp: ThermodynamicTemperature) -> SpecificHeatCapacity{

    rangeCheck(fluidTemp);
    // note, specific entropy and heat capcity are the same unit...
    //
    let cp_value_joule_per_kg = 1518.0 + 2.82*fluidTemp.get::<degree_celsius>();

    return SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg);
}

#[allow(non_snake_case)]
pub fn getDowthermAThermalConductivity(
    fluidTemp: ThermodynamicTemperature) -> ThermalConductivity {


    rangeCheck(fluidTemp);
    let thermalConductivityValue = 0.142 - 0.00016* fluidTemp
        .get::<degree_celsius>();

    return ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermalConductivityValue);
}

// i also have an analytically integrated function for enthalpy of 
// dowtherm A
// the thing is that with enthalpy
// we need a reference value
// i take mine to be 0 enthalpy at 20C
// integrating heat capacity with respect to T, we get
//
// cp = 1518 + 2.82*T
//
// H = 1518*T + 2.82/2.0*T^2 + C
// at T = 20C, 
// H = 30924 + C
// H = 0
// C = -30924 (i used libre office to calculate this)
#[allow(non_snake_case)]
pub fn getDowthermAEnthalpy(
    fluidTemp: ThermodynamicTemperature) -> AvailableEnergy{

    rangeCheck(fluidTemp);
    // note, specific entropy and heat capcity are the same unit...
    //
    // H = 1518*T + 2.82/2.0*T^2 - 30924
    let tempCValue = fluidTemp.get::<degree_celsius>();
    let enthalpy_value_joule_per_kg 
        = 1518.0 * tempCValue 
        + 2.82/2.0 * tempCValue.powf(2.0) -
        30924.0;

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    return AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg);
}

// this functions enables us to get temperature from enthalpy using
// a root finding method
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature {

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
        let rhs = getDowthermAEnthalpy(fluid_temperature);
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


// function checks if a fluid temperature falls in a range (20-180C)
// it is assumed that temperature here is in degrees C
// to avoid units, use the overload above.
#[allow(non_snake_case)]
pub fn rangeCheck(fluidTemp: ThermodynamicTemperature) -> bool{

    // first i convert the fluidTemp object into a degree 
    // celsius
    let tempvalueCelsius = 
        fluidTemp.get::<degree_celsius>();

    if tempvalueCelsius < 20.0 {
        let errorMsg = "Your fluid temperature \n";
        let errorMsg1 = "is too low :";
        let errorMsg3 = "C \n";
        let errorMsg4 = "\n the minimum is 20C";


        panic!("{}{}{:?}{}{}",
               errorMsg,
               errorMsg1,
               fluidTemp,
               errorMsg3,
               errorMsg4);
    }


    if tempvalueCelsius > 180.0 {
        let errorMsg = "Your fluid temperature \n";
        let errorMsg1 = "is too high :";
        let errorMsg3 = "C \n";
        let errorMsg4 = "\n the max is 180C";

        panic!("{}{}{:?}{}{}",
               errorMsg,
               errorMsg1,
               fluidTemp,
               errorMsg3,
               errorMsg4);
    }

    return true;

}
