use uom::si::f64::*;
use uom::si::pressure::pascal;

/// This trait contains functions for error propagation
/// It basically contains a function to return the
/// total error based on some Type
///
/// You will need to specify the input and output type
/// for this trait

pub trait FluidComponentError{


    /// returns (result,deviation) tuple for pressure
    /// given mass flowrate deviation
    fn mass_flow_to_pressure_result_error_tuple(
        x_value: MassRate,
        x_error: MassRate,
        function: &dyn Fn(MassRate) -> Pressure) -> (Pressure, Pressure) {

        let result = function(x_value);

        let deviation = Self::partial_pressure_deviation_massrate(
            x_value, 
            x_error, 
            function);

        return (result, deviation);

    }

    /// computes pressure deviation based on 
    /// square root of sum of squares
    fn sqrt_sum_pressure_deviation(pressure_vec: Vec<Pressure>)
    -> Pressure{

        // we square and sum up the pressure first

        let mut pressure_sq_sum = 
            Pressure::new::<pascal>(0.0)*Pressure::new::<pascal>(0.0);

        for pressure_value in pressure_vec.iter(){
            // first let's dereference the pointers
            // and multiply them
            let pressure_sq = *pressure_value * *pressure_value;
            pressure_sq_sum += pressure_sq;

        }

        let pressure_error = pressure_sq_sum.sqrt();

        return pressure_error;
    }


    /// does partial deviation for pressure 
    /// based on fldk value
    /// using finite difference
    fn partial_pressure_deviation_fldk(
        x_value: f64,
        x_error: f64, 
        function: &dyn Fn(f64) -> Pressure) -> Pressure {

        // first, we compute a finite difference 
        let x_upper = x_value + x_error;
        let x_lower = x_value - x_error;

        let y_upper  = function(x_upper);
        let y_lower = function(x_lower);

        let gradient_estimate = 
            (y_upper - y_lower)/(x_upper - x_lower);

        // now that we computed the finite difference, we can
        // return the y_error

        let y_error = 
            x_error * gradient_estimate;

        return y_error;


    }


    /// does partial deviation for pressure 
    /// based on mass flowrate
    /// using finite difference
    fn partial_pressure_deviation_massrate(
        x_value: MassRate,
        x_error: MassRate, 
        function: &dyn Fn(MassRate) -> Pressure) -> Pressure {

        // first, we compute a finite difference 
        let x_upper = x_value + x_error;
        let x_lower = x_value - x_error;

        let y_upper  = function(x_upper);
        let y_lower = function(x_lower);

        let gradient_estimate = 
            (y_upper - y_lower)/(x_upper - x_lower);

        // now that we computed the finite difference, we can
        // return the y_error

        let y_error = 
            x_error * gradient_estimate;

        return y_error;


    }

    /// a function to return the partial deviation of
    /// a type
    ///
    /// for example, 
    /// a pressure change error
    /// is dependent on some
    /// mass flowrate error 
    ///
    /// we can calculate this using:
    ///
    /// (\delta delta P)^2 = 
    /// (\delta m_i)^2 [(\partial Delta P)/(\partial m_i)]^2
    ///
    ///
    /// (\delta delta P) is the partial_deviation in this
    /// case due to mass flowrate. it's not the full deviation
    ///
    /// There are three things you need to supply
    /// (1) how much error there is
    /// (2) the function which takes in 
    /// (3) where you want to evaluate this error
    fn partial_deviation_f64(
        x_value: f64,
        x_error: f64, 
        function: &dyn Fn(f64) -> f64) -> f64 {

        // first, we compute a finite difference 
        let x_upper = x_value + x_error;
        let x_lower = x_value - x_error;

        let y_upper  = function(x_upper);
        let y_lower = function(x_lower);

        let gradient_estimate = 
            (y_upper - y_lower)/(x_upper - x_lower);

        // now that we computed the finite difference, we can
        // return the y_error

        let y_error = 
            x_error * gradient_estimate;

        return y_error;


    }
}
