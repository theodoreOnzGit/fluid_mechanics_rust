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
/// Now the enthalpy, specific heat capacity and thermal
/// conductivity may not be used in fluid mechanics, but they are useful
/// for heat transfer
///
/// this is updated from the FluidProperties trait in therminol component,
/// because i want it to be made into objects. Therefore, it takes
/// an immutable reference to self
///
pub trait FluidProperties {
    /// fluid density based on temperature,
    fn density(&self,
               fluid_temp: ThermodynamicTemperature) -> MassDensity;

    /// fluid dynamic viscosity based on temperature,
    fn viscosity(&self,
                 fluid_temp: ThermodynamicTemperature) -> DynamicViscosity;

    /// fluid enthalpy  based on temperature,
    fn enthalpy(&self,
                fluid_temp: ThermodynamicTemperature) -> AvailableEnergy;

    /// fluid specific heat capacity  based on temperature,
    fn specific_heat_capacity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity;

    /// fluid thermal conductivity based on temperature,
    fn thermal_conductivity(
        &self,
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity;

    /// fluid temperature based on fluid enthalpy
    fn get_temperature_from_enthalpy(
        &self,
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature;
}


// ideally i'd want an easy way to make a selection of which fluid i want
// to use, perhaps via an enum or something,
// then the fluid properties are automatically loaded
// perhaps for convenience i can make a trait object of sorts
// the trait object will be a dependency 
// injected into another trait along with
// the temperature at the constructor
//
// This trait will then be "inherited" and available for immediate use
// I can call it a FluidPropertyAssociatedFunctions kind of trait

/// This trait makes it easier to set the fluid properties
///
/// The basic idea is that you just make an object that implements
/// the FluidProperties trait,
/// use that object as an argument to the function
/// and then get the desired fluid property.
///
/// You one can pick this out from a library or something
/// and basically you're all set
pub trait ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions
<'trait_lifetime>{

    /// fluid density based on temperature,
    /// it uses a static dispatch impl rather than &dyn because
    /// one fluid property should be used for a 
    /// fluid with constant composition and single phase
    ///
    /// it must have a solid implementation though, otherwise it won't work
    fn density(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> MassDensity {

        return fluid_properties.density(fluid_temp);

    }
    /// fluid  viscosity based on temperature
    fn viscosity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> DynamicViscosity{

        return fluid_properties.viscosity(fluid_temp);

    }

    /// fluid specific enthalpy based on temperature
    fn enthalpy(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> AvailableEnergy{

        return fluid_properties.enthalpy(fluid_temp);

    }

    /// fluid  specific_heat_capacity based on temperature
    fn specific_heat_capacity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> SpecificHeatCapacity{

        return fluid_properties.specific_heat_capacity(fluid_temp);

    }

    /// fluid thermal conductivity based on temperature
    fn thermal_conductivity(fluid_temp: ThermodynamicTemperature,
               fluid_properties: &dyn FluidProperties) -> ThermalConductivity{

        return fluid_properties.thermal_conductivity(fluid_temp);

    }

    /// fluid temperature based on enthalpy
    fn get_temperature_from_enthalpy(fluid_enthalpy: AvailableEnergy,
               fluid_properties: &dyn FluidProperties) 
        -> ThermodynamicTemperature{

        return fluid_properties.get_temperature_from_enthalpy(fluid_enthalpy);

    }

    /// a function to return a set FluidProperties Object
    fn get_fluid_properties() -> &'trait_lifetime dyn FluidProperties;
}

