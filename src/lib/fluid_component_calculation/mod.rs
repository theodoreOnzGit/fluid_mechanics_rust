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


    /// gets pressure loss
    fn get_pressure_loss(&mut self) -> Pressure;

    /// sets the pressure loss of the component
    fn set_pressure_loss(&mut self, pressure_loss: Pressure);


    /// gets cross sectional area
    fn get_cross_sectional_area(&mut self) -> Area;


    /// gets hydraulic diamter
    fn get_hydraulic_diameter(&mut self) -> Length;

    /// gets fluid viscosity
    fn get_fluid_viscosity(&mut self) -> DynamicViscosity;

    /// gets fluid density
    fn get_fluid_density(&mut self) -> MassDensity;

    /// gets the component length
    fn get_component_length(&mut self) -> Length;


}



#[cfg(test)]
pub mod fluid_component_tests {
    use std::f64::consts::PI;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::standard_pipe_calc
        ::FluidPipeCalcPressureLoss;
    use uom::si::dynamic_viscosity::millipascal_second;
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal, kilopascal};
    
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

            /// sets the mass flowrate of the component
            fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
                self.mass_flowrate = mass_flowrate;
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

            /// gets hydraulic diamter
            /// im giving this pipe a two inch inner diameter 
            fn get_hydraulic_diameter(&mut self) -> Length {
                return Length::new::<inch>(2.0);
            }

            /// gets fluid viscosity
            /// air has a dynamic viscosity of about 18.6 millipascal
            /// seconds
            fn get_fluid_viscosity(&mut self) -> DynamicViscosity{ 
                return DynamicViscosity::new::<millipascal_second>(18.6);
            }

            /// gets fluid density
            /// air density is about 1kg/m3
            fn get_fluid_density(&mut self) -> MassDensity {
                return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
            }

            /// gets the component length
            /// you can set the component length here
            fn get_component_length(&mut self) -> Length {
                return Length::new::<meter>(1.0);
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


}

