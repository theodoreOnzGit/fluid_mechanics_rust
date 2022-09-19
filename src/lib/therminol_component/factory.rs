extern crate uom;

// adding units from uom

// custom therminol component imports
use crate::therminol_component::custom_therminol_component::
DowthermACustomComponent;

use crate::therminol_component::
StandardCustomComponentProperties;

pub struct Flowmeter40 {
    // ctah line flowmeter 40
    // label 14a on simulation diagram
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
    // hence degree is a 180-90 degrees = -90 degrees
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
                180.0-90.0, //incline angle in degrees
                &Flowmeter30::custom_darcy,
                &Flowmeter30::custom_k);

        return flowmeter_30;
    }
}

pub struct Flowmeter20 {
    // DHX flow flowmeter 20
    // natural convection heat exchanger in primary loop
    // diagram label is 21a
    // we use the convention of top of bypass branch to bottom
    // hence degree is -90
}
impl Flowmeter20 {

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

        let flowmeter_20: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "flowmeter_20_label_21a".to_string(),
                2.79e-2, // component diameter in meters
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -90.0, //incline angle in degrees
                &Flowmeter20::custom_darcy,
                &Flowmeter20::custom_k);

        return flowmeter_20;
    }
}

pub struct Flowmeter60 {
    // DHX flow flowmeter 60
    // natural convection heat exchanger in DRACS loop
    // this is the secondary loop equivalent for
    // decay heat removal
    //
    // diagram label 37a on simulation model
    // we use the convention of top of bypass branch to bottom (Tank 2)
    // hence degree is -90
}
impl Flowmeter60 {

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

        let flowmeter_60: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "flowmeter_60_label_37a".to_string(),
                2.79e-2, // component diameter in meters
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -90.0, //incline angle in degrees
                &Flowmeter60::custom_darcy,
                &Flowmeter60::custom_k);

        return flowmeter_60;
    }
}

/// static mixers are here
///
///

pub struct StaticMixer40 {
    // static mixer 40 (MX-40) on CIET diagram
    // just before CTAH (AKA IHX)
    // from top to bottom
    //
}
impl StaticMixer40 {

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

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_40: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_40_label_37a".to_string(),
                2.79e-2, // component diameter in meters
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -90.0, //incline angle in degrees
                &StaticMixer40::custom_darcy,
                &StaticMixer40::custom_k);

        return static_mixer_40;
    }
}

pub struct StaticMixer41 {
    // static mixer 41 (MX-41) on CIET diagram
    // in the pump and CTAH branch
    // just after CTAH (AKA IHX)
    // from top to bottom
    //
}

pub struct StaticMixer10 {
    // static mixer 10 (MX-10) on CIET diagram
    // just before the heater in the heater branch
    // from top to bottom
    //
    // though in reality flow goes from bottom to
    // top in forced convection
    // so from a flow perspective it is before the 
    // heater
    //
}

pub struct StaticMixer20 {
    // static mixer 20 (MX-20) on CIET diagram
    // in the DRACS branch in primary loop
    // just after the DRACS heat exchanger
    // from top to bottom
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // also in the DRACS
    // loop there are flow diodes to make 
    // it such that flow going from bottom to top
    // encounters more resistance
    //
}

pub struct StaticMixer21 {
    // static mixer 21 (MX-21) on CIET diagram
    // in the DRACS branch in primary loop
    // just before the DRACS heat exchanger
    // from top to bottom
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // also in the DRACS
    // loop there are flow diodes to make 
    // it such that flow going from bottom to top
    // encounters more resistance
    //
}

pub struct StaticMixer60 {
    // static mixer 60 (MX-60) on CIET diagram
    // in the NDHX branch in secondary DRACS loop
    // just after the NDHX heat exchanger
    // from top to bottom
    // ie this is where hot fluid gets cooled by a fan
    //
    // in reality flow goes from top to
    // bottom in natural convection
    //
}

pub struct StaticMixer61 {
    // static mixer 61 (MX-61) on CIET diagram
    // in the DHX branch in secondary DRACS loop
    // just before the DHX heat exchanger
    // from top to bottom
    // ie this is where cool fluid gets heated by the 
    // primary loop heat exchanger
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // so it is actually after the DHX from perspective of flow
    //
}
