use uom::si::f64::*;

#[test]
pub fn my_test(){
    // now for rust , we don't have assert equal
    // showing expected and test values
    // we just see if left == right
    // not like C#,
    // where left is expected value,
    // right is asserted value
    //
    assert_eq!(2.0,2.0);
}

// now let's import a test for CTAH
// CTAH has a characteristic pressure loss
// which is measured by M-44 and M-45
//
// at 20C the experimental data is provided for us
//
#[test]
pub fn when_ctah_pressure_change_expect_correct_value_zero_flow(){
    //import necessary things...
    use fluid_mechanics_rust;
    use fluid_mechanics_rust::therminol_component::factory;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::f64::*;

    let expected_pressure_loss = 0.0;

    // let's get the component for ctah
    let ctah_vertical = factory::CTAHVertical::get();
    let ctah_horizontal = factory::CTAHHorizontal::get();

    // now let's have a temperature of 21C and mass flow of 0.15 kg/s
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(20.0);
    let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.0);

    // let's get the pressure changes
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;


    let ctah_vertical_pressure_change = 
        CalcPressureChange::from_mass_rate(
            &ctah_vertical,
            mass_flow_expected,
            fluid_temp);

    let ctah_horizontal_pressure_change = 
        CalcPressureChange::from_mass_rate(
            &ctah_horizontal,
            mass_flow_expected,
            fluid_temp);

    // let's subtract out the pressure change due to hydrostatic pressure
    //
    use fluid_mechanics_rust::therminol_component::
        StandardCustomComponentProperties;

    let ctah_hydrostatic_pressure_change = 
        ctah_vertical.get_hydrostatic_pressure_change(
            fluid_temp) +
        ctah_horizontal.get_hydrostatic_pressure_change(
            fluid_temp);



    let actual_pressure_loss = ctah_vertical_pressure_change.value +
        ctah_horizontal_pressure_change.value -
        ctah_hydrostatic_pressure_change.value;

    assert_eq!(expected_pressure_loss,actual_pressure_loss);
}

#[test]
pub fn when_ctah_pressure_change_expect_correct_value_non_zero_flow(){
    //import necessary things...
    use fluid_mechanics_rust;
    use fluid_mechanics_rust::therminol_component::factory;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::f64::*;


    // let's get the component for ctah
    let ctah_vertical = factory::CTAHVertical::get();
    let ctah_horizontal = factory::CTAHHorizontal::get();

    // now let's have a temperature of 21C and mass flow of 0.15 kg/s
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(20.0);
    let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.18);

    // let's get the pressure changes
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;


    let ctah_vertical_pressure_change = 
        CalcPressureChange::from_mass_rate(
            &ctah_vertical,
            mass_flow_expected,
            fluid_temp);

    let ctah_horizontal_pressure_change = 
        CalcPressureChange::from_mass_rate(
            &ctah_horizontal,
            mass_flow_expected,
            fluid_temp);

    // let's subtract out the pressure change due to hydrostatic pressure
    //
    use fluid_mechanics_rust::therminol_component::
        StandardCustomComponentProperties;

    let ctah_hydrostatic_pressure_change = 
        ctah_vertical.get_hydrostatic_pressure_change(
            fluid_temp) +
        ctah_horizontal.get_hydrostatic_pressure_change(
            fluid_temp);

    let ctah_reference_pressure_change = 
        get_ctah_pressure_change_empirical(
            mass_flow_expected,
            fluid_temp);

    let expected_pressure_loss = -(ctah_reference_pressure_change.
        value);



    let actual_pressure_loss = -(ctah_vertical_pressure_change.value +
        ctah_horizontal_pressure_change.value -
        ctah_hydrostatic_pressure_change.value);

    assert_eq!(expected_pressure_loss,actual_pressure_loss);
}

pub fn get_ctah_pressure_change_empirical(
    mass_flow_rate: MassRate,
    fluid_temp: ThermodynamicTemperature) -> Pressure {

    let delta_h = - get_m45_value(mass_flow_rate) + 
        get_m44_value(mass_flow_rate);

    let g: Acceleration = 
        Acceleration::new::<meter_per_second_squared>(-9.81);

    // now to get density
    //import necessary things...
    use fluid_mechanics_rust;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use fluid_mechanics_rust::therminol_component::
        custom_therminol_component::DowthermACustomComponent;

    use fluid_mechanics_rust::therminol_component::FluidProperties;
    use uom::si::acceleration::meter_per_second_squared;


    use uom::si::f64::*;

    let expected_pressure_loss = 0.0;

    // let's get the component for ctah

    let density =  DowthermACustomComponent::density(fluid_temp);

    return -density*g*delta_h;

}


pub fn get_m44_value(mass_flow_rate: MassRate) -> Length {
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::length::meter;

    let mass_rate_kg_per_s: f64 = mass_flow_rate.
        get::<kilogram_per_second>();

    let length_meter = -13.2227506059971 * mass_rate_kg_per_s.powf(2.0) -
        0.839154476992101 * mass_rate_kg_per_s + 
        1.0;

    return Length::new::<meter>(length_meter);

}
pub fn get_m45_value(mass_flow_rate: MassRate) -> Length {
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::length::meter;

    let mass_rate_kg_per_s: f64 = mass_flow_rate.
        get::<kilogram_per_second>();

    let length_meter = -3.43269538780197 * mass_rate_kg_per_s.powf(2.0) -
        0.0361603736781258 * mass_rate_kg_per_s + 
        1.0;

    return Length::new::<meter>(length_meter);

}


