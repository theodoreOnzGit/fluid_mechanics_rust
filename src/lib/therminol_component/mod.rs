pub mod custom_therminol_component;
pub mod dowtherm_a_properties;
pub mod custom_therminol_pipe;

// this allows for units
use uom::si::f64::*;

/// A generic base class or struct for therminol vp 1 
/// or dowtherm A components
// i want to create an interface for a generic 
// component, which could be a pipe or a custom component
#[allow(non_snake_case)]
pub struct PipeProperties {
    pub _name: String,
    pub hydraulic_diameter: Length,
    pub component_length: Length,
    pub absolute_roughness: Length,

    // also we should have incline angle in degrees
    // in case of dealing with elevation
    pub incline_angle: Angle,

    // for pipes, we specify a fixed form loss k term
    // not a custom one
    pub form_loss_k: f64,

    // internal pressure term, in case you want to have
    // a pump or something

    pub internal_pressure: Pressure,

}


// here is a trait (or interface for C# or java people), 
// however, traits only deal with methods, not properties.
// note that traits are also types
/// A trait (or interface) for getting fluid properties from
/// temperature or enthalpy
pub trait FluidProperties {
    fn density(fluid_temp: ThermodynamicTemperature) -> MassDensity;

    fn viscosity(fluid_temp: ThermodynamicTemperature) -> DynamicViscosity;

    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy;

    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity;
    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity;

    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature;
}

/// A trait (or interface) for getting pressure change (not loss)
/// from mass flowrate and vice versa
// it inherits from Fluid Properties as Reynolds number always needs
// to be calculated
pub trait CalcPressureChange {
    fn from_mass_rate(&self, fluid_mass_flowrate: MassRate,
                      fluid_temp: ThermodynamicTemperature) -> Pressure;

    fn to_mass_rate(&self, pressure_change: Pressure,
                    fluid_temp: ThermodynamicTemperature) -> MassRate;
}

/// A trait (or interface) for getting pipe form losses and cross
/// sectional areas
/// i also force the implementation of a constructor
// it inherits from CalcPressureChange
// because it will need to calculate pressure changes
// from the above properties, 
// also, constructor will need to be given
pub trait StandardPipeProperties : FluidProperties {
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           form_loss_k: f64) -> Self;

    fn get_cross_sectional_area(&self) -> Area;
    fn get_internal_pressure_term(&self) -> Pressure;
    fn set_internal_pressure_term(&mut self, pressure_pascals: f64);
}




