/// Contains structs or classes which
/// help you calculate pressure loss from mass 
/// flowrate and vice versa for pipes
/// with some fixed form losses
///
///
///
pub mod standard_pipe_calc;

/// Contains structs or classes which
/// help you calculate pressure loss from mass 
/// flowrate and vice versa for custom components
/// with custom friction factor and
/// form losses specified by the user
pub mod custom_component_calc;



use uom::si::f64::*;


/// This is a fluid component trait,
/// which specifies that fluid components in general
/// should have the following properties accessed
/// via get and set methods
///
/// ```rust
/// ```
pub trait FluidComponent {

    /// gets the mass flowrate of the component
    fn get_mass_flowrate(&mut self) -> MassRate ;

    /// sets the mass flowrate of the component
    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate);


    /// gets pressure loss
    fn get_pressure_loss(&mut self) -> Pressure;

    /// sets the pressure loss of the component
    fn set_pressure_loss(&mut self, pressure_loss: Pressure);


    /// gets cross sectional area
    fn get_cross_sectional_area(&mut self) -> Area;

    /// sets the cross sectional area of the component
    fn set_cross_sectional_area(&mut self, cross_sectional_area: Area);


    /// gets hydraulic diamter
    fn get_hydraulic_diameter(&mut self) -> Length;

    /// sets the hydraulic diameter of the component
    fn set_hydraulic_diameter(&mut self, hydraulic_diameter: Length);


    /// gets fluid viscosity
    fn get_fluid_viscosity(&mut self) -> DynamicViscosity;

    /// sets the fluid viscosity of the component
    fn set_fluid_viscosity(&mut self, fluid_viscosity: DynamicViscosity);


    /// gets fluid density
    fn get_fluid_density(&mut self) -> MassDensity;

    /// sets the fluid density of the component
    fn set_fluid_density(&mut self, fluid_density: MassDensity);


    /// gets the component length
    fn get_component_length(&mut self) -> Length;

    /// sets the component length 
    fn set_component_length(&mut self, component_length: Length);

}



#[cfg(test)]
mod fluid_component_tests {

    #[test]
    pub fn fluid_component_sandbox_1 () {

    }

}

