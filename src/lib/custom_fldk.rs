extern crate peroxide;
use peroxide::prelude::*;



// here are the functions used for components with 
// custom friction factor and K, rather messy but


// this first function allows for custom fldk, 
// ie both friction factor and form loss k are user defined
// https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter
#[allow(non_snake_case)]
pub fn custom_fLDK(customDarcy: &dyn Fn(f64, f64) -> f64,
        ReynoldsNumber: f64,
        roughnessRatio: f64,
        lengthToDiameterRatio: f64,
        customK: &dyn Fn(f64) -> f64) -> f64{

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if lengthToDiameterRatio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    let K = customK(ReynoldsNumber);


    let f = customDarcy(ReynoldsNumber, roughnessRatio);
    let fLDK = f*lengthToDiameterRatio + K;

    return fLDK;
}

// this is a special case of the fLDK component,
// where we just specify a custom K but friction factor is based
// on darcy friction factor
#[allow(non_snake_case)]
pub fn custom_Kpipe(ReynoldsNumber: f64,
                    roughnessRatio: f64,
                    lengthToDiameterRatio: f64,
                    customK: &dyn Fn(f64) -> f64) -> f64{

    let darcyFn = crate::churchill_friction_factor::darcy;

    let fLDK = custom_fLDK(&darcyFn,
                           ReynoldsNumber,
                           roughnessRatio,
                           lengthToDiameterRatio,
                           customK);

    return fLDK;

}

#[allow(non_snake_case)]
pub fn custom_Kpipe_Be_D(ReynoldsNumber: f64,
                    roughnessRatio: f64,
                    lengthToDiameterRatio: f64,
                    customK: &dyn Fn(f64) -> f64) -> f64{

    if ReynoldsNumber == 0.0 {
        return 0.0;
    }

    let fLDK = custom_Kpipe(ReynoldsNumber,
                           roughnessRatio,
                           lengthToDiameterRatio,
                           customK);

    let Be_D = 0.5*fLDK*ReynoldsNumber.powf(2.0);

    return Be_D;

}


#[allow(non_snake_case)]
pub fn custom_fLDK_Be_D(customDarcy: &dyn Fn(f64, f64) -> f64, 
                        ReynoldsNumber: f64,
                        roughnessRatio: f64,
                        lengthToDiameterRatio: f64,
                        customK: &dyn Fn(f64) -> f64) -> f64{

    if ReynoldsNumber == 0.0 {
        return 0.0;
    }

    let fLDK = custom_fLDK(customDarcy,
                           ReynoldsNumber,
                           roughnessRatio,
                           lengthToDiameterRatio,
                            customK);

    let Be_D = 0.5*fLDK*ReynoldsNumber.powf(2.0);

    return Be_D;

}



