#[cfg(test)]
pub mod fluid_component_collection_test_and_examples {

    use std::f64::consts::PI;

    use crate::fluid_component_calculation::FluidComponent;
    use crate::fluid_component_calculation::standard_pipe_calc
        ::{FluidPipeCalcPressureLoss};
    use crate::fluid_component_collection::{
        FluidComponentCollection, FluidComponentCollectionMethods,
        FluidComponentCollectionSeriesAssociatedFunctions, FluidComponentCollectionParallelAssociatedFunctions};
    use uom::si::dynamic_viscosity::{millipascal_second};
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::{pascal};
    use uom::si::angle::degree;


    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component collections
    ///
    /// First i make a typical fluid component, a set of air pipes
    /// perhaps 10 air pipes and i want to put them in series
    #[test]
    pub fn simple_fluid_collection_example_1 () {

        
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


        // with the AirPipe struct setup, you can caluclate
        // the pressure loss easily


        let air_pipe_1 = AirPipe::new();
        let air_pipe_2 = AirPipe::new();
        let air_pipe_3 = AirPipe::new();
        let air_pipe_4 = AirPipe::new();
        let air_pipe_5 = AirPipe::new();
        let air_pipe_6 = AirPipe::new();
        let air_pipe_7 = AirPipe::new();
        let air_pipe_8 = AirPipe::new();
        let air_pipe_9 = AirPipe::new();
        let air_pipe_10 = AirPipe::new();

        // next i make a struct of 
        struct AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fluid_component_vector_immutable: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>
        }

        impl<'air_pipe_collection_lifetime> 
            FluidComponentCollection<'air_pipe_collection_lifetime>
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {


            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'air_pipe_collection_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

        }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionMethods for
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
                fn get_pressure_change(
                    &self,
                    fluid_mass_flowrate: MassRate) -> Pressure {

                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let pressure_change = 
                        Self::calculate_pressure_change_from_mass_flowrate(
                            fluid_mass_flowrate, immutable_vector_ref);

                    return pressure_change;
                }

