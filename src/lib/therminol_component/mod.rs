/// Contains code to specify therminol user
/// defined components
///
/// Basically, we use the therminol or dowtherm A
/// properties, calculate viscosity and density
/// based on temperature
/// and then obtain mass flowrate from pressure change
/// or vice versa
///
/// However the user will be able to defined fldk here
pub mod custom_therminol_component;
/// contains correlations for dowtherm A viscosity, density,
/// thermal conductivity and heat capacity
pub mod dowtherm_a_properties;
/// Contains code to specify therminol pipes
///
/// Basically, we use the therminol or dowtherm A
/// properties, calculate viscosity and density
/// based on temperature
/// and then obtain mass flowrate from pressure change
/// or vice versa
///
/// The correlations are only valid for 20C to 180C,
/// the code will panic otherwise (this means throw
/// error or exception)
///
pub mod therminol_pipe;
/// Contains code to initialise user specified
/// therminol components
/// 
/// Example usage:
/// ```rust
///
/// use fluid_mechanics_rust::therminol_component::factory;
///
/// let flowmeter_40 = factory::Flowmeter40::get();
///
/// // now let's have a temperature of 21C and mass flow of 0.15 kg/s
/// use uom::si::f64::*;
/// use uom::typenum::P2;
/// use uom::si::mass_rate::kilogram_per_second;
/// use uom::si::thermodynamic_temperature::degree_celsius;
///
/// let fluid_temp = ThermodynamicTemperature::new::<
///     degree_celsius>(21.0);
/// let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);
///
/// // now let's use the calc pressure change object
/// use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;
///
/// // (1) calculating pressure change from mass flowrate
/// let pressure_change = CalcPressureChange::from_mass_rate(
///     &flowmeter_40,
///     mass_flow_expected,
///     fluid_temp);
///
/// println!("calculated pressure_change: {:?} \n", pressure_change);
///
/// // (2) calculating pressure change to mass flowrate
/// let test_mass_flow = CalcPressureChange::to_mass_rate(
///     &flowmeter_40,
///     pressure_change,
///     fluid_temp);
///
/// println!("expected_mass_rate: {:?}\n", mass_flow_expected);
/// println!("actual_mass_rate: {:?} \n", test_mass_flow);
/// ```
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


/// A trait (or interface) for getting fluid properties from
/// temperature or enthalpy
///
/// here is a trait (or interface for C# or java people), 
/// however, traits only deal with methods, not properties.
/// note that traits are also types
///
/// This trait (or interface for methods) ensures
/// that density, dynamic viscosity,
/// fluid enthalpy,
/// specific heat capacity,
/// thermal conductivity 
///
/// can be evaluated from temperature
///
/// Also, we can get temperature given fluid enthalpy
/// this is because we may want in future to quickly
/// obtain fluid temperature after energy balance
///
///
/// Now the enthalpy, specific heat capacity and
///
pub trait FluidProperties {
    /// fluid density based on temperature,
    fn density(fluid_temp: ThermodynamicTemperature) -> MassDensity;

    /// fluid dynamic viscosity based on temperature,
    fn viscosity(fluid_temp: ThermodynamicTemperature) -> DynamicViscosity;

    /// fluid enthalpy  based on temperature,
    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy;

    /// fluid specific heat capacity  based on temperature,
    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity;

    /// fluid thermal conductivity based on temperature,
    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity;

    /// fluid temperature based on fluid enthalpy
    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature;
}

/// A trait (or interface) for getting pressure change (not loss)
/// from mass flowrate and vice versa
pub trait CalcPressureChange {
    /// pressure change from mass flowrate
    fn from_mass_rate(&self, fluid_mass_flowrate: MassRate,
                      fluid_temp: ThermodynamicTemperature) -> Pressure;

    /// calculates pressure change to mass flowrate
    fn to_mass_rate(&self, pressure_change: Pressure,
                    fluid_temp: ThermodynamicTemperature) -> MassRate;
}

/// A trait (or interface) for getting pipe form losses and cross
/// sectional areas
/// i also force the implementation of a constructor
///
/// it inherits from CalcPressureChange
/// because it will need to calculate pressure changes
/// from the pipe properties, 
///
/// I also have several get and set functions because
/// I was trying to migrate properties from my C# code
pub trait StandardPipeProperties : FluidProperties {
    /// This is the constructor
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           form_loss_k: f64) -> Self;

    /// Just a function to get cross sectional area
    fn get_cross_sectional_area(&self) -> Area;
    /// function to get the internal pressure or
    /// user defined driving force
    fn get_internal_pressure_term(&self) -> Pressure;
    /// function to set the internal pressure or
    /// user defined driving force
    fn set_internal_pressure_term(&mut self, pressure_pascals: f64);
    /// function to obtain hydrostatic pressure change
    /// of the pipe
    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure;
}


/// A trait (or interface) for getting user defined
/// component form losses and cross
/// sectional areas
/// i also force the implementation of a constructor
///
/// it inherits from CalcPressureChange
/// because it will need to calculate pressure changes
/// from the fluid component properties, 
///
/// I also have several get and set functions because
/// I was trying to migrate properties from my C# code
pub trait StandardCustomComponentProperties : FluidProperties {
    /// This is the constructor
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           cross_sectional_area_meters_sq: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           custom_darcy: &'static dyn Fn(f64,f64) -> f64,
           custom_k: &'static dyn Fn(f64) -> f64) -> Self;

    /// Just a function to get cross sectional area
    fn get_cross_sectional_area(&self) -> Area;
    /// function to get the internal pressure or
    /// user defined driving force
    fn get_internal_pressure_term(&self) -> Pressure;
    /// function to set the internal pressure or
    /// user defined driving force
    fn set_internal_pressure_term(&mut self, pressure_pascals: f64);
    /// function to obtain hydrostatic pressure change
    /// of the user defined component
    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure;

}
