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
use uom::si::acceleration::meter_per_second_squared;

/// This is a generic fluid component trait,
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

    /// gets the mass flowrate of component given a 
    /// fixed pressure change
    /// does so by immutably borrowing the object
    /// 
    fn get_mass_flowrate_from_pressure_change_immutable(
        &self, pressure_change: Pressure) -> MassRate {

        // the basic idea is to change the pressure change
        // variable into pressure loss and call the pressure loss
        // function
        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //

        let pipe_length = 
            self.get_component_length_immutable();

        let incline_angle = 
            self.get_incline_angle_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();



        let pressure_loss = -pressure_change +
            self.get_hydrostatic_pressure_change_immutable(
                pipe_length, 
                incline_angle, 
                fluid_density)+
            self.get_internal_pressure_source_immutable();


        let mass_rate = 
            self.get_mass_flowrate_from_pressure_loss_immutable(
                pressure_loss);

        return mass_rate;
    }


    /// gets the mass flowrate of component given a 
    /// fixed pressure change
    /// does so by immutably borrowing the object
    /// 
    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self, pressure_loss: Pressure) -> MassRate;

    /// gets pressure loss
    fn get_pressure_loss(&mut self) -> Pressure;

    /// sets the pressure loss of the component
    fn set_pressure_loss(&mut self, pressure_loss: Pressure);

    /// gets the pressure loss of component given a 
    /// fixed mass flowrate
    /// does so by immutably borrowing the object
    fn get_pressure_loss_immutable(
        &self, mass_flowrate: MassRate) -> Pressure;


    /// gets cross sectional area
    fn get_cross_sectional_area(&mut self) -> Area;

    /// gets cross sectional area with immutable instance of self
    fn get_cross_sectional_area_immutable(&self) -> Area;

    /// gets hydraulic diamter
    fn get_hydraulic_diameter(&mut self) -> Length;

    /// gets hydraulic diamter with immutable instance of self
    fn get_hydraulic_diameter_immutable(&self) -> Length;

    /// gets fluid viscosity
    fn get_fluid_viscosity(&mut self) -> DynamicViscosity;

    /// gets fluid viscosity with an immutable instance of self
    fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity;

    /// gets fluid density
    fn get_fluid_density(&mut self) -> MassDensity;

    /// gets fluid density with an immutable instance of self
    fn get_fluid_density_immutable(&self) -> MassDensity;

    /// gets the component length
    fn get_component_length(&mut self) -> Length;

    /// gets the component length immutably
    fn get_component_length_immutable(&self) -> Length;

    /// gets pressure change for a pipe given
    /// the set parameters
    fn get_pressure_change(&mut self) -> Pressure {

        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //


        let pressure_loss = self.get_pressure_loss();

        // this is the second component: hydrostatic pressure

        let component_length = self.get_component_length();
        let incline_angle = self.get_incline_angle();
        let fluid_density = self.get_fluid_density();

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change(
                component_length,
                incline_angle,
                fluid_density);

        // third component is pressure source

        let pressure_source = self.get_internal_pressure_source();

        return -pressure_loss + hydrostatic_pressure_increase + 
            pressure_source;
    }

    /// gets the pressure loss of component given a 
    /// fixed mass flowrate
    /// does so by immutably borrowing the object
    fn get_pressure_change_immutable(
        &self, mass_flowrate: MassRate) -> Pressure{


        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //


        let pressure_loss = self.get_pressure_loss_immutable(
            mass_flowrate);

        // this is the second component: hydrostatic pressure

        let component_length = self.get_component_length_immutable();
        let incline_angle = self.get_incline_angle_immutable();
        let fluid_density = self.get_fluid_density_immutable();

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change_immutable(
                component_length,
                incline_angle,
                fluid_density);

        // third component is pressure source

        let pressure_source = self.get_internal_pressure_source_immutable();

        return -pressure_loss + hydrostatic_pressure_increase + 
            pressure_source;
    }


    /// sets the pressure change for the given pipe
    fn set_pressure_change(&mut self, pressure_change: Pressure){

        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //

        let component_length = self.get_component_length();
        let incline_angle = self.get_incline_angle();
        let fluid_density = self.get_fluid_density();

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change(
                component_length,
                incline_angle,
                fluid_density);

        // third component is pressure source
        // for any internal pressure source or external, eg pumps

        let pressure_source = self.get_internal_pressure_source();

        // we then get the pressure loss term
        //

        let pressure_loss = -pressure_change + hydrostatic_pressure_increase +
            pressure_source;

        self.set_pressure_loss(pressure_loss);
    }

    

    /// gets the angle of incline for a pipe
    fn get_incline_angle(&mut self) -> Angle;

    /// gets the incline angle of the pipe with immutable self
    fn get_incline_angle_immutable(&self) -> Angle;

    /// gets the hydrostatic pressure change
    /// using h rho g
    ///
    /// the height increase is equal
    ///
    /// h = pipe_length * sin (incline_angle)
    fn get_hydrostatic_pressure_change(
        &mut self, 
        pipe_length: Length,
        incline_angle: Angle,
        fluid_density: MassDensity) -> Pressure {

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }

    /// gets the hydrostatic pressure change
    /// with an immutable instance of self
    /// using h rho g
    ///
    /// the height increase is equal
    ///
    /// h = pipe_length * sin (incline_angle)
    fn get_hydrostatic_pressure_change_immutable(
        &self,
        pipe_length: Length,
        incline_angle: Angle,
        fluid_density: MassDensity) -> Pressure {

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }

    /// gets the pressure source for a fluid component
    fn get_internal_pressure_source(&mut self) -> Pressure;


    /// gets the pressure source for a fluid component
    /// with an immutable instance of self
    fn get_internal_pressure_source_immutable(&self) -> Pressure;

    /// sets the internal pressure source for a pipe
    fn set_internal_pressure_source(
        &mut self,
        internal_pressure: Pressure);

}





