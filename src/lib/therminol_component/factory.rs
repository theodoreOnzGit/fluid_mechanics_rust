extern crate uom;

// adding units from uom

// custom therminol component imports
use crate::therminol_component::custom_therminol_component::
DowthermACustomComponent;

use crate::therminol_component::
StandardCustomComponentProperties;

pub struct Flowmeter40 {
    // ctah line flowmeter 40
    // label 14a on diagram
}
impl Flowmeter40 {

    // let's import everything necessary:

    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value = 
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let flowmeter_40_14a: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "flowmeter_40_14a".to_string(),
                2.79e-2, // component diameter in meters
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0, //incline angle in degrees
                &Flowmeter40::custom_darcy,
                &Flowmeter40::custom_k);

        return flowmeter_40_14a;
    }
}

pub struct Flowmeter30 {
    // bypass flow flowmeter FM30 
    // not labelled on diagram
    // we use the convention of top of bypass branch to bottom
    // hence degree is -90
}
impl Flowmeter30 {

    // let's import everything necessary:

    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value = 
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let flowmeter_30: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "flowmeter_30".to_string(),
                2.79e-2, // component diameter in meters
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -90.0, //incline angle in degrees
                &Flowmeter30::custom_darcy,
                &Flowmeter30::custom_k);

        return flowmeter_30;
    }
}
