//! Welcome to the fluid mechanics rust
//!
//! here is a library of traits meant to help calculate friction losses 
//!
//! in pipes and fluid components, as well as some of these components
//! connected in series or parallel configuration
//!
//! 
/// These traits help you create pipes
///
/// Here is one example of how that is done.
///
/// 
///
/// ```rust
/// extern crate fluid_mechanics_rust;
/// use fluid_mechanics_rust::prelude::*;
///
///    // we will implement a few properties here for our therminol pipe
///    // for clarity we will list them in a
///    // supertrait
///    // This makes it easy to see what traits are being implemented here
///
///    pub trait TherminolPipeTraits<'trait_lifetime> :
///        ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'trait_lifetime>
///        + FluidComponent
///        + FluidPipeCalcPressureChange
///        + FluidPipeCalcPressureLoss
///    {}
///    
///    // first we create an therminol pipe struct
///    // and start implementing it
///    struct TherminolPipe<'pipe_lifetime> {
///
///        therminol_properties_reference: &'pipe_lifetime dyn FluidProperties,
///        fluid_temp: ThermodynamicTemperature,
///        fluid_mass_flowrate: MassRate,
///
///        internal_pressure: Pressure,
///        incline_angle: Angle,
///        component_length: Length,
///        hydraulic_diameter: Length,
///
///        pressure_loss: Pressure,
///        form_loss_k: f64,
///        absolute_roughness: Length,
///
///    }
///
///    impl<'pipe_lifetime> 
///        TherminolPipeTraits<'pipe_lifetime> for TherminolPipe<'pipe_lifetime> {}
///
///    impl<'pipe_lifetime> 
///        FluidPipeCalcPressureChange for TherminolPipe<'pipe_lifetime> {
///        }
///
///    impl<'pipe_lifetime> 
///        FluidPipeCalcPressureLoss for TherminolPipe<'pipe_lifetime> {
///
///            fn get_pipe_form_loss_k(&mut self) -> f64 {
///                return self.form_loss_k;
///            }
///
///            fn get_pipe_form_loss_k_immutable(&self) -> f64 {
///                return self.form_loss_k;
///            }
///
///            /// return absolute roughness for pipe
///            /// for a typical copper pipe
///            /// it is 0.002 mm 
///            /// i did a web search
///            ///
///            fn get_pipe_absolute_roughness(&mut self) -> Length {
///                return self.absolute_roughness;
///            }
///
///            fn get_pipe_absolute_roughness_immutable(&self) -> Length {
///                return self.absolute_roughness;
///            }
///
///        }
///
///    impl<'pipe_lifetime> 
///        FluidComponent for TherminolPipe<'pipe_lifetime>{
///        fn get_pressure_loss(&mut self) -> Pressure {
///
///
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k();
///            let absolute_roughness = self.get_pipe_absolute_roughness();
///            let cross_sectional_area = self.get_cross_sectional_area();
///            let mass_flowrate = self.fluid_mass_flowrate;
///            let hydraulic_diameter = self.get_hydraulic_diameter();
///            let viscosity = self.get_fluid_viscosity();
///            let density = self.get_fluid_density();
///            let pipe_legnth = self.get_component_length();
///
///
///            // calculate the pressure loss
///
///            let pressure_loss = 
///                Self::pipe_calc_pressure_loss(
///                    mass_flowrate,
///                    cross_sectional_area,
///                    hydraulic_diameter,
///                    viscosity,
///                    density,
///                    pipe_legnth,
///                    absolute_roughness,
///                    form_loss_k);
///
///            // you can return the pressure loss straightaway
///            // or set the struct variable first and then
///            // return it
///
///            self.pressure_loss = pressure_loss;
///
///            return self.pressure_loss;
///        }
///
///        fn get_pressure_loss_immutable(
///            &self,
///            mass_flowrate: MassRate) -> Pressure {
///
///
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k_immutable();
///            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
///            let cross_sectional_area = self.get_cross_sectional_area_immutable();
///            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
///            let viscosity = self.get_fluid_viscosity_immutable();
///            let density = self.get_fluid_density_immutable();
///            let pipe_legnth = self.get_component_length_immutable();
///
///
///            // calculate the pressure loss
///
///            let pressure_loss = 
///                Self::pipe_calc_pressure_loss(
///                    mass_flowrate,
///                    cross_sectional_area,
///                    hydraulic_diameter,
///                    viscosity,
///                    density,
///                    pipe_legnth,
///                    absolute_roughness,
///                    form_loss_k);
///
///            // you can return the pressure loss straightaway
///            // or set the struct variable first and then
///            // return it
///
///
///            return pressure_loss;
///        }
///        fn set_pressure_loss(&mut self, pressure_loss: Pressure){
///            self.pressure_loss = pressure_loss;
///        }
///
///        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
///            self.fluid_mass_flowrate = mass_flowrate;
///        }
///
///        fn get_mass_flowrate(&mut self) -> MassRate {
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k();
///            let absolute_roughness = self.get_pipe_absolute_roughness();
///            let cross_sectional_area = self.get_cross_sectional_area();
///            let hydraulic_diameter = self.get_hydraulic_diameter();
///            let fluid_viscosity = self.get_fluid_viscosity();
///            let fluid_density = self.get_fluid_density();
///            let pipe_length = self.get_component_length();
///            let pressure_loss = self.pressure_loss;
///            let incline_angle = self.get_incline_angle();
///            let internal_pressure_source = self.get_internal_pressure_source();
///
///            let pressure_change = 
///                -pressure_loss 
///                + internal_pressure_source 
///                + self.get_hydrostatic_pressure_change();
///
///            let mass_flowrate = 
///                Self::pipe_calculate_mass_flowrate_from_pressure_change(
///                    pressure_change, 
///                    cross_sectional_area, 
///                    hydraulic_diameter, 
///                    fluid_viscosity, 
///                    fluid_density, 
///                    pipe_length, 
///                    absolute_roughness, 
///                    form_loss_k,
///                    incline_angle,
///                    internal_pressure_source);
///
///            // you can return the mass flowrate straightaway
///            // or set the struct variable first and then
///            // return it
///
///            self.set_mass_flowrate(mass_flowrate);
///
///            return self.fluid_mass_flowrate;
///
///        }
///
///        fn get_mass_flowrate_from_pressure_loss_immutable(
///            &self,
///            pressure_loss: Pressure) -> MassRate {
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k_immutable();
///            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
///            let cross_sectional_area = self.get_cross_sectional_area_immutable();
///            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
///            let fluid_viscosity = self.get_fluid_viscosity_immutable();
///            let fluid_density = self.get_fluid_density_immutable();
///            let pipe_length = self.get_component_length_immutable();
///            let incline_angle = self.get_incline_angle_immutable();
///            let internal_pressure_source = self.get_internal_pressure_source_immutable();
///
///            let pressure_change = 
///                -pressure_loss 
///                + internal_pressure_source 
///                + <Self as FluidPipeCalcPressureChange>::
///                get_hydrostatic_pressure_change(
///                    pipe_length,
///                    incline_angle,
///                    fluid_density);
///
///            let mass_flowrate = 
///                Self::pipe_calculate_mass_flowrate_from_pressure_change(
///                    pressure_change, 
///                    cross_sectional_area, 
///                    hydraulic_diameter, 
///                    fluid_viscosity, 
///                    fluid_density, 
///                    pipe_length, 
///                    absolute_roughness, 
///                    form_loss_k,
///                    incline_angle,
///                    internal_pressure_source);
///
///            // you can return the mass flowrate straightaway
///            // or set the struct variable first and then
///            // return it
///
///
///            return mass_flowrate;
///
///        }
///
///            fn get_cross_sectional_area(&mut self) -> Area {
///                return self.get_hydraulic_diameter()*
///                    self.get_hydraulic_diameter()*
///                    PI/4.0_f64;
///            }
///
///            fn get_cross_sectional_area_immutable(&self) -> Area {
///                return self.get_hydraulic_diameter_immutable()*
///                    self.get_hydraulic_diameter_immutable()*
///                    PI/4.0_f64;
///            }
///
///            fn get_hydraulic_diameter(&mut self) -> Length {
///
///                return self.hydraulic_diameter;
///
///            }
///
///            fn get_hydraulic_diameter_immutable(&self) -> Length {
///
///
///                return self.hydraulic_diameter;
///
///            }
///
///
///            fn get_fluid_viscosity(&mut self) -> DynamicViscosity {
///
///                // get fluid temp first
///                let fluid_temp = self.get_fluid_temp();
///
///                // then the fluid properties
///
///                let fluid_properties = self.get_fluid_properties();
///
///                // let's get viscosity
///
///                let fluid_viscosity = 
///                    Self::viscosity(fluid_temp, fluid_properties);
///
///                return fluid_viscosity;
///                
///
///            }
///
///            fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {
///
///
///                // get fluid temp first
///                let fluid_temp = self.get_fluid_temp();
///
///                // then the fluid properties
///
///                let fluid_properties = self.get_fluid_properties();
///
///                // let's get viscosity
///
///                let fluid_viscosity = 
///                    Self::viscosity(fluid_temp, fluid_properties);
///
///                return fluid_viscosity;
///                
///
///
///            }
///
///            fn get_fluid_density(&mut self) -> MassDensity {
///
///                // get fluid temp first
///                let fluid_temp = self.get_fluid_temp();
///
///                // then the fluid properties
///
///                let fluid_properties = self.get_fluid_properties();
///
///                // let's get density
///
///                let fluid_density = 
///                    Self::density(fluid_temp, fluid_properties);
///
///                return fluid_density;
///                
///
///            }
///
///            fn get_fluid_density_immutable(&self) -> MassDensity {
///
///
///                // get fluid temp first
///                let fluid_temp = self.get_fluid_temp();
///
///                // then the fluid properties
///
///                let fluid_properties = self.get_fluid_properties();
///
///                // let's get density
///
///                let fluid_density = 
///                    Self::density(fluid_temp, fluid_properties);
///
///                return fluid_density;
///                
///
///
///            }
///
///            fn get_component_length(&mut self) -> Length {
///
///                return self.component_length;
///            }
///
///            fn get_component_length_immutable(&self) -> Length {
///
///                return self.component_length;
///            }
///
///            fn get_incline_angle(&mut self) -> Angle {
///
///                return self.incline_angle;
///            }
///
///            fn get_incline_angle_immutable(&self) -> Angle {
///
///                return self.incline_angle;
///            }
///
///
///
///            fn get_internal_pressure_source(&mut self) -> Pressure {
///
///                return self.internal_pressure;
///            }
///
///            fn get_internal_pressure_source_immutable(&self) -> Pressure {
///
///                return self.internal_pressure;
///            }
///
///            fn set_internal_pressure_source(&mut self,
///                                            internal_pressure: Pressure){
///
///                self.internal_pressure = internal_pressure;
///            }
///
///        }
///
///    impl<'pipe_lifetime> 
///        ConstantCompositionSinglePhaseFluidPropertiesAssociatedFunctions<'pipe_lifetime>
///        for TherminolPipe<'pipe_lifetime>{
///
///            fn get_fluid_properties(&self) -> &'pipe_lifetime dyn FluidProperties {
///
///                return self.therminol_properties_reference;
///
///            }
///
///            fn set_fluid_properties(&mut self,
///                                    fluid_properties: &'pipe_lifetime dyn FluidProperties){
///
///                self.therminol_properties_reference = fluid_properties;
///
///            }
///
///            fn get_fluid_temp(&self) -> ThermodynamicTemperature {
///
///                return self.fluid_temp;
///
///            }
///
///            fn set_fluid_temp(&mut self,
///                              fluid_temp: ThermodynamicTemperature){
///
///                self.fluid_temp = fluid_temp;
///
///            }
///        }
///
///    impl<'pipe_lifetime> TherminolPipe<'pipe_lifetime>{
///
///        // let's implement a generic constructor
///        fn new(fluid_temp: ThermodynamicTemperature,
///               incline_angle: Angle,
///               component_length: Length,
///               hydraulic_diameter: Length,
///               form_loss_k: f64,
///               absolute_roughness: Length,
///               therminol_properties_reference: &'pipe_lifetime TherminolVP1Properties) -> Self {
///
///            return Self { 
///                therminol_properties_reference: therminol_properties_reference,
///                fluid_temp: fluid_temp, 
///                fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
///                internal_pressure: Pressure::new::<pascal>(0.0), 
///                incline_angle: incline_angle, 
///                component_length: component_length ,
///                hydraulic_diameter: hydraulic_diameter ,
///                pressure_loss: Pressure::new::<pascal>(0.0),
///                form_loss_k: form_loss_k ,
///                absolute_roughness: absolute_roughness,
///            };
///
///
///
///        }
///    }
///
///    // now to use this code, we need to define a few things
///
///    let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
///    let incline_angle = Angle::new::<degree>(0.0);
///    let component_length  = Length::new::<meter>(0.5);
///    let hydraulic_diameter = Length::new::<inch>(1.0);
///    let form_loss_k: f64 = 5.0;
///    let absolute_roughness = Length::new::<millimeter>(0.002);
///    let therminol_properties = TherminolVP1Properties::new();
///
///    // let's make a new therminol pipe
///
///    let therminol_pipe = 
///        TherminolPipe::new(fluid_temp, 
///                           incline_angle, 
///                           component_length, 
///                           hydraulic_diameter, 
///                           form_loss_k, 
///                           absolute_roughness, 
///                           &therminol_properties);
///
///    // pass 0.2 kg/s of therminol through
///
///    let pressure_change = 
///        therminol_pipe.get_pressure_change_immutable(
///            MassRate::new::<kilogram_per_second>(0.2));
///
///    // this should be equal to -413 Pa
///
///    approx::assert_relative_eq!(
///        -413_f64,
///        pressure_change.value,
///        max_relative = 0.001);
///
///    // now let's get the mass flowrate
///
///    let mass_flowrate = 
///        therminol_pipe.get_mass_flowrate_from_pressure_change_immutable(
///            Pressure::new::<pascal>(-413_f64));
///
///    approx::assert_relative_eq!(
///        0.2,
///        mass_flowrate.value,
///        max_relative = 0.001);
///
///
///
///
/// ```
/// You can find more examples in the unit tests in the source code
///
pub use crate::fluid_component_calculation::FluidComponent;
pub use crate::fluid_component_calculation::standard_pipe_calc::FluidPipeCalcPressureChange;
pub use crate::fluid_component_calculation::standard_pipe_calc::FluidPipeCalcPressureLoss;

