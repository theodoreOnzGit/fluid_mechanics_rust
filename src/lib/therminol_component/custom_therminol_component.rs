use crate::fluid_component_calculation::custom_component_calc;
use crate::therminol_component::*;
use dowtherm_a_properties;

use uom::si::length::{meter,millimeter};
use uom::si::pressure::pascal;
use uom::si::angle::degree;
use uom::si::acceleration::meter_per_second_squared;
use uom::si::area::square_meter;


// the 'a notation states that
// the dowthermACustomComponent only lives as long as
// the custom_component_properties object
pub struct DowthermACustomComponent {
    pub dowtherm_custom_component_properties: CustomComponentProperties,
}

impl FluidProperties for DowthermACustomComponent {
    fn density(fluid_temp: ThermodynamicTemperature) -> MassDensity {
        return dowtherm_a_properties::getDowthermADensity(fluid_temp);
    }

    fn viscosity(
        fluid_temp: ThermodynamicTemperature) -> DynamicViscosity{
        return dowtherm_a_properties::getDowthermAViscosity(fluid_temp);
    }

    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{
        return dowtherm_a_properties::getDowthermAEnthalpy(fluid_temp);
    }

    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity{
        return dowtherm_a_properties::
            getDowthermAConstantPressureSpecificHeatCapacity(
            fluid_temp);
    }

    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity{
        return dowtherm_a_properties::
            getDowthermAThermalConductivity(fluid_temp);
    }

    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature{
        return dowtherm_a_properties::
            get_temperature_from_enthalpy(fluid_enthalpy);
    }

}

impl StandardCustomComponentProperties for DowthermACustomComponent {
    // constructor
    fn new(name: String,
           hydraulic_diameter_meters: f64,
           cross_sectional_area_meters_sq: f64,
           component_length_meters: f64,
           absolute_roughness_millimeters: f64,
           incline_angle_degrees: f64,
           custom_darcy: &'static dyn Fn(f64,f64) -> f64,
           custom_k: &'static dyn Fn(f64) -> f64) -> Self {

        let input_hydraulic_diameter = Length::new::<meter>(
            hydraulic_diameter_meters);
        let input_xs_area = Area::new::<square_meter>(cross_sectional_area_meters_sq);
        let input_component_length = Length::new::<meter>(
            component_length_meters);
        let input_absolute_roughness = Length::new::<millimeter>(
            absolute_roughness_millimeters);
        let input_incline_angle = Angle::new::<degree>(
            incline_angle_degrees);
        let input_internal_pressure = Pressure::new::<pascal>(
            0.0);

        let custom_pipe_properties = CustomComponentProperties {
            _name: name,
            hydraulic_diameter: input_hydraulic_diameter,
            xs_area: input_xs_area,
            component_length: input_component_length,
            absolute_roughness: input_absolute_roughness,
            incline_angle: input_incline_angle,
            custom_darcy: custom_darcy,
            custom_k: custom_k,
            internal_pressure: input_internal_pressure,
        };

        return Self { dowtherm_custom_component_properties : custom_pipe_properties };
    }

    fn get_cross_sectional_area(&self) -> Area {

        return self.dowtherm_custom_component_properties.
            xs_area;
    }


    fn get_internal_pressure_term(&self) -> Pressure {
        return self.dowtherm_custom_component_properties.internal_pressure;
    }

    fn set_internal_pressure_term(&mut self, pressure_pascals: f64) {
        self.dowtherm_custom_component_properties.internal_pressure =
            Pressure::new::<pascal>(pressure_pascals);
    }

    fn get_hydrostatic_pressure_change(
        &self, fluid_temp: ThermodynamicTemperature) -> Pressure {

        let pipe_length = self.dowtherm_custom_component_properties.component_length;
        let incline_angle = self.dowtherm_custom_component_properties.incline_angle;
        let fluid_density = DowthermACustomComponent::density(fluid_temp);

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }
}

impl CalcPressureChange for DowthermACustomComponent {

    fn from_mass_rate(&self, fluid_mass_flowrate: MassRate,
                      fluid_temp: ThermodynamicTemperature) -> Pressure {

        // first let's get all the relevant properties...
        let hydraulic_diameter = self.dowtherm_custom_component_properties.hydraulic_diameter;
        let pipe_length = self.dowtherm_custom_component_properties.component_length;
        let absolute_roughness = self.dowtherm_custom_component_properties.absolute_roughness;
        let xs_area = self.get_cross_sectional_area();

        let fluid_viscosity = DowthermACustomComponent::viscosity(fluid_temp);
        let fluid_density = DowthermACustomComponent::density(fluid_temp);

        let custom_k = self.dowtherm_custom_component_properties.custom_k;
        let custom_darcy = self.dowtherm_custom_component_properties.custom_darcy;

        // second let's get pressure loss from mass rate
        // by using this function or method, we assume that
        // the pipe behaves symmetrically in reverse flow
        let pressure_loss = custom_component_calc::CalcPressureLoss::
            from_mass_rate(
                fluid_mass_flowrate,
                xs_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                custom_darcy,
                custom_k);
        // now to calculate pressure change
        // we note this equation
        //
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        //
        // for hydrostatic pressure gain
        // g is earth gravity at 9.81
        // delta H is positive upwards
        let hydrostatic_pressure_increase: Pressure =
            self.get_hydrostatic_pressure_change(
                fluid_temp);
        // last but not least we need our source pressure

        let source_pressure: Pressure = self.dowtherm_custom_component_properties.
            internal_pressure;

        // now we can calculate pressure change

        let pressure_change = 
            -pressure_loss +
            hydrostatic_pressure_increase +
            source_pressure;

        return pressure_change;
    }

    fn to_mass_rate(&self, pressure_change: Pressure,
                    fluid_temp: ThermodynamicTemperature) -> MassRate {
        // first let's get all the relevant properties...
        let hydraulic_diameter = self.dowtherm_custom_component_properties.hydraulic_diameter;
        let pipe_length = self.dowtherm_custom_component_properties.component_length;
        let absolute_roughness = self.dowtherm_custom_component_properties.absolute_roughness;
        let xs_area = self.get_cross_sectional_area();

        let fluid_viscosity = DowthermACustomComponent::viscosity(fluid_temp);
        let fluid_density = DowthermACustomComponent::density(fluid_temp);

        let custom_k = self.dowtherm_custom_component_properties.custom_k;
        let custom_darcy = self.dowtherm_custom_component_properties.custom_darcy;

        // now we need to calculate a pressure loss term
        // we use:
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        // so we just add pressure loss to both sides and subtract pressure
        // change to both sides
        // pressure loss  = - pressure change + hydrostatic pressure +
        // source pressure

        // for hydrostatic pressure gain
        // g is earth gravity at 9.81
        // delta H is positive upwards
        let hydrostatic_pressure_increase: Pressure =
            self.get_hydrostatic_pressure_change(
                fluid_temp);
        // last but not least we need our source pressure

        let source_pressure: Pressure = self.dowtherm_custom_component_properties.
            internal_pressure;

        // now calculate pressure loss
        let pressure_loss = 
            -pressure_change +
            hydrostatic_pressure_increase +
            source_pressure;

        let mass_rate = custom_component_calc::CalcPressureLoss::
            to_mass_rate(
                pressure_loss,
                xs_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                custom_darcy,
                custom_k);

        return mass_rate;
    }
}



