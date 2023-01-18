
use uom::si::f64::*;
use uom::si::acceleration::meter_per_second_squared;
/// This is a generic fluid component trait,
/// which specifies that fluid components in general
/// should have the following properties accessed
/// via get and set methods
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




        let pressure_loss = -pressure_change +
            self.get_hydrostatic_pressure_change_immutable()+
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


        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change();

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

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change_immutable();

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

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change();

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
    /// h = component_length * sin (incline_angle)
    ///
    /// component length is the shortest or straight line
    /// distance between
    /// inlet and outlet
    /// and incline angle is the angle that straight line makes
    /// with the horizontal plane
    fn get_hydrostatic_pressure_change(
        &mut self) -> Pressure {

        let component_length =
            self.get_component_length();

        let incline_angle = 
            self.get_incline_angle();

        let fluid_density = 
            self.get_fluid_density();

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = component_length*incline_angle.sin();

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
    ///
    /// component length is the shortest or straight line
    /// distance between
    /// inlet and outlet
    /// and incline angle is the angle that straight line makes
    /// with the horizontal plane
    fn get_hydrostatic_pressure_change_immutable(
        &self) -> Pressure {

        let component_length =
            self.get_component_length_immutable();

        let incline_angle = 
            self.get_incline_angle_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();


        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = component_length*incline_angle.sin();

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
