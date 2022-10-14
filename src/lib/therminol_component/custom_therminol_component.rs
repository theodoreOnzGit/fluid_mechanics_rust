use crate::fluid_component_calculation::custom_component_calc;
use crate::therminol_component::*;
use dowtherm_a_properties;

use uom::si::length::{meter,millimeter};
use uom::si::pressure::pascal;
use uom::si::angle::degree;
use uom::si::acceleration::meter_per_second_squared;
use uom::si::area::square_meter;


/// This structure contains methods to calculate 
/// pressure losses for a dowtherm A custom component
///
/// The user only needs to follow this code example
/// and use the constructor (or the "new" method)
/// in order to create a custom component to user specifications:
/// 
/// ```rust
/// // first you need to use the DowthermACustomComponent
/// // struct and the 
/// // StandardCustomComponentProperties struct
///
/// use fluid_mechanics_rust::therminol_component::custom_therminol_component::
///     DowthermACustomComponent;
///
/// use crate::fluid_mechanics_rust::therminol_component::
///     StandardCustomComponentProperties;
///
/// // secondly you need to define your fldk functions
/// // both custom f and custom k
/// fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
///     return 0.0;
/// }
///
/// // you should also define behaviour for 0 reynold's number;
/// // the underlying code does not deal with negative Re or
/// // zero Re scenarios
///
/// fn custom_k(mut reynolds_number: f64) -> f64 {
///     let mut reverse_flow = false;
///
///     // the user account for reverse flow scenarios...
///     // and also zero flow scenarios
///     if reynolds_number < 0.0 {
///         reverse_flow = true;
///         reynolds_number = reynolds_number * -1.0;
///     }
///
///     if reynolds_number == 0.0 {
///         return 0.0;
///     }
///
///
///     let custom_k_value = 
///         18.0 + 93000.0/reynolds_number.powf(1.35);
///     // coriolis flowmeter
///
///     if reverse_flow {
///         return -custom_k_value;
///     }
///
///     return custom_k_value;
///
/// }
///
/// // to make a new pipe object, in this case
/// // the flowmeter_40_14a object,
/// // you need to make sure it is of the type
/// // DowthermACustomComponent, using the type annotation
///
/// // after that, use the new method within
/// // StandardCustomComponentProperties
/// // to construct it according to the template
/// // make sure the name is of the type string
/// // using the to_string() method
///
/// let flowmeter_40_14a: DowthermACustomComponent 
///     = StandardCustomComponentProperties::new(
///     "flowmeter_40_14a".to_string(),
///     2.79e-2, // component diameter in meters
///     6.11e-4, // component area in square meters
///     0.36, // component length in meters
///     0.015, // estimated component wall roughness (doesn't matter here,
///            // but i need to fill in
///     0.0, //incline angle in degrees
///     &custom_darcy,
///     &custom_k);
///
/// 
/// // now let's have a temperature of 21C and mass flow of 0.15 kg/s
/// // that's if you want to calculate pressure change
/// // to and from mass flowrate:
///
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::thermodynamic_temperature::degree_celsius;
/// 
/// use uom::si::f64::*;
/// use uom::typenum::P2;
/// use uom::si::mass_rate::kilogram_per_second;
///
/// let fluid_temp = ThermodynamicTemperature::new::<
///     degree_celsius>(21.0);
/// let mass_flow_expected = MassRate::new::<kilogram_per_second>(0.15);
///
/// // now let's use the calc pressure change object
/// use crate::fluid_mechanics_rust::therminol_component::CalcPressureChange;
///
/// // (1) so if you want to calculate pressure change:
///
/// let pressure_change = CalcPressureChange::from_mass_rate(
///                                     &flowmeter_40_14a,
///                                     mass_flow_expected,
///                                     fluid_temp);
///
/// println!("calculated pressure_change: {:?} \n", pressure_change);
///
/// use uom::si::pressure::pascal;
/// // (2) if you want to calculate mass flowrate
///
/// let pressure_change = Pressure::new::<pascal>(0.0_f64);
///
/// let test_mass_flow = CalcPressureChange::to_mass_rate(
///                                     &flowmeter_40_14a,
///                                     pressure_change,
///                                     fluid_temp);
///
/// println!("zeroPressure_mass_rate: {:?} \n", test_mass_flow);
///
/// let pressure_change = Pressure::new::<pascal>(1000.0_f64);
///
/// let test_mass_flow = CalcPressureChange::to_mass_rate(
///                                     &flowmeter_40_14a,
///                                     pressure_change,
///                                     fluid_temp);
///
/// println!("positivePressure_mass_rate: {:?} \n", test_mass_flow);
///
/// let pressure_change = Pressure::new::<pascal>(-1000.0_f64);
///
/// let test_mass_flow = CalcPressureChange::to_mass_rate(
///                                     &flowmeter_40_14a,
///                                     pressure_change,
///                                     fluid_temp);
///
/// println!("negativePressure_mass_rate: {:?} \n", test_mass_flow);
///
/// ```
pub struct DowthermACustomComponent {
    /// the dowtherm_custom_component_properties object
    /// or struct instance
    /// will have pipe length, hydraulic diameter,
    /// angle of inclilne, 
    /// form losses and
    /// roughness 
    ///
    /// as well as the 
    /// custom f and custom K values
    ///
    /// all are user specified through the constructor
    /// 
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