/// these traits help you create components which do not exhibit pipe pressure losses
pub use crate::fluid_component_calculation::custom_component_calc::
{FluidCustomComponentCalcPressureLoss,FluidCustomComponentCalcPressureChange};

/// If you want to connect components or pipes in series or parallel, this
/// import is good for that
///
/// in fact, if you wanted to make a system with 4 parallel branches
/// and each branch has a series of fluid components,
/// this trait is meant to help with that
///
/// here's an example of how that's done
///
/// ```rust
/// extern crate fluid_mechanics_rust;
/// use fluid_mechanics_rust::prelude::*;
///
///    // there are three steps to this
///    // 1) make a pipe structure
///    // 2) make a series collection of pipe structures
///    // 3) make 3 branches of series collections for the pipe structures
///    //
///
///    
///    // first we create an air pipe struct
///    //
///    struct AirPipe {
///        mass_flowrate: MassRate,
///        pressure_loss: Pressure,
///    }
///
///    // we implement get and set methods for each of the 
///    // properties, you can set these properties in the constructor
///    // or you can simply return the appropriate values in the 
///    // functions
///    // 
///    // likewise, when you get the mass flowrate
///    // or density, you can invoke calculation methods straightaway
///    // 
///    // but for calculation methods, you can "inherit" the default
///    // trait implementations for a generic fluid pipe
///    impl FluidComponent for AirPipe {
///
///        /// gets the mass flowrate of the component
///        fn get_mass_flowrate(&mut self) -> MassRate {
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k();
///            let absolute_roughness = self.get_pipe_absolute_roughness();
///            let cross_sectional_area = self.get_cross_sectional_area();
///            let hydraulic_diameter = self.get_hydraulic_diameter();
///            let fluid_viscosity = self.get_fluid_viscosity();
///            let fluid_density = self.get_fluid_density();
///            let pipe_length = self.get_component_length();
///            let pressure_loss = self.pressure_loss;
///
///            let mass_flowrate = 
///                AirPipe::pipe_calc_mass_flowrate(
///                    pressure_loss, 
///                    cross_sectional_area, 
///                    hydraulic_diameter, 
///                    fluid_viscosity, 
///                    fluid_density, 
///                    pipe_length, 
///                    absolute_roughness, 
///                    form_loss_k);
///
///            // you can return the mass flowrate straightaway
///            // or set the struct variable first and then
///            // return it
///
///            self.set_mass_flowrate(mass_flowrate);
///
///            return self.mass_flowrate;
///        }
///
///        /// gets the mass flowrate of the component
///        /// with immutable instance of self
///        fn get_mass_flowrate_from_pressure_loss_immutable(
///            &self,
///            pressure_loss: Pressure) -> MassRate {
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k_immutable();
///            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
///            let cross_sectional_area = self.get_cross_sectional_area_immutable();
///            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
///            let fluid_viscosity = self.get_fluid_viscosity_immutable();
///            let fluid_density = self.get_fluid_density_immutable();
///            let pipe_length = self.get_component_length_immutable();
///
///            let mass_flowrate = 
///                AirPipe::pipe_calc_mass_flowrate(
///                    pressure_loss, 
///                    cross_sectional_area, 
///                    hydraulic_diameter, 
///                    fluid_viscosity, 
///                    fluid_density, 
///                    pipe_length, 
///                    absolute_roughness, 
///                    form_loss_k);
///
///            // you can return the mass flowrate straightaway
///            // or set the struct variable first and then
///            // return it
///
///            return mass_flowrate;
///        }
///
///        /// sets the mass flowrate of the component
///        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
///            self.mass_flowrate = mass_flowrate;
///        }
///
///
///        /// pressure change is accounts for total pressure
///        /// differential between start and end point of the pipe,
///        /// including hydrostatic pressure and any sources
///        /// which may contribute to the pressure, eg. pumps
///        /// 
///        /// pressure change = -pressure loss + hydrostatic pressure
///        fn get_pressure_change(&mut self) -> Pressure {
///
///            // for this, i have
///            // pressure change = -pressure loss + hydrostatic pressure
///            // + internal pressure
///            return -self.get_pressure_loss();
///        }
///
///
///        fn set_pressure_change(&mut self, pressure_change:Pressure) {
///            self.set_pressure_loss(-pressure_change);
///        }
///
///        /// gets pressure loss
///        /// i calculate pressure loss when i invoke this method
///        /// and the method comes from the 
///        /// FluidPipeCalcPressureLoss trait 
///        fn get_pressure_loss(&mut self) -> Pressure {
///
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k();
///            let absolute_roughness = self.get_pipe_absolute_roughness();
///            let cross_sectional_area = self.get_cross_sectional_area();
///            let mass_flowrate = self.mass_flowrate;
///            let hydraulic_diameter = self.get_hydraulic_diameter();
///            let viscosity = self.get_fluid_viscosity();
///            let density = self.get_fluid_density();
///            let pipe_legnth = self.get_component_length();
///
///
///            // calculate the pressure loss
///
///            let pressure_loss = 
///                AirPipe::pipe_calc_pressure_loss(
///                    mass_flowrate,
///                    cross_sectional_area,
///                    hydraulic_diameter,
///                    viscosity,
///                    density,
///                    pipe_legnth,
///                    absolute_roughness,
///                    form_loss_k);
///
///            // you can return the pressure loss straightaway
///            // or set the struct variable first and then
///            // return it
///
///            self.pressure_loss = pressure_loss;
///
///            return self.pressure_loss;
///        }
///
///        fn get_pressure_loss_immutable(
///            &self, mass_flowrate: MassRate) -> Pressure {
///
///            // get pipe parameters and flow conditions
///            // from the get methods
///            let form_loss_k = self.get_pipe_form_loss_k_immutable();
///            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
///            let cross_sectional_area = self.get_cross_sectional_area_immutable();
///            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
///            let viscosity = self.get_fluid_viscosity_immutable();
///            let density = self.get_fluid_density_immutable();
///            let pipe_legnth = self.get_component_length_immutable();
///
///
///            // calculate the pressure loss
///
///            let pressure_loss = 
///                AirPipe::pipe_calc_pressure_loss(
///                    mass_flowrate,
///                    cross_sectional_area,
///                    hydraulic_diameter,
///                    viscosity,
///                    density,
///                    pipe_legnth,
///                    absolute_roughness,
///                    form_loss_k);
///
///            // you can return the pressure loss straightaway
///            // or set the struct variable first and then
///            // return it
///
///            return pressure_loss;
///        }
///
///        /// sets the pressure loss of the component
///        fn set_pressure_loss(&mut self, pressure_loss: Pressure){
///            self.pressure_loss = pressure_loss;
///        }
///
///
///        /// gets cross sectional area
///        /// the inner diameter is 2 in
///        /// and the area is Pi*d^2/4
///        fn get_cross_sectional_area(&mut self) -> Area {
///            return self.get_hydraulic_diameter()*
///                self.get_hydraulic_diameter()*
///                PI/4.0_f64;
///        }
///
///        fn get_cross_sectional_area_immutable(&self) -> Area {
///            return self.get_hydraulic_diameter_immutable()*
///                self.get_hydraulic_diameter_immutable()*
///                PI/4.0_f64;
///        }
///
///        /// gets hydraulic diamter
///        /// im giving this pipe a two inch inner diameter 
///        fn get_hydraulic_diameter(&mut self) -> Length {
///            return Length::new::<inch>(2.0);
///        }
///
///        fn get_hydraulic_diameter_immutable(&self) -> Length {
///            return Length::new::<inch>(2.0);
///        }
///
///        /// gets fluid viscosity
///        /// air has a dynamic viscosity of about 18.6 millipascal
///        /// seconds
///        fn get_fluid_viscosity(&mut self) -> DynamicViscosity{ 
///            return DynamicViscosity::new::<millipascal_second>(18.6);
///        }
///
///        fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity{ 
///            return DynamicViscosity::new::<millipascal_second>(18.6);
///        }
///
///
///        /// gets fluid density
///        /// air density is about 1kg/m3
///        fn get_fluid_density(&mut self) -> MassDensity {
///            return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
///        }
///
///        fn get_fluid_density_immutable(&self) -> MassDensity {
///            return MassDensity::new::<kilogram_per_cubic_meter>(1.0);
///        }
///
///        /// gets the component length
///        /// you can set the component length here
///        fn get_component_length(&mut self) -> Length {
///            return Length::new::<meter>(1.0);
///        }
///
///        fn get_component_length_immutable(&self) -> Length {
///            return Length::new::<meter>(1.0);
///        }
///
///        /// i'm manually fixing the incline angle at zero
///        /// meaning that this pipe is horizontal
///        fn get_incline_angle(&mut self) -> Angle {
///            return Angle::new::<degree>(0.0);
///        }
///        
///        fn get_incline_angle_immutable(&self) -> Angle {
///            return Angle::new::<degree>(0.0);
///        }
///
///        /// For the air pipe, there should be no internal source
///
///        fn get_internal_pressure_source(&mut self) -> Pressure {
///            return Pressure::new::<pascal>(0.0);
///        }
///
///        fn get_internal_pressure_source_immutable(&self) -> Pressure {
///            return Pressure::new::<pascal>(0.0);
///        }
///
///        fn set_internal_pressure_source(
///            &mut self, 
///            _internal_pressure_source: Pressure
///            ){
///            // doesn't actually do anything,
///            // i refuse to let it set anything
///            //
///            // rather i have it panic a special kind of panic
///            // called unimplemented
///
///            unimplemented!();
///
///        }
///
///
///    }
///
///
///    // we can "inherit" methods for the pipe pressure loss
///    // and mass flowrate calculations 
///    //
///    // all you need to do is set form loss K
///    // and absolute roughness
///
///    impl FluidPipeCalcPressureLoss for AirPipe {
///
///        /// return form loss K for the pipe
///        /// i set it at 5
///        ///
///        fn get_pipe_form_loss_k(&mut self) -> f64 {
///            return 5.0;
///        }
///
///        fn get_pipe_form_loss_k_immutable(&self) -> f64 {
///            return 5.0;
///        }
///
///        /// return absolute roughness for pipe
///        /// for a typical copper pipe
///        /// it is 0.002 mm 
///        /// i did a web search
///        ///
///        fn get_pipe_absolute_roughness(&mut self) -> Length {
///            return Length::new::<millimeter>(0.002);
///        }
///
///        fn get_pipe_absolute_roughness_immutable(&self) -> Length {
///            return Length::new::<millimeter>(0.002);
///        }
///    }
///
///    // finally you can implement a constructor
///
///    impl AirPipe {
///        pub fn new() -> Self {
///            let default_mass_flowrate = 
///                MassRate::new::<kilogram_per_second>(0.0);
///
///            let default_pressure_loss = 
///                Pressure::new::<pascal>(0.0);
///
///            return Self { 
///                mass_flowrate: default_mass_flowrate, 
///                pressure_loss: default_pressure_loss
///            }
///        }
///    }
///
///
///    // with the AirPipe struct setup, you can caluclate
///    // the pressure loss easily
///
///
///    let air_pipe_1 = AirPipe::new();
///    let air_pipe_2 = AirPipe::new();
///    let air_pipe_3 = AirPipe::new();
///    let air_pipe_4 = AirPipe::new();
///    let air_pipe_5 = AirPipe::new();
///    let air_pipe_6 = AirPipe::new();
///    let air_pipe_7 = AirPipe::new();
///    let air_pipe_8 = AirPipe::new();
///    let air_pipe_9 = AirPipe::new();
///    let air_pipe_10 = AirPipe::new();
///
///    // next i make a struct of 
///    struct AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
///        fluid_component_vector_immutable: 
///            Vec<&'air_pipe_collection_lifetime dyn FluidComponent>
///    }
///
///    impl<'air_pipe_collection_lifetime> 
///        FluidComponentCollection<'air_pipe_collection_lifetime>
///        for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
///
///
///        fn get_immutable_fluid_component_vector(&self)
///            -> &Vec<&'air_pipe_collection_lifetime dyn FluidComponent> {
///
///                return &self.fluid_component_vector_immutable;
///            }
///
///        fn set_fluid_component_vector(
///            &mut self, 
///            fluid_component_vector: 
///            Vec<&'air_pipe_collection_lifetime dyn FluidComponent>){
///
///            self.fluid_component_vector_immutable = 
///                fluid_component_vector;
///
///        }
///
///    }
///
///    impl<'air_pipe_collection_lifetime> FluidComponentCollectionMethods for
///        AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
///            fn get_pressure_change(
///                &self,
///                fluid_mass_flowrate: MassRate) -> Pressure {
///
///                // first we get the vector
///
///                let immutable_vector_ref = 
///                    self.get_immutable_fluid_component_vector();
///
///                // second we use the associated function
///
///                let pressure_change = 
///                    Self::calculate_pressure_change_from_mass_flowrate(
///                        fluid_mass_flowrate, immutable_vector_ref);
///
///                return pressure_change;
///            }
///
///            fn get_mass_flowrate_from_pressure_change(
///                &self,
///                pressure_change: Pressure) -> MassRate {
///
///
///                // first we get the vector
///
///                let immutable_vector_ref = 
///                    self.get_immutable_fluid_component_vector();
///
///                // second we use the associated function
///
///                let mass_flowrate = 
///                    Self::calculate_mass_flowrate_from_pressure_change(
///                        pressure_change, immutable_vector_ref);
///
///                return mass_flowrate;
///
///            }
///
///
///        }
///
///    impl<'air_pipe_collection_lifetime> FluidComponentCollectionSeriesAssociatedFunctions
///        for AirPipeCollectionSeries<'air_pipe_collection_lifetime> {}
///
///    // constructor is here
///
///    impl<'air_pipe_collection_lifetime>
///        AirPipeCollectionSeries<'air_pipe_collection_lifetime> {
///        fn new() -> Self {
///            return Self { 
///                fluid_component_vector_immutable:  vec![]
///            };
///        }
///    }
///
///    
///    let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];
///
///    air_pipe_vec.push(&air_pipe_1);
///    air_pipe_vec.push(&air_pipe_2);
///    air_pipe_vec.push(&air_pipe_3);
///    air_pipe_vec.push(&air_pipe_4);
///    air_pipe_vec.push(&air_pipe_5);
///    air_pipe_vec.push(&air_pipe_6);
///    air_pipe_vec.push(&air_pipe_7);
///    air_pipe_vec.push(&air_pipe_8);
///    air_pipe_vec.push(&air_pipe_9);
///    air_pipe_vec.push(&air_pipe_10);
///
///    // now i've made my air pipe vector, i can push it into the air pipe collection
///    let mut air_pipe_series = 
///        AirPipeCollectionSeries::new();
///
///    air_pipe_series.set_fluid_component_vector(air_pipe_vec);
///
///    // now let's push a 0.1kg/s airflow through this pipe series
///    //
///    let pipe_airflow = MassRate::new::<kilogram_per_second>(0.1);
///
///    // and then let's get the pressure change
///
///    let pipe_pressure_change = air_pipe_series.
///        get_pressure_change(pipe_airflow);
///
///    // the pressure losses are about -174650 Pa
///    approx::assert_relative_eq!(
///        pipe_pressure_change.value,
///        -174650.0,
///        max_relative=0.001);
///
///    // the next step is that i want to make a parallel collection of
///    // such pipes in series
///    // so i'll make two more pipe series first
///    // and I'll "recycle" the air pipe by using immutable references
///    // to them
///
///    let mut air_pipe_series_2 = 
///        AirPipeCollectionSeries::new();
///
///    // note that i cannot use air_pipe_vec anymore, it has been
///    // moved or used up by the earlier function 
///    // i'll instead use a new air pipe vector and push the same air pipes
///
///    let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];
///
///    air_pipe_vec.push(&air_pipe_1);
///    air_pipe_vec.push(&air_pipe_2);
///    air_pipe_vec.push(&air_pipe_3);
///    air_pipe_vec.push(&air_pipe_4);
///    air_pipe_vec.push(&air_pipe_5);
///    air_pipe_vec.push(&air_pipe_6);
///    air_pipe_vec.push(&air_pipe_7);
///    air_pipe_vec.push(&air_pipe_8);
///    air_pipe_vec.push(&air_pipe_9);
///    air_pipe_vec.push(&air_pipe_10);
///
///    air_pipe_series_2.set_fluid_component_vector(air_pipe_vec);
///    
///    let mut air_pipe_series_3 = 
///        AirPipeCollectionSeries::new();
///
///    let mut air_pipe_vec: Vec<&dyn FluidComponent> = vec![];
///
///    air_pipe_vec.push(&air_pipe_1);
///    air_pipe_vec.push(&air_pipe_2);
///    air_pipe_vec.push(&air_pipe_3);
///    air_pipe_vec.push(&air_pipe_4);
///    air_pipe_vec.push(&air_pipe_5);
///    air_pipe_vec.push(&air_pipe_6);
///    air_pipe_vec.push(&air_pipe_7);
///    air_pipe_vec.push(&air_pipe_8);
///    air_pipe_vec.push(&air_pipe_9);
///    air_pipe_vec.push(&air_pipe_10);
///
///    air_pipe_series_3.set_fluid_component_vector(air_pipe_vec);
///
///    // now that this is okay, i will then need to put these air pipe
///    // series into a vector as well
///    // using immutable references in the vector
///    //
///    // The super vector is a vector of vectors
///
///    let mut air_pipe_super_vector: 
///        Vec<&dyn FluidComponentCollectionMethods> = vec![];
///
///    air_pipe_super_vector.push(&air_pipe_series);
///    air_pipe_super_vector.push(&air_pipe_series_2);
///    air_pipe_super_vector.push(&air_pipe_series_3);
///
///    // so we are going to make a new struct called
///    // AirPipeParallelSuperCollection
///    //
///    // There are two main traits to apply,
///    //
///    // (1) the FluidComponentSuperCollection trait
///    // (2) the FluidComponentCollectionParallelAssociatedFunctions trait
///    //
///    // the FluidComponentSuperCollection trait also includes 
///    // implementing the FluidComponentCollectionMethods trait
///    //
///    // so there are three traits in total but we only need remember two
///
///    struct AirPipeParallelSuperCollection<'super_collection_lifetime> {
///
///        // the struct only needs us implement a 
///        // super collection vector
///        // NOT references here
///        super_collection_vector_immutable: 
///            Vec<&'super_collection_lifetime dyn FluidComponentCollectionMethods>
///
///    }
///
///    impl<'super_collection_lifetime> 
///        FluidComponentSuperCollection<'super_collection_lifetime>
///            for AirPipeParallelSuperCollection<'super_collection_lifetime> {
///
///        fn get_immutable_vector(&self) 
///            -> &Vec<&'super_collection_lifetime dyn FluidComponentCollectionMethods>{
///
///                return &self.super_collection_vector_immutable;
///            }
///
///
///        fn set_vector(
///            &mut self,
///            fluid_component_vector: 
///            Vec<&'super_collection_lifetime dyn FluidComponentCollectionMethods>){
///
///            self.super_collection_vector_immutable = fluid_component_vector;
///
///        }
///
///
///    }
///
///    impl<'super_collection_lifetime> 
///        FluidComponentCollectionMethods
///            for AirPipeParallelSuperCollection<'super_collection_lifetime> {
///
///                fn get_pressure_change(
///                    &self,
///                    fluid_mass_flowrate: MassRate) -> Pressure {
///
///                    // gets pressure change from a mass flowrate
///                    // going through this super collection
///                    //
///
///                    let fluid_component_collection_vector = 
///                        self.get_immutable_vector();
///
///                    let pressure_change = 
///                        <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>
///                        ::calculate_pressure_change_from_mass_flowrate(
///                            fluid_mass_flowrate, 
///                            fluid_component_collection_vector);
///
///                    return pressure_change;
///                }
///
///                fn get_mass_flowrate_from_pressure_change(
///                    &self,
///                    pressure_change: Pressure)  -> MassRate {
///
///                    let fluid_component_collection_vector = 
///                        self.get_immutable_vector();
///
///                    let mass_flowrate = 
///                        <Self as FluidComponentSuperCollectionParallelAssociatedFunctions>
///                        ::calculate_mass_flowrate_from_pressure_change(
///                            pressure_change, 
///                            fluid_component_collection_vector);
///
///                    return mass_flowrate;
///
///                }
///
///    }
///
///    impl<'super_collection_lifetime> 
///        FluidComponentSuperCollectionParallelAssociatedFunctions
///        for AirPipeParallelSuperCollection<'super_collection_lifetime>{
///
///
///        }
///
///    impl<'super_collection_lifetime> 
///        AirPipeParallelSuperCollection<'super_collection_lifetime> {
///            
///            fn new() -> Self {
///                // constructor returns an instance with an empty vector
///                return Self { super_collection_vector_immutable: vec![] };
///            }
///        }
///
///    // note that the air pipe parallel super collection must
///    // be instantiated as mutable
///    let mut air_pipe_parallel_super_collection = 
///        AirPipeParallelSuperCollection::new();
///
///    air_pipe_parallel_super_collection.set_vector(
///        air_pipe_super_vector);
///
///    // now based on previous tests of the air pipe,
///    // for a 0.1kg/s airflow 
///    // through this pipe series of 10 pipes 
///    // the pressure change are about -174650 Pa
///    //
///    // that means for the super collection, applying this
///    // pressure change to a super collections of three
///    // branches of 10 pipes each, the flowrate should be
///    // about 0.3 kg/s
///    //
///
///    let super_collection_mass_flowrate = 
///        air_pipe_parallel_super_collection.
///        get_mass_flowrate_from_pressure_change(
///            Pressure::new::<pascal>(-174650_f64));
///
///    // if this is set up correct, we expect about 0.3 kg/s of mass flowrate
///    approx::assert_relative_eq!(
///        0.3,
///        super_collection_mass_flowrate.value,
///        max_relative=0.001);
///
///
///    // now let's apply the mass flowrate of 0.3 kg/s and see if we get
///    // the same pressure change
///
///    let super_collection_pressure_change = 
///        air_pipe_parallel_super_collection.
///        get_pressure_change(
///            MassRate::new::<kilogram_per_second>(0.3));
///
///    // if this is set up correct, we expect about 
///    // -174650 Pa of pressure loss
///    approx::assert_relative_eq!(
///        -174650_f64,
///        super_collection_pressure_change.value,
///        max_relative=0.001);
///
///
///    return;
///
/// ```
pub use crate::fluid_component_collection::*;


/// Last but not least, we also have fluid thermophsyical properties as
/// a trait
pub use crate::fluid_thermophysical_properties::*;


/// Here are some additional helpful things you might need
/// because the crate is heavily dependent upon the units of measure (uom)
/// crate for unit safety
///
/// The import list is NOT exhaustive, but is a good way to start
extern crate uom;
pub use std::f64::consts::PI;
pub use uom::si::f64::*;
pub use uom::si::length::{meter, inch, millimeter};
pub use uom::si::mass_rate::kilogram_per_second;
pub use uom::si::pressure::{pascal};
pub use uom::si::angle::degree;
pub use uom::si::mass_density::kilogram_per_cubic_meter;
pub use uom::si::thermodynamic_temperature::degree_celsius;
pub use uom::si::dynamic_viscosity::{millipascal_second, pascal_second};
pub use uom::si::area::square_meter;
