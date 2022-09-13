// here are the functions used for friction factor, rather messy but
// for fast prototyping and sandboxing don't really care too much
//

// first, to allow non snake case names..
#[allow(non_snake_case)]
fn B(Re: f64) -> f64 {
    let numerator = 37530.0_f64.powf(16.0);
    let denominator = Re.powf(16.0);
    return numerator/denominator;
}

#[allow(non_snake_case)]
fn A(Re: f64, roughnessRatio: f64) -> f64 {
    let seven_over_Re = 7.0/Re;
    let reynolds_term = seven_over_Re.powf(0.9);

    let roughness_term = 0.27 * roughnessRatio;

    let log_fraction = 1.0/(reynolds_term + roughness_term);
    // we will need natural logarithms:
    let inner_bracket_term = 2.457*log_fraction.ln();

    let A = inner_bracket_term.powf(16.0);

    return A;


}

#[allow(non_snake_case)]
fn churchillInnerTerm(Re: f64, roughnessRatio: f64) -> f64 {

    let eight_over_Re = 8.0/Re;
    let laminarTerm = eight_over_Re.powf(12.0);

    let Aterm = A(Re,roughnessRatio);
    let Bterm = B(Re);

    let APlusBInverse = 1.0/(Aterm+Bterm);
    let turbulentTerm = APlusBInverse.powf(3.0/2.0);

    return laminarTerm + turbulentTerm;
}

// this particular implementation uses the churchill correlation
#[allow(non_snake_case)]
fn fanning(ReynoldsNumber: f64, roughnessRatio: f64) -> f64{

    if ReynoldsNumber == 0.0 {
        panic!("Re = 0.0");
    }

    if ReynoldsNumber < 0.0 {
        panic!("Re<0.0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    let innerTerm = churchillInnerTerm(ReynoldsNumber, roughnessRatio);
    let powerTerm = innerTerm.powf(1.0/12.0);
    let fanningFrictionFactor = 2.0 * powerTerm;
    return fanningFrictionFactor;
}

#[allow(non_snake_case)]
pub fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
pub fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}


#[allow(non_snake_case)]
pub fn fLDK(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64{
    if ReynoldsNumber == 0.0 {
        panic!("Re = 0");
    }

    if ReynoldsNumber < 0.0 {
        panic!("Re < 0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if lengthToDiameterRatio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if K < 0.0 {
        panic!("For m loss coefficient K < 0.0");
    }

    let f = darcy(ReynoldsNumber, roughnessRatio);
    let fLDK = f*lengthToDiameterRatio + K;

    return fLDK;
}


#[allow(non_snake_case)]
pub fn getBe(ReynoldsNumber: f64,
                    roughnessRatio: f64,
                    lengthToDiameterRatio: f64,
                    K: f64) -> f64{

    if ReynoldsNumber == 0.0 {
        return 0.0;
    }

    let f = darcy(ReynoldsNumber, roughnessRatio);

    let fLDK = f*lengthToDiameterRatio + K;

    let Be = 0.5*fLDK*ReynoldsNumber.powf(2.0);

    return Be;
}

