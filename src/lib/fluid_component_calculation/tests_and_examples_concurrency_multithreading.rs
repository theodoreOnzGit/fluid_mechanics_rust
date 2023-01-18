#[cfg(test)]
pub mod concurrency_tests {
    use std::f64::consts::PI;
    use std::thread;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::
        custom_component_calc::{FluidCustomComponentCalcPressureChange, FluidCustomComponentCalcPressureLoss};
    use crate::fluid_component_calculation::standard_pipe_calc
        ::{FluidPipeCalcPressureLoss,FluidPipeCalcPressureChange};
    use uom::num_traits::Zero;
    use uom::si::dynamic_viscosity::{millipascal_second, poise};
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal, kilopascal};
    use uom::si::angle::degree;
    
    /// Example 1: 
    ///
    ///
    /// Testing if fluid component structs can be put into threads with move closures
    #[test]
    pub fn thread_spawn_test () {

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
                    AirPipe::pipe_calc_mass_flowrate(
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
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let fluid_viscosity = self.get_fluid_viscosity_immutable();
                let fluid_density = self.get_fluid_density_immutable();
                let pipe_length = self.get_component_length_immutable();

                let mass_flowrate = 
                    AirPipe::pipe_calc_mass_flowrate(
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

                return mass_flowrate;
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
                    AirPipe::pipe_calc_pressure_loss(
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
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let viscosity = self.get_fluid_viscosity_immutable();
                let density = self.get_fluid_density_immutable();
                let pipe_legnth = self.get_component_length_immutable();


                // calculate the pressure loss

                let pressure_loss = 
                    AirPipe::pipe_calc_pressure_loss(
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

                return pressure_loss;
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

            fn get_pipe_form_loss_k_immutable(&self) -> f64 {
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

            fn get_pipe_absolute_roughness_immutable(&self) -> Length {
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


        // now make a new air pipe and move it into thread spawn

        let air_pipe = AirPipe::new();
        let mut air_pipe_mutable = AirPipe::new();

        let pressure = Pressure::new::<pascal>(100_f64);

        // i'm going to make an arbitrary function and move some values in

        let mut mass_flow = MassRate::zero();

        let f  = move || {
            // looks like instantiating structs here is ok
            let mut air_pipe_in_thread = AirPipe::new();

            // i want to bring a pressure loss in
            air_pipe_in_thread.set_pressure_loss(pressure);

            let local_mass_flow = 
                air_pipe_in_thread.get_mass_flowrate();

            println!("{:?}", local_mass_flow);
            
            // now let me bring in the mutable air pipe

            air_pipe_mutable.set_pressure_loss(pressure);
            let local_2_mass_flow = air_pipe_mutable.get_mass_flowrate();

            println!("{:?}", local_2_mass_flow);

            // and then let me bring in the air pipe

            mass_flow = air_pipe.get_mass_flowrate_from_pressure_loss_immutable(pressure);
            
        };


        let concurrent_thread = thread::spawn(f);

        concurrent_thread.join().unwrap();

        println!("{:?}", mass_flow);

    }

    /// Example 2: 
    ///
    ///
    /// Testing if fluid componnets can be put into threads with move closures
    /// as trait objects
    ///
    /// it shows you CANNOT bring trait objects into the thread
    #[test]
    pub fn thread_spawn_test_trait_objects () {

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
                    AirPipe::pipe_calc_mass_flowrate(
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
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let fluid_viscosity = self.get_fluid_viscosity_immutable();
                let fluid_density = self.get_fluid_density_immutable();
                let pipe_length = self.get_component_length_immutable();

                let mass_flowrate = 
                    AirPipe::pipe_calc_mass_flowrate(
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

                return mass_flowrate;
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
                    AirPipe::pipe_calc_pressure_loss(
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
                let form_loss_k = self.get_pipe_form_loss_k_immutable();
                let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
                let cross_sectional_area = self.get_cross_sectional_area_immutable();
                let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
                let viscosity = self.get_fluid_viscosity_immutable();
                let density = self.get_fluid_density_immutable();
                let pipe_legnth = self.get_component_length_immutable();


                // calculate the pressure loss

                let pressure_loss = 
                    AirPipe::pipe_calc_pressure_loss(
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

                return pressure_loss;
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

            fn get_pipe_form_loss_k_immutable(&self) -> f64 {
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

            fn get_pipe_absolute_roughness_immutable(&self) -> Length {
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


        // now make a new air pipe and move it into thread spawn

        let air_pipe = AirPipe::new();
        let mut air_pipe_mutable = AirPipe::new();
        // i'll make a trait object out of thread and move it in



        let pressure = Pressure::new::<pascal>(100_f64);

        // i'm going to make an arbitrary function and move some values in

        let mut mass_flow = MassRate::zero();

        let f  = move || {
            // looks like instantiating structs here is ok
            // and trait objects also
            let mut air_pipe_in_thread = AirPipe::new();
            let mut trait_object_in_thread: 
                &dyn FluidComponent = &mut air_pipe_in_thread;

            // i want to bring a pressure loss in
            air_pipe_in_thread.set_pressure_loss(pressure);

            let local_mass_flow = 
                air_pipe_in_thread.get_mass_flowrate();

            println!("{:?}", local_mass_flow);

            // and then let me bring in the immutable air pipe trait object
            // now we successfully get an error
            // trait objects cannot be brought in because they don't implement the Send Trait

            // let me see also if i can reference objects outside the thread
            // this is OK!
            let trait_object_immutable: &dyn FluidComponent = &air_pipe;
            
            // i can also make mutable trait objects within here:
            // so yes i can borrow these as mutable and dow hat i want here
            let trait_object_mutable: &mut dyn FluidComponent = &mut air_pipe_mutable;
            
            // now let me bring in the mutable air pipe
            // trait object

            air_pipe_mutable.set_pressure_loss(pressure);
            let local_2_mass_flow = air_pipe_mutable.get_mass_flowrate();

            println!("{:?}", local_2_mass_flow);

            
        };


        let concurrent_thread = thread::spawn(f);

        concurrent_thread.join().unwrap();

        println!("{:?}", mass_flow);

    }

}

