mod churchill_friction_factor;
mod custom_fldk;

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

#[allow(non_snake_case)]
pub fn get_reynolds_number(Be_D: f64,
             roughnessRatio: f64,
             lengthToDiameter: f64,
             formLossK: f64) -> f64 {
    return churchill_friction_factor::
        getRe(Be_D, roughnessRatio,
              lengthToDiameter, formLossK);

}

pub struct CustomComponent {
}

#[allow(non_snake_case)]
impl CustomComponent {

    // i allow users to define their own fldk
    pub fn fldk(customDarcy: &dyn Fn(f64, f64) -> f64,
    ReynoldsNumber: f64,
    roughnessRatio: f64,
    lengthToDiameterRatio: f64,
    customK: &dyn Fn(f64) -> f64) -> f64{
        return custom_fldk::custom_fLDK(&customDarcy,
                                        ReynoldsNumber,
                                        roughnessRatio,
                                        lengthToDiameterRatio,
                                        &customK);

    }

    // if the user only wants to change K to be a custom value
    // then fldk_pipe is more appropriate
    pub fn fldk_pipe(ReynoldsNumber: f64,
                         roughnessRatio: f64,
                         lengthToDiameterRatio: f64,
                         customK: &dyn Fn(f64) -> f64) -> f64{

        return custom_fldk::custom_Kpipe(ReynoldsNumber,
                                         roughnessRatio,
                                         lengthToDiameterRatio,
                                         &customK);
    }
}
