#![warn(missing_docs)]
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

// this code allos us to get Reynold's number from a Bejan
// number for a custom pipe.
// i make no assumptions about the symmetry of flow
// ie. i don't make assumptions about whether
// the pipe exhibits the same pressure loss
// in forwards and backwards flow,
// that is up to the user to decide when 
// customDarcy and customK is put in
#[allow(non_snake_case)]
pub fn getRe(customDarcy: &dyn Fn(f64, f64) -> f64, 
             Be_D: f64,
             roughnessRatio: f64,
             lengthToDiameter: f64,
             customK: &dyn Fn(f64) -> f64) -> f64 {

    if lengthToDiameter <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }


    // this part deals with negative Be_L values
    // invalid Be_L values

    let maxRe = 1.0e12;

    // i calculate the Be_D corresponding to 
    // Re = 1e12
    let maxBe_D = custom_fLDK_Be_D(
        customDarcy,
        maxRe,
        roughnessRatio, 
        lengthToDiameter,
        customK);

    if Be_D >= maxBe_D {
        panic!("Be too large");
    }
    // the above checks for all the relevant exceptions
    // including formLossK < 0
    //
    // now we are ready to do root finding
    //
    // the underlying equation is 
    // Be = 0.5*fLDK*Re^2


    let pressureDropRoot = |Re: AD| -> AD {
        // i'm solving for
        // Be - 0.5*fLDK*Re^2 = 0 
        // the fLDK term can be calculated using
        // getBe
        //
        // now i don't really need the interpolation
        // term in here because when Re = 0,
        // Be = 0 in the getBe code.
        // so really, no need for fancy interpolation.
        //
        // Now in peroxide, the type taken in and out
        // is not a f64 double
        // but rather AD which stands for automatic 
        // differentiation
        // https://docs.rs/peroxide/latest/peroxide/structure/ad/index.html

        let reynoldsDouble = Re.x();
        let fLDKterm = custom_fLDK_Be_D(
            customDarcy,
            reynoldsDouble, 
            roughnessRatio,
            lengthToDiameter,
            customK);

        return AD0(Be_D - fLDKterm);

    };

    let ReynoldsNumberResult = bisection(pressureDropRoot,
                                         (-maxRe,maxRe),
                                         100,
                                         1e-8);



    // the unwrap turns the result into f64
    let ReynoldsNumber = ReynoldsNumberResult.unwrap();

    return ReynoldsNumber;
}