                fn get_mass_flowrate_from_pressure_change(
                    &self,
                    pressure_change: Pressure) -> MassRate {


                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let mass_flowrate = 
                        Self::calculate_mass_flowrate_from_pressure_change(
                            pressure_change, immutable_vector_ref);

                    return mass_flowrate;

                }


            }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionSeriesAssociatedFunctions
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {}

        // constructor is here

        impl<'air_pipe_collection_lifetime>
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fn new() -> Self {
                return Self { 
                    fluid_component_vector_immutable:  vec![]
                };
            }
        }

        
        let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];

        air_pipe_vec.push(&air_pipe_1);
        air_pipe_vec.push(&air_pipe_2);
        air_pipe_vec.push(&air_pipe_3);
        air_pipe_vec.push(&air_pipe_4);
        air_pipe_vec.push(&air_pipe_5);
        air_pipe_vec.push(&air_pipe_6);
        air_pipe_vec.push(&air_pipe_7);
        air_pipe_vec.push(&air_pipe_8);
        air_pipe_vec.push(&air_pipe_9);
        air_pipe_vec.push(&air_pipe_10);

        // now i've made my air pipe vector, i can push it into the air pipe collection
        let mut air_pipe_series = 
            AirPipeCollectionSeries::new();

        air_pipe_series.set_fluid_component_vector(air_pipe_vec);

        // now let's push a 0.1kg/s airflow through this pipe series
        //
        let pipe_airflow = MassRate::new::<kilogram_per_second>(0.1);

        // and then let's get the pressure change

        let pipe_pressure_change = air_pipe_series.
            get_pressure_change(pipe_airflow);

        // the pressure losses are about -1144 Pa
        approx::assert_relative_eq!(
            pipe_pressure_change.value,
            -174650.0,
            max_relative=0.001);

        // i will also test the get pressure loss function

        let pipe_pressure_loss = air_pipe_series.
            get_pressure_loss(pipe_airflow);

        // in this case, there is no elevation or internal
        // pressure source, so the pipe pressure losses should
        // be the same as the inverse of the pressure change
        assert_eq!(-pipe_pressure_change,
                   pipe_pressure_loss);

        // all right, so now we want to check if the same pressure loss
        // will yield us 0.001 kg/s

        let test_pressure_loss = 
            Pressure::new::<pascal>(174650.0);

        let pipe_test_air_mass_flowrate = 
            air_pipe_series.get_mass_flowrate_from_pressure_change(
                -test_pressure_loss);

        approx::assert_relative_eq!(
            pipe_airflow.value,
            pipe_test_air_mass_flowrate.value,
            max_relative=0.001);


        // the last thing to assert is whether the pressure loss of 1 pipe
        // is equal to 1/10 of the pipes in series

        let pressure_loss_1_pipe =
            air_pipe_1.get_pressure_change_immutable(pipe_airflow);

        let pressure_loss_10_pipe_series =
            air_pipe_series.get_pressure_change(pipe_airflow);

        approx::assert_relative_eq!(
            pressure_loss_1_pipe.value,
            pressure_loss_10_pipe_series.value/10.0,
            max_relative=1e-3);



        return;

    }

    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component collections
    ///
    /// First i make a typical fluid component, a set of air pipes
    /// perhaps 10 air pipes and i want to put them in parallel
    #[test]
    pub fn simple_fluid_collection_example_2 () {

        
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


        // with the AirPipe struct setup, you can caluclate
        // the pressure loss easily


        let air_pipe_1 = AirPipe::new();
        let air_pipe_2 = AirPipe::new();
        let air_pipe_3 = AirPipe::new();
        let air_pipe_4 = AirPipe::new();
        let air_pipe_5 = AirPipe::new();
        let air_pipe_6 = AirPipe::new();
        let air_pipe_7 = AirPipe::new();
        let air_pipe_8 = AirPipe::new();
        let air_pipe_9 = AirPipe::new();
        let air_pipe_10 = AirPipe::new();

        // next i make a struct of 
        struct AirPipeCollectionParallel<'air_pipe_collection_lifetime> {
            fluid_component_vector_immutable: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>
        }

        impl<'air_pipe_collection_lifetime> 
            FluidComponentCollection<'air_pipe_collection_lifetime>
            for AirPipeCollectionParallel<'air_pipe_collection_lifetime> {


            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'air_pipe_collection_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

        }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionMethods for
            AirPipeCollectionParallel<'air_pipe_collection_lifetime> {
                fn get_pressure_change(
                    &self,
                    fluid_mass_flowrate: MassRate) -> Pressure {


                    // ALWAYS handle the zero mass flowrate case first
                    if fluid_mass_flowrate.value == 0.0_f64 {
                        return Pressure::new::<pascal>(0.0);
                    }

                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let pressure_change = 
                        <Self as FluidComponentCollectionParallelAssociatedFunctions>
                        ::calculate_pressure_change_from_mass_flowrate(
                            fluid_mass_flowrate, immutable_vector_ref);

                    return pressure_change;
                }

                fn get_mass_flowrate_from_pressure_change(
                    &self,
                    pressure_change: Pressure) -> MassRate {


                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let mass_flowrate = 
                        <Self as FluidComponentCollectionParallelAssociatedFunctions>
                        ::calculate_mass_flowrate_from_pressure_change(
                            pressure_change, immutable_vector_ref);

                    return mass_flowrate;

                }


            }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionParallelAssociatedFunctions
            for AirPipeCollectionParallel<'air_pipe_collection_lifetime> {}

        // constructor is here

        impl<'air_pipe_collection_lifetime>
            AirPipeCollectionParallel<'air_pipe_collection_lifetime> {
            fn new() -> Self {
                return Self { 
                    fluid_component_vector_immutable:  vec![]
                };
            }
        }

        
        let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];

        air_pipe_vec.push(&air_pipe_1);
        air_pipe_vec.push(&air_pipe_2);
        air_pipe_vec.push(&air_pipe_3);
        air_pipe_vec.push(&air_pipe_4);
        air_pipe_vec.push(&air_pipe_5);
        air_pipe_vec.push(&air_pipe_6);
        air_pipe_vec.push(&air_pipe_7);
        air_pipe_vec.push(&air_pipe_8);
        air_pipe_vec.push(&air_pipe_9);
        air_pipe_vec.push(&air_pipe_10);

        // now i've made my air pipe vector, i can push it into the air pipe collection
        let mut air_pipe_parallel = 
            AirPipeCollectionParallel::new();

        air_pipe_parallel.set_fluid_component_vector(air_pipe_vec);

        // now let's push a 1000 Pa pressure loss through one of the pipes
        // and through all of the pipes
        //
        let pressure_loss_specified = 
            Pressure::new::<pascal>(1000_f64);

        let pipe_reference_mass_flowrate = 
            air_pipe_1.get_mass_flowrate_from_pressure_loss_immutable(
                pressure_loss_specified);

        // the expected mass flowrate from this
        // pressure loss is about 0.00841 kg/s

        approx::assert_relative_eq!(
            pipe_reference_mass_flowrate.value,
            0.00841,
            max_relative=0.001);

        // now let's test the get pressure_loss from this value,
        
        approx::assert_relative_eq!(
            pressure_loss_specified.value,
            &air_pipe_1.get_pressure_loss_immutable(pipe_reference_mass_flowrate).value,
            max_relative=0.001);
        
        // now i also want to push a -1000 Pa pressure loss through one of the pipes
        // and see  the results
        //
        approx::assert_relative_eq!(
            -pressure_loss_specified.value,
            &air_pipe_1.get_pressure_loss_immutable(-pipe_reference_mass_flowrate).value,
            max_relative=0.001);

        

        // and then let's get the mass flowrate from the pipe and
        // assert that it is 10 times the flow of one pipe
        // the value is approximately 0.0841 kg/s

        let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
            get_mass_flowrate_from_pressure_loss(
                pressure_loss_specified);

        approx::assert_relative_eq!(
            pipe_reference_mass_flowrate.value*10.0,
            pipe_parallel_collection_mass_flowrate.value,
            max_relative=0.001);

        approx::assert_relative_eq!(
            0.0841,
            pipe_parallel_collection_mass_flowrate.value,
            max_relative=0.001);

        // this result should be the same as specifying a -1000 Pa pressure change
        // becuase i set no elevation for this

        let pipe_parallel_collection_mass_flowrate_2 = air_pipe_parallel.
            get_mass_flowrate_from_pressure_change(
                -pressure_loss_specified);

        approx::assert_relative_eq!(
            0.0841,
            pipe_parallel_collection_mass_flowrate_2.value,
            max_relative=0.001);

        // now we can get the pressure change from mass flowrate
        // which should be -1000 Pa approximately
        //
        // note that pressure loss here is defined as the pressure
        // difference between the fluid component collection at zero flow
        // and the fluid component collection at user specified net flow

        // getting guesses for this is complex, so i want to test for 
        // zero flow first
        let pipe_parallel_collection_pressure_change = 
            air_pipe_parallel
            .get_pressure_change(MassRate::new::<kilogram_per_second>(0.0));

        // pressure change should be zero here

        approx::assert_relative_eq!(
            0.0,
            pipe_parallel_collection_pressure_change.value,
            max_relative=0.001);
        

        // since the zero mass flow situation works,
        // we can proceed
        //
        // and the pressure should be about -1000 Pa

        // this section tests if my get mass flowrate from pressure change works
        // for front flow and backflow
        {

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(-1000.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                0.0841,
                max_relative=0.001);

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(-995.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                0.0837,
                max_relative=0.001);

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(-1005.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                0.0845,
                max_relative=0.001);

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(1000.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                -0.0841,
                max_relative=0.001);

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(995.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                -0.0837,
                max_relative=0.001);

            let pipe_parallel_collection_mass_flowrate = air_pipe_parallel.
                get_mass_flowrate_from_pressure_change(
                    Pressure::new::<pascal>(1005.0));

            approx::assert_relative_eq!(
                pipe_parallel_collection_mass_flowrate.value,
                -0.0845,
                max_relative=0.001);
        }

        
        let pipe_parallel_collection_pressure_change =
            air_pipe_parallel
            .get_pressure_change(
                MassRate::new::<kilogram_per_second>(0.0841));

        // the pressure change here should be about -1000 Pa
        approx::assert_relative_eq!(
            pipe_parallel_collection_pressure_change.value,
            -1000_f64,
            max_relative=0.001);

        return;

    }

    /// Here is a test which is meant to test a simple struct made
    /// to hold and calculate fluid component super collections
    ///
    /// Super collections are basically struts which contain
    /// vectors of fluid component collections,
    ///
    /// take example a parallel pipe setup which has three
    /// branches, each has about 10-20 fluid components or pipes
    ///
    /// we may want to calculate pressure change and flow across each 
    /// of these branches if they are connected in parallel.
    ///
    /// This is what i would call a super collection
    ///
    /// First i make a typical fluid component, a set of air pipes
    /// perhaps 10 air pipes and i want to put them in series
    #[test]
    pub fn simple_fluid_collection_example_3 () {

        
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


        // with the AirPipe struct setup, you can caluclate
        // the pressure loss easily


        let air_pipe_1 = AirPipe::new();
        let air_pipe_2 = AirPipe::new();
        let air_pipe_3 = AirPipe::new();
        let air_pipe_4 = AirPipe::new();
        let air_pipe_5 = AirPipe::new();
        let air_pipe_6 = AirPipe::new();
        let air_pipe_7 = AirPipe::new();
        let air_pipe_8 = AirPipe::new();
        let air_pipe_9 = AirPipe::new();
        let air_pipe_10 = AirPipe::new();

        // next i make a struct of 
        struct AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fluid_component_vector_immutable: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>
        }

        impl<'air_pipe_collection_lifetime> 
            FluidComponentCollection<'air_pipe_collection_lifetime>
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {


            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'air_pipe_collection_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'air_pipe_collection_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

        }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionMethods for
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
                fn get_pressure_change(
                    &self,
                    fluid_mass_flowrate: MassRate) -> Pressure {

                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let pressure_change = 
                        Self::calculate_pressure_change_from_mass_flowrate(
                            fluid_mass_flowrate, immutable_vector_ref);

                    return pressure_change;
                }

                fn get_mass_flowrate_from_pressure_change(
                    &self,
                    pressure_change: Pressure) -> MassRate {


                    // first we get the vector

                    let immutable_vector_ref = 
                        self.get_immutable_fluid_component_vector();

                    // second we use the associated function

                    let mass_flowrate = 
                        Self::calculate_mass_flowrate_from_pressure_change(
                            pressure_change, immutable_vector_ref);

                    return mass_flowrate;

                }


            }

        impl<'air_pipe_collection_lifetime> FluidComponentCollectionSeriesAssociatedFunctions
            for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {}

        // constructor is here

        impl<'air_pipe_collection_lifetime>
            AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
            fn new() -> Self {
                return Self { 
                    fluid_component_vector_immutable:  vec![]
                };
            }
        }

        
        let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];

        air_pipe_vec.push(&air_pipe_1);
        air_pipe_vec.push(&air_pipe_2);
        air_pipe_vec.push(&air_pipe_3);
        air_pipe_vec.push(&air_pipe_4);
        air_pipe_vec.push(&air_pipe_5);
        air_pipe_vec.push(&air_pipe_6);
        air_pipe_vec.push(&air_pipe_7);
        air_pipe_vec.push(&air_pipe_8);
        air_pipe_vec.push(&air_pipe_9);
        air_pipe_vec.push(&air_pipe_10);

        // now i've made my air pipe vector, i can push it into the air pipe collection
        let mut air_pipe_series = 
            AirPipeCollectionSeries::new();

        air_pipe_series.set_fluid_component_vector(air_pipe_vec);

        // now let's push a 0.1kg/s airflow through this pipe series
        //
        let pipe_airflow = MassRate::new::<kilogram_per_second>(0.1);

        // and then let's get the pressure change

        let pipe_pressure_change = air_pipe_series.
            get_pressure_change(pipe_airflow);

        // the pressure losses are about -1144 Pa
        approx::assert_relative_eq!(
            pipe_pressure_change.value,
            -174650.0,
            max_relative=0.001);

        // i will also test the get pressure loss function

        let pipe_pressure_loss = air_pipe_series.
            get_pressure_loss(pipe_airflow);

        // in this case, there is no elevation or internal
        // pressure source, so the pipe pressure losses should
        // be the same as the inverse of the pressure change
        assert_eq!(-pipe_pressure_change,
                   pipe_pressure_loss);

        // all right, so now we want to check if the same pressure loss
        // will yield us 0.001 kg/s

        let test_pressure_loss = 
            Pressure::new::<pascal>(174650.0);

        let pipe_test_air_mass_flowrate = 
            air_pipe_series.get_mass_flowrate_from_pressure_change(
                -test_pressure_loss);

        approx::assert_relative_eq!(
            pipe_airflow.value,
            pipe_test_air_mass_flowrate.value,
            max_relative=0.001);


        // the last thing to assert is whether the pressure loss of 1 pipe
        // is equal to 1/10 of the pipes in series

        let pressure_loss_1_pipe =
            air_pipe_1.get_pressure_change_immutable(pipe_airflow);

        let pressure_loss_10_pipe_series =
            air_pipe_series.get_pressure_change(pipe_airflow);

        approx::assert_relative_eq!(
            pressure_loss_1_pipe.value,
            pressure_loss_10_pipe_series.value/10.0,
            max_relative=1e-3);



        return;

    }
}

