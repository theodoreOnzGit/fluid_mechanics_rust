
pub mod custom_therminol_component;
/// contains correlations for dowtherm A viscosity, density,
/// thermal conductivity and heat capacity
pub mod dowtherm_a_properties;
pub mod therminol_pipe;
/// Contains code to initialise user specified
/// therminol components
pub mod factory;

// this allows for units
use uom::si::f64::*;

/// A generic base class or struct for therminol vp 1 
/// or dowtherm A pipe like components
// i want to create an interface for a generic 
// component, which could be a pipe or a custom component
pub struct PipeProperties {
    /// name of the therminol pipe
    pub _name: String,
    /// hydraulic diameter of the therminol pipe
    /// pipes are assumed circular in cross section
    pub hydraulic_diameter: Length,
    /// pipe length
    pub component_length: Length,
    /// pipe absolute roughness, not relative roughness
    pub absolute_roughness: Length,

    /// also we should have incline angle in degrees
    /// in case of dealing with elevation
    pub incline_angle: Angle,

    /// for pipes, we specify a fixed form loss k term
    /// not a custom one
    pub form_loss_k: f64,

    /// internal pressure term, in case you want to have
    /// a pump or something
    ///
    /// The user here specifies a fixed internal pressure
    /// source, and that will drive the flow forward or
    /// backward depending on the sign of the pressure
    pub internal_pressure: Pressure,

}

/// A generic base class for therminol or dowtherm A
/// components with user defined fldk terms
pub struct CustomComponentProperties{
    /// name of the user specified component
    pub _name: String,
    /// component hydraulic diameter, usually 4A/P
    /// 
    /// A is cross sectional area
    /// P is wetted perimeter
    pub hydraulic_diameter: Length,
    /// cross sectional area of the user specified
    /// component
    pub xs_area: Area,
    /// component length for user specified component
    pub component_length: Length,
    /// component absolute roughness, not relative roughness
    pub absolute_roughness: Length,

    /// also we should have incline angle in degrees
    /// in case of dealing with elevation
    pub incline_angle: Angle,

    /// for custom fldk component, i have a custom 
    /// form loss term which is essentially a function
    /// i put it as static because i want it to live through the
    /// duration of the program
    ///
    /// so, this custom k is actually a reference to the user 
    /// specified function, not a copy
    /// However, it is not a mutable reference so 
    /// it should be thread safe in case you want to do
    /// parallel computation
    pub custom_k: &'static dyn Fn(f64) -> f64,

    /// for custom fldk component, i have a custom 
    /// form loss term which is essentially a function
    /// i put it as static because i want it to live through the
    /// duration of the program
    ///
    /// so, this custom darcy is actually a reference to the user 
    /// specified function, not a copy
    /// However, it is not a mutable reference so 
    /// it should be thread safe in case you want to do
    /// parallel computation
    pub custom_darcy: &'static dyn Fn(f64,f64) -> f64,

    /// internal pressure term, in case you want to have
    /// a pump or something
    ///
    /// The user here specifies a fixed internal pressure
    /// source, and that will drive the flow forward or
    /// backward depending on the sign of the pressure
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
    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure;
}


pub trait StandardCustomComponentProperties : FluidProperties {
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           cross_sectional_area_meters_sq: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           custom_darcy: &'static dyn Fn(f64,f64) -> f64,
           custom_k: &'static dyn Fn(f64) -> f64) -> Self;

    fn get_cross_sectional_area(&self) -> Area;
    fn get_internal_pressure_term(&self) -> Pressure;
    fn set_internal_pressure_term(&mut self, pressure_pascals: f64);
    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure;

}
