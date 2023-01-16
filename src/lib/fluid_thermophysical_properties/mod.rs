use uom::si::f64::*;


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
