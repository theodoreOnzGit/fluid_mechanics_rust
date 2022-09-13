mod churchill_friction_factor;

#[allow(non_snake_case)]
pub fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        darcy(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
pub fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return churchill_friction_factor:: 
        moody(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
pub fn fldk(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64{
    return churchill_friction_factor::
        fLDK(ReynoldsNumber,
             roughnessRatio,
             lengthToDiameterRatio,
             K);
}


#[allow(non_snake_case)]
pub fn get_bejan_d(ReynoldsNumber: f64,
                   roughnessRatio: f64,
                   lengthToDiameterRatio: f64,
                   K: f64) -> f64 {
    return churchill_friction_factor::
        getBe(ReynoldsNumber, roughnessRatio,
              lengthToDiameterRatio, K);
}