#[cfg(test)]
pub mod fluid_component_tests_and_examples {
    use std::f64::consts::PI;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::
        custom_component_calc::{FluidCustomComponentCalcPressureChange, FluidCustomComponentCalcPressureLoss};
    use crate::fluid_component_calculation::standard_pipe_calc
        ::{FluidPipeCalcPressureLoss,FluidPipeCalcPressureChange};
    use crate::therminol_component::
        dowtherm_a_properties::getDowthermAConstantPressureSpecificHeatCapacity;
    use uom::si::dynamic_viscosity::{millipascal_second, pascal_second,poise};
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal, kilopascal};
    use uom::si::angle::degree;
    
    /// Example 1: 
    ///
    /// This example shows how to create a simple pipe
    /// using the FluidComponent and FluidPipeCalcPressureLoss,
    /// traits
    ///
    /// this is by no means the best way to do it, but its a start
    /// remember to use the relevant imports in the fluid component
    /// tests
    ///
    /// it is made of copper, 1m long, 2 in in diameter
    ///
    /// This does not take inclined angles into consideration yet
    #[test]
    pub fn simple_fluid_pipe_example_1 () {
        
        // first we create an air pipe struct
        //
        struct AirPipe {
            mass_flowrate: MassRate,
            pressure_loss: Pressure,
        }

        // we implement get and set methods for each of the 
        // properties, you can set these properties in the constructor
        // or you can simply return the appropriate values in the 
        // functions
        // 
        // likewise, when you get the mass flowrate
        // or density, you can invoke calculation methods straightaway
        // 
        // but for calculation methods, you can "inherit" the default
        // trait implementations for a generic fluid pipe
        impl FluidComponent for AirPipe {

            /// gets the mass flowrate of the component
            fn get_mass_flowrate(&mut self) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();
                let pressure_loss = self.pressure_loss;

                let mass_flowrate = 
                    self.pipe_calc_mass_flowrate(
                        pressure_loss, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.mass_flowrate;
            }

            /// gets the mass flowrate of the component
            /// with immutable instance of self
            fn get_mass_flowrate_from_pressure_loss_immutable(
                &self,
                pressure_loss: Pressure) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();

                let mass_flowrate = 
                    self.pipe_calc_mass_flowrate(
                        pressure_loss, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.mass_flowrate;
            }

            /// sets the mass flowrate of the component
            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.mass_flowrate = mass_flowrate;
            }


            /// pressure change is accounts for total pressure
            /// differential between start and end point of the pipe,
            /// including hydrostatic pressure and any sources
            /// which may contribute to the pressure, eg. pumps
            /// 
            /// pressure change = -pressure loss + hydrostatic pressure
            fn get_pressure_change(&mut self) -> Pressure {

                // for this, i have
                // pressure change = -pressure loss + hydrostatic pressure
                // + internal pressure
                return -self.get_pressure_loss();
            }


            fn set_pressure_change(&mut self, pressure_change:Pressure) {
                self.set_pressure_loss(-pressure_change);
            }

            /// gets pressure loss
            /// i calculate pressure loss when i invoke this method
            /// and the method comes from the 
            /// FluidPipeCalcPressureLoss trait 
            fn get_pressure_loss(&mut self) -> Pressure {

                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let mass_flowrate = self.mass_flowrate;
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    self.pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it

                self.pressure_loss = pressure_loss;

                return self.pressure_loss;
            }

            fn get_pressure_loss_immutable(
                &self, mass_flowrate: MassRate) -> Pressure {

                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    self.pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it

                self.pressure_loss = pressure_loss;

                return self.pressure_loss;
            }

            /// sets the pressure loss of the component
            fn set_pressure_loss(&mut self, pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }


            /// gets cross sectional area
            /// the inner diameter is 2 in
            /// and the area is Pi*d^2/4
            fn get_cross_sectional_area(&mut self) -> Area {
                return self.get_hydraulic_diameter()*
                    self.get_hydraulic_diameter()*
                    PI/4.0_f64;
            }

            fn get_cross_sectional_area_immutable(&self) -> Area {
                return self.get_hydraulic_diameter_immutable()*
                    self.get_hydraulic_diameter_immutable()*
                    PI/4.0_f64;
            }

            /// gets hydraulic diamter
            /// im giving this pipe a two inch inner diameter 
            fn get_hydraulic_diameter(&mut self) -> Length {
                return Length::new::<inch>(2.0);
            }

            fn get_hydraulic_diameter_immutable(&self) -> Length {
                return Length::new::<inch>(2.0);
            }

            /// gets fluid viscosity
            /// air has a dynamic viscosity of about 18.6 millipascal
            /// seconds
            fn get_fluid_viscosity(&mut self) -> DynamicViscosity{ 
                return DynamicViscosity::new::<millipascal_second>(18.6);
            }

            fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity{ 
                return DynamicViscosity::new::<millipascal_second>(18.6);
            }


            /// gets fluid density
            /// air density is about 1kg/m3
            fn get_fluid_density(&mut self) -> MassDensity {
                return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
            }

            fn get_fluid_density_immutable(&self) -> MassDensity {
                return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
            }

            /// gets the component length
            /// you can set the component length here
            fn get_component_length(&mut self) -> Length {
                return Length::new::<meter>(1.0);
            }

            fn get_component_length_immutable(&self) -> Length {
                return Length::new::<meter>(1.0);
            }

            /// i'm manually fixing the incline angle at zero
            /// meaning that this pipe is horizontal
            fn get_incline_angle(&mut self) -> Angle {
                return Angle::new::<degree>(0.0);
            }
            
            fn get_incline_angle_immutable(&self) -> Angle {
                return Angle::new::<degree>(0.0);
            }

            /// For the air pipe, there should be no internal source

            fn get_internal_pressure_source(&mut self) -> Pressure {
                return Pressure::new::<pascal>(0.0);
            }

            fn get_internal_pressure_source_immutable(&self) -> Pressure {
                return Pressure::new::<pascal>(0.0);
            }

            fn set_internal_pressure_source(
                &mut self, 
                _internal_pressure_source: Pressure
                ){
                // doesn't actually do anything,
                // i refuse to let it set anything
                //
                // rather i have it panic a special kind of panic
                // called unimplemented

                unimplemented!();

            }


        }


        // we can "inherit" methods for the pipe pressure loss
        // and mass flowrate calculations 
        //
        // all you need to do is set form loss K
        // and absolute roughness

        impl FluidPipeCalcPressureLoss for AirPipe {

            /// return form loss K for the pipe
            /// i set it at 5
            ///
            fn get_pipe_form_loss_k(&mut self) -> f64 {
                return 5.0;
            }

            /// return absolute roughness for pipe
            /// for a typical copper pipe
            /// it is 0.002 mm 
            /// i did a web search
            ///
            fn get_pipe_absolute_roughness(&mut self) -> Length {
                return Length::new::<millimeter>(0.002);
            }
        }

        // finally you can implement a constructor

        impl AirPipe {
            pub fn new() -> Self {
                let default_mass_flowrate = 
                    MassRate::new::<kilogram_per_second>(0.0);

                let default_pressure_loss = 
                    Pressure::new::<pascal>(0.0);

                return Self { 
                    mass_flowrate: default_mass_flowrate, 
                    pressure_loss: default_pressure_loss
                }
            }
        }


        // with the AirPipe struct setup, you can caluclate
        // the pressure loss easily

        let mut pipe_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(0.5);

        let mut air_pipe_1 = AirPipe::new();

        // first we set the mass flowrate
        air_pipe_1.set_mass_flowrate(pipe_mass_flowrate);

        // next we obtain the pressure loss
        let mut pressure_loss = air_pipe_1.get_pressure_loss();

        // the value is around 209 kPa
        approx::assert_relative_eq!(
            209.0*1000.0,
            pressure_loss.value,
            max_relative=0.01);

        // we can of course use the 209 kPa value and set the
        // air pipe presusre to such
        //

        pressure_loss = Pressure::new::<kilopascal>(209_f64);

        air_pipe_1.set_pressure_loss(pressure_loss);

        pipe_mass_flowrate = 
            air_pipe_1.get_mass_flowrate();


        // we should get back our 0.5 kg/s
        approx::assert_relative_eq!(
            0.5,
            pipe_mass_flowrate.value,
            max_relative=0.01);

        return;

    }

    /// Example 2:
    ///
    /// We saw previously how to create an air pipe
    /// now we shall make a slanted water pipe
    /// with some internal pressure source (as if it had a pump attached
    /// to it)
    ///
    /// we shall improve on how we can create the pipes
    /// to do so, we shall use the FluidComponent trait and the 
    /// FluidPipeCalcPressureChange trait
    ///
    #[test]
    pub fn water_pipe_with_internal_pump_example_2() {

        // first we want to start with a water pipe struct,
        // this time, we use the constructor to define both
        // pipe properties and fluid properties
        //
        // this is still an isothermal case
        //
        // you may want to implement the traits so that you know what data
        // you need to have

        struct WaterPipe {
            mass_flowrate: MassRate,
            pressure_loss: Pressure,
            dynamic_viscosity: DynamicViscosity,
            density: MassDensity,
            form_loss_k: f64,
            absolute_roughness: Length,
            incline_angle: Angle,
            internal_pressure_source: Pressure,
            pipe_length: Length,
            hydraulic_diameter: Length,
        }

        impl FluidPipeCalcPressureChange for WaterPipe {}

        impl FluidPipeCalcPressureLoss for WaterPipe {
            fn get_pipe_form_loss_k(&mut self) -> f64 {
                return self.form_loss_k;
            }

            fn get_pipe_absolute_roughness(&mut self) -> Length {
                return self.absolute_roughness;
            }
        }

        impl FluidComponent for WaterPipe {
            fn get_internal_pressure_source(&mut self) -> Pressure {
                return self.internal_pressure_source;
            }

            fn get_internal_pressure_source_immutable(&self) -> Pressure {
                return self.internal_pressure_source;
            }

            fn set_internal_pressure_source(
                &mut self,
                internal_pressure_source: Pressure){
                self.internal_pressure_source = internal_pressure_source;
            }

            fn get_component_length(&mut self) -> Length {
                return self.pipe_length;
            }


            fn get_component_length_immutable(&self) -> Length {
                return self.pipe_length;
            }

            fn get_incline_angle(&mut self) -> Angle {
                return self.incline_angle;
            }

            fn get_incline_angle_immutable(&self) -> Angle {
                return self.incline_angle;
            }

            fn get_fluid_density(&mut self) -> MassDensity {
                return self.density;
            }

            fn get_fluid_density_immutable(&self) -> MassDensity {
                return self.density;
            }

            fn get_fluid_viscosity(&mut self) -> DynamicViscosity {
                return self.dynamic_viscosity;
            }

            fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {
                return self.dynamic_viscosity;
            }

            fn get_hydraulic_diameter(&mut self) -> Length {
                return self.hydraulic_diameter;
            }

            fn get_hydraulic_diameter_immutable(&self) -> Length {
                return self.hydraulic_diameter;
            }

            fn get_cross_sectional_area(&mut self) -> Area {
                return self.get_hydraulic_diameter()*
                    self.get_hydraulic_diameter()*
                    PI/4.0_f64;
            }

            fn get_cross_sectional_area_immutable(&self) -> Area {
                return self.get_hydraulic_diameter()*
                    self.get_hydraulic_diameter()*
                    PI/4.0_f64;
            }

            fn set_pressure_loss(&mut self, pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }

            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.mass_flowrate = mass_flowrate;
            }

            fn get_mass_flowrate(&mut self) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();
                let pressure_loss = self.pressure_loss;
                let incline_angle = self.get_incline_angle();
                let internal_pressure_source = self.get_internal_pressure_source();

                let pressure_change = 
                    -pressure_loss 
                    + internal_pressure_source 
                    + self.get_hydrostatic_pressure_change(
                        pipe_length,
                        incline_angle,
                        fluid_density);

                let mass_flowrate = 
                    self.pipe_calculate_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k,
                        incline_angle,
                        internal_pressure_source);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.mass_flowrate;

            }

            fn get_mass_flowrate_from_pressure_loss_immutable(
                &self,
                pressure_loss: Pressure) -> MassRate {
                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();
                let pressure_loss = self.pressure_loss;
                let incline_angle = self.get_incline_angle();
                let internal_pressure_source = self.get_internal_pressure_source();

                let pressure_change = 
                    -pressure_loss 
                    + internal_pressure_source 
                    + self.get_hydrostatic_pressure_change(
                        pipe_length,
                        incline_angle,
                        fluid_density);

                let mass_flowrate = 
                    self.pipe_calculate_mass_flowrate_from_pressure_change(
                        pressure_change, 
                        cross_sectional_area, 
                        hydraulic_diameter, 
                        fluid_viscosity, 
                        fluid_density, 
                        pipe_length, 
                        absolute_roughness, 
                        form_loss_k,
                        incline_angle,
                        internal_pressure_source);

                // you can return the mass flowrate straightaway
                // or set the struct variable first and then
                // return it

                self.set_mass_flowrate(mass_flowrate);

                return self.mass_flowrate;

            }

            fn get_pressure_loss(&mut self) -> Pressure {


                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let mass_flowrate = self.mass_flowrate;
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    self.pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it

                self.pressure_loss = pressure_loss;

                return self.pressure_loss;
            }

            fn get_pressure_loss_immutable(
                &self,
                mass_flowrate: MassRate) -> Pressure {


                // get pipe parameters and flow conditions
                // from the get methods
                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let mass_flowrate = self.mass_flowrate;
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let viscosity = self.get_fluid_viscosity();
                let density = self.get_fluid_density();
                let pipe_legnth = self.get_component_length();


                // calculate the pressure loss

                let pressure_loss = 
                    self.pipe_calc_pressure_loss(
                        mass_flowrate,
                        cross_sectional_area,
                        hydraulic_diameter,
                        viscosity,
                        density,
                        pipe_legnth,
                        absolute_roughness,
                        form_loss_k);

                // you can return the pressure loss straightaway
                // or set the struct variable first and then
                // return it

                self.pressure_loss = pressure_loss;

                return self.pressure_loss;
            }

            fn set_pressure_change(&mut self, pressure_change: Pressure){
                // we use the following formula
                // pressure_change = -pressure_loss + hydrostatic_pressure +
                // internal pressure source
                //
                // by setting pressure change, we are indirectly setting
                // pressure loss
                //

                let pipe_length = self.get_component_length();
                let incline_angle = self.get_incline_angle();
                let fluid_density = self.get_fluid_density();

                let pressure_loss = -pressure_change +
                    self.get_hydrostatic_pressure_change(
                        pipe_length,
                        incline_angle,
                        fluid_density) +
                    self.get_internal_pressure_source();

                self.set_pressure_loss(pressure_loss);
            }

            fn get_pressure_change(&mut self) -> Pressure {

                let form_loss_k = self.get_pipe_form_loss_k();
                let absolute_roughness = self.get_pipe_absolute_roughness();
                let cross_sectional_area = self.get_cross_sectional_area();
                let hydraulic_diameter = self.get_hydraulic_diameter();
                let fluid_viscosity = self.get_fluid_viscosity();
                let fluid_density = self.get_fluid_density();
                let pipe_length = self.get_component_length();
                let incline_angle = self.get_incline_angle();
                let internal_pressure_source = self.get_internal_pressure_source();
                let mass_flowrate = self.mass_flowrate;


                // return the pressure change value
                let pressure_change = self.pipe_calc_pressure_change(
                    mass_flowrate,
                    cross_sectional_area,
                    hydraulic_diameter,
                    fluid_viscosity,
                    fluid_density,
                    pipe_length,
                    absolute_roughness,
                    form_loss_k,
                    incline_angle,
                    internal_pressure_source);

                self.set_pressure_change(pressure_change);

                return pressure_change;

            }
        }

        // lastly we implement the constructor,
        // since we know the pipe has water flowing through,
        // density and viscosity are fixed
        //
        // Everything else though, has to be set by the user
        // mass flowrate and pressure loss can be
        // set to 0 by default
        // 
        // internal pressure source is also set to 0,
        // it is up to the user to set internal pressure source
        impl WaterPipe {
            fn new(form_loss_k: f64,
                   absolute_roughness: Length,
                   incline_angle: Angle,
                   pipe_length: Length,
                   hydraulic_diameter: Length) -> Self {

                return Self {
                    mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
                    pressure_loss: Pressure::new::<pascal>(0.0),
                    dynamic_viscosity: DynamicViscosity::new::<poise>(0.01),
                    density: MassDensity::new::<kilogram_per_cubic_meter>(1000.0),
                    form_loss_k: form_loss_k,
                    absolute_roughness: absolute_roughness,
                    incline_angle: incline_angle,
                    internal_pressure_source: Pressure::new::<pascal>(0.0),
                    pipe_length: pipe_length,
                    hydraulic_diameter: hydraulic_diameter,
                };
            }
        }

        // and just like that we've finished defining our water pipe
        //
        // pipe shall be 1m long, angled 25 degrees
        // 1 inch diameter
        // form loss is 0.5
        // copper, 0.002 mm roughness

        let mut water_pipe_1 = WaterPipe::new(
            0.5, // form losses
            Length::new::<millimeter>(0.002), // surface roughness
            Angle::new::<degree>(25.0), // incline angle
            Length::new::<meter>(1.0), // pipe length
            Length::new::<inch>(1.0)); // pipe inner diameter


        // let's set mass flowrate at 0.5 kg/s
        water_pipe_1.set_mass_flowrate(
            MassRate::new::<kilogram_per_second>(0.5)
            );

        // find the pressure change

        let pressure_change = water_pipe_1.get_pressure_change();

        // pressure change is -4861 Pa
        approx::assert_relative_eq!(
            pressure_change.value,
            -4861_f64,
            max_relative = 0.01 );

        // likewise when i get my mass flowrate from pressure change
        // i should get the same value



        let mass_flowrate = 
            water_pipe_1.get_mass_flowrate();

        water_pipe_1.set_pressure_change(
            Pressure::new::<pascal>(-4861_f64));

        approx::assert_relative_eq!(
            mass_flowrate.value,
            0.5,
            max_relative = 0.01 );

        // and that concludes the example! You can now set 
        // the water pipe to anything you want.
        //
        // of course, it will be good to have common enums and cases
        // that can return surface roughness of commonly used material
        // as well as densities, viscosities, etc.
        //
        // Likely I'll put them in some property library stored as a trait



    }

    /// Example 3,
    /// 
    /// suppose now we have a coriolis flowmeter
    /// with a custom friction factor correlation
    ///
    /// (f_darcy L/D + K) = 18 + 93000/Re^1.35
    ///
    /// we shall use water to push flow through this coriolis flowmeter
    ///
    /// also, the programming is rather tedious
    /// because of lifetimes, but this is one example of how it can be done
    #[test]
    pub fn coriolis_flowmeter_empirical_custom_component_example_3(){

        struct CoriolisFlowmeter<'coriolis_lifetime> {

            pressure_loss: Pressure,
            mass_flowrate: MassRate,
            internal_pressure: Pressure,
            hydraulic_diameter: Length,
            incline_angle: Angle,
            component_length: Length,
            fluid_density: MassDensity,
            fluid_viscosity: DynamicViscosity,
            absolute_roughness: Length,
            custom_darcy: &'coriolis_lifetime dyn Fn(f64, f64) -> f64,
            custom_k: &'coriolis_lifetime dyn Fn(f64) -> f64,

        }

        impl <'coriolis_lifetime> FluidCustomComponentCalcPressureChange
            <'coriolis_lifetime> for 
            CoriolisFlowmeter <'coriolis_lifetime> {

        }

        impl <'coriolis_lifetime> FluidCustomComponentCalcPressureLoss
            <'coriolis_lifetime> for 
            CoriolisFlowmeter <'coriolis_lifetime>{

            fn get_custom_component_absolute_roughness(
                &mut self) -> Length {

                return self.absolute_roughness;
            }

            fn get_custom_component_absolute_roughness_immutable(
                &self) -> Length {

                return self.absolute_roughness;
            }

            fn get_custom_darcy(&mut self) 
                -> &dyn Fn(f64, f64) -> f64 {

                    return self.custom_darcy.clone();

            }


            fn get_custom_darcy_immutable(&self) 
                -> &dyn Fn(f64, f64) -> f64 {

                    return self.custom_darcy.clone();

            }

            fn get_custom_k(&mut self) 
                -> &dyn Fn(f64) -> f64 {

                    return self.custom_k.clone();

            }
            
            fn get_custom_k_immutable(&self) 
                -> &dyn Fn(f64) -> f64 {

                    return self.custom_k.clone();

            }

            fn set_custom_k(
                &mut self,
                custom_k: &'coriolis_lifetime dyn Fn(f64) -> f64){

                self.custom_k = custom_k;

            }

            fn set_custom_darcy(
                &mut self,
                custom_darcy: &'coriolis_lifetime dyn Fn(f64,f64) -> f64){

                self.custom_darcy = custom_darcy;
            }


        }

        impl <'coriolis_lifetime> FluidComponent for 
            CoriolisFlowmeter <'coriolis_lifetime>{
            fn set_internal_pressure_source(
                &mut self,
                internal_pressure: Pressure) {

                self.internal_pressure = internal_pressure;
            }


            fn get_internal_pressure_source(
                &mut self) -> Pressure{
                return self.internal_pressure;
            }

            fn get_internal_pressure_source_immutable(
                &self) -> Pressure{
                return self.internal_pressure;
            }

            fn set_mass_flowrate(
                &mut self,
                mass_flowrate: MassRate){

                self.mass_flowrate = mass_flowrate;
            }

            fn set_pressure_loss(
                &mut self,
                pressure_loss: Pressure){
                self.pressure_loss = pressure_loss;
            }

            fn get_hydraulic_diameter(&mut self) -> Length{

                return self.hydraulic_diameter;

            }

            fn get_hydraulic_diameter_immutable(
                &self) -> Length{

                return self.hydraulic_diameter;

            }

            fn get_incline_angle(&mut self) -> Angle {

                return self.incline_angle;

            }

            fn get_incline_angle_immutable(&self) -> Angle {

                return self.incline_angle;

            }

            fn get_component_length(&mut self) -> Length {

                return self.component_length;
            }

            fn get_component_length_immutable(&self) -> Length {

                return self.component_length;
            }

            fn get_fluid_density(&mut self) -> MassDensity {

                return self.fluid_density;
                
            }

            fn get_fluid_density_immutable(&self) -> MassDensity {

                return self.fluid_density;
                
            }

            fn get_fluid_viscosity(&mut self) -> DynamicViscosity {

                return self.fluid_viscosity;

            }

            fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {

                return self.fluid_viscosity;

            }

            fn get_cross_sectional_area(&mut self) -> Area {

                return self.get_hydraulic_diameter()*
                    self.get_hydraulic_diameter()*
                    PI/4.0_f64;

            }

            fn get_cross_sectional_area_immutable(&self) -> Area {

                return self.get_hydraulic_diameter()*
                    self.get_hydraulic_diameter()*
                    PI/4.0_f64;

            }


            /// gets pressure loss given current state of
            /// the component 
            fn get_pressure_loss(&mut self) -> Pressure {

                let fluid_mass_flowrate = 
                    self.mass_flowrate;

                let cross_sectional_area = 
                    self.get_cross_sectional_area();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter();

                let fluid_viscosity = 
                    self.get_fluid_viscosity();

                let fluid_density = 
                    self.get_fluid_density();

                let component_length = 
                    self.get_component_length();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                // i need to make some immutable borrows here...
                let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;

                let pressure_loss =
                    CoriolisFlowmeter::<'coriolis_lifetime>::
                    fluid_custom_component_calc_pressure_loss(
                    fluid_mass_flowrate, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    component_length, 
                    absolute_roughness, 
                    custom_darcy, custom_k);

                self.pressure_loss = pressure_loss;

                return pressure_loss;

            }
            
            /// gets pressure loss given current state
            /// of the system except for mass flowrate
            /// with an immutable borrow of self
            fn get_pressure_loss_immutable(
                &self,
                mass_flowrate: MassRate) -> Pressure {

                let fluid_mass_flowrate = 
                    self.mass_flowrate;

                let cross_sectional_area = 
                    self.get_cross_sectional_area();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter();

                let fluid_viscosity = 
                    self.get_fluid_viscosity();

                let fluid_density = 
                    self.get_fluid_density();

                let component_length = 
                    self.get_component_length();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                // i need to make some immutable borrows here...
                let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;

                let pressure_loss =
                    CoriolisFlowmeter::<'coriolis_lifetime>::
                    fluid_custom_component_calc_pressure_loss(
                    fluid_mass_flowrate, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    component_length, 
                    absolute_roughness, 
                    custom_darcy, custom_k);

                self.pressure_loss = pressure_loss;

                return pressure_loss;

            }

            /// gets mass flowrate given current state
            /// of the pipe
            fn get_mass_flowrate(&mut self) -> MassRate {

                //i'll have to get the pressure change
                //
                // pressure_change = 
                // - pressure_change
                // + hydrostatic pressure change
                // + internal pressure source
                //
                
                // internal pressure source
                let internal_pressure_source = 
                    self.get_internal_pressure_source();

                // hydrostatic pressure
                let component_length =
                    self.get_component_length();

                let incline_angle = 
                    self.get_incline_angle();

                let fluid_density =
                    self.get_fluid_density();

                let hydrostatic_pressure_change =
                    self.get_hydrostatic_pressure_change(
                        component_length, 
                        incline_angle, 
                        fluid_density);

                // pressure_loss term
                //
                //
                let pressure_loss = 
                    self.get_pressure_loss();

                // now we get pressure change

                let pressure_change =
                    - pressure_loss
                    + hydrostatic_pressure_change
                    + internal_pressure_source;

                let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;


                let cross_sectional_area = 
                    self.get_cross_sectional_area();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter();

                let fluid_viscosity = 
                    self.get_fluid_viscosity();

                let fluid_density = 
                    self.get_fluid_density();

                let component_length = 
                    self.get_component_length();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                let source_pressure = 
                    self.get_internal_pressure_source();

                let mass_flowrate =
                    CoriolisFlowmeter::
                    <'coriolis_lifetime>::
                    fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                    pressure_change, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    component_length, 
                    absolute_roughness, 
                    incline_angle, 
                    source_pressure, 
                    custom_darcy, 
                    custom_k);

                self.mass_flowrate = mass_flowrate;

                return mass_flowrate;
            }

            /// gets mass flowrate given current state of the pipe
            /// except for pressure loss
            fn get_mass_flowrate_from_pressure_loss_immutable(
                &self,
                pressure_loss: Pressure) -> MassRate {

                //i'll have to get the pressure change
                //
                // pressure_change = 
                // - pressure_change
                // + hydrostatic pressure change
                // + internal pressure source
                //
                
                // internal pressure source
                let internal_pressure_source = 
                    self.get_internal_pressure_source();

                // hydrostatic pressure
                let component_length =
                    self.get_component_length();

                let incline_angle = 
                    self.get_incline_angle();

                let fluid_density =
                    self.get_fluid_density();

                let hydrostatic_pressure_change =
                    self.get_hydrostatic_pressure_change(
                        component_length, 
                        incline_angle, 
                        fluid_density);


                // now we get pressure change

                let pressure_change =
                    - pressure_loss
                    + hydrostatic_pressure_change
                    + internal_pressure_source;

                let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
                    self.custom_darcy;

                let custom_k : &dyn Fn(f64) -> f64 =
                    self.custom_k;


                let cross_sectional_area = 
                    self.get_cross_sectional_area_immutable();

                let hydraulic_diameter = 
                    self.get_hydraulic_diameter_immutable();

                let fluid_viscosity = 
                    self.get_fluid_viscosity_immutable();

                let fluid_density = 
                    self.get_fluid_density_immutable();

                let component_length = 
                    self.get_component_length_immutable();

                let absolute_roughness = 
                    self.get_custom_component_absolute_roughness();

                let source_pressure = 
                    self.get_internal_pressure_source_immutable();

                let mass_flowrate =
                    CoriolisFlowmeter::
                    <'coriolis_lifetime>::
                    fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                    pressure_change, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    component_length, 
                    absolute_roughness, 
                    incline_angle, 
                    source_pressure, 
                    custom_darcy, 
                    custom_k);

                return mass_flowrate;
            }
        }

        impl <'coriolis_lifetime> CoriolisFlowmeter <'coriolis_lifetime> {

            fn new(hydraulic_diameter: Length,
                   incline_angle: Angle,
                   component_length: Length,
                   absolute_roughness: Length,
                   custom_darcy: &'coriolis_lifetime dyn Fn(f64, f64) -> f64,
                   custom_k: &'coriolis_lifetime dyn Fn(f64) -> f64 ) -> Self {

                // by default, i set pressure loss and mass flowrate to 0 
                // internal pressure also set to 0
                return Self { 
                    pressure_loss: Pressure::new::<pascal>(0.0), 
                    mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
                    internal_pressure: Pressure::new::<pascal>(0.0),
                    hydraulic_diameter: hydraulic_diameter, 
                    incline_angle: incline_angle, 
                    component_length: component_length, 
                    fluid_density: MassDensity::new::<kilogram_per_cubic_meter>(1000.0), 
                    fluid_viscosity: DynamicViscosity::new::<poise>(0.01),
                    absolute_roughness: absolute_roughness,
                    custom_darcy: custom_darcy, 
                    custom_k: custom_k,
                }
            }
        }

        // now we have defined our coriolis flowmeter with water, we can start!

        let hydraulic_diameter = 
            Length::new::<inch>(1.0);

        let incline_angle = 
            Angle::new::<degree>(90.0);

        let component_length = 
            Length::new::<meter>(0.5);

        let absolute_roughness = 
            Length::new::<millimeter>(0.001);

        fn custom_darcy(_reynolds_number: f64,
                        _roughness_ratio:f64) -> f64 {

            return 0.0;
        }

        fn custom_k(reynolds_number: f64) -> f64 {

            // the correlation is:
            // 18.0 + 93000/Re^1.35
            //

            if reynolds_number > 0.0 {

                return 18.0 + 93000_f64/reynolds_number.powf(1.35);
            }

            if reynolds_number < 0.0 {

                let abs_reynolds_number = reynolds_number.abs();
                let fldk = 18.0 + 93000_f64/abs_reynolds_number.powf(1.35);

                return -fldk;

            }

            // return no fldk = 0 for no flow, doesn't really matter anyway
            // because Be_D = 0.5 fldk * Re^2
            // Re = 0
            // and we expect Be_D = 0
            // so fldk can be 0 and it still makes physical sense
            // ie Be_D = 0 when Re = 0
            return 0.0;

        }

        let mut flowmeter_object = 
            CoriolisFlowmeter::new(
                hydraulic_diameter, 
                incline_angle, 
                component_length, 
                absolute_roughness, 
                &custom_darcy, 
                &custom_k);

        
        // set mass flowrate at 0.2 kg/s

        let mut mass_flowrate = MassRate::new::<kilogram_per_second>(0.2);

        flowmeter_object.set_mass_flowrate(mass_flowrate);

        let mut pressure_change = 
            flowmeter_object.get_pressure_change();

        // expected pressure loss is 1430 pascals
        // expected pressure change is -6335 pascals
        // becuase of elevation
        approx::assert_relative_eq!(
            -6335_f64,
            pressure_change.value,
            max_relative=0.01);

        // we'll now test the mass flowrate portion

        pressure_change = Pressure::new::<pascal>(-6335_f64);

        flowmeter_object.set_pressure_change(pressure_change);

        mass_flowrate = flowmeter_object.get_mass_flowrate();


        approx::assert_relative_eq!(
            0.2,
            mass_flowrate.value,
            max_relative=0.01);

        // now we can get pressure loss in both direction
        // one should be the negative value of the other if
        // done correctly...

        flowmeter_object.set_mass_flowrate(
            MassRate::new::<kilogram_per_second>(0.2));

        let pressure_loss_positive_direction = 
            flowmeter_object.get_pressure_loss();

        flowmeter_object.set_mass_flowrate(
            MassRate::new::<kilogram_per_second>(-0.2));

        let pressure_loss_negative_direction = 
            flowmeter_object.get_pressure_loss();

        approx::assert_relative_eq!(
            pressure_loss_positive_direction.value,
            1430_f64,
            max_relative=0.01);

        approx::assert_relative_eq!(
            pressure_loss_positive_direction.value,
            -pressure_loss_negative_direction.value,
            max_relative=0.01);


    }
}

