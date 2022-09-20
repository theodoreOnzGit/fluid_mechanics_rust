extern crate uom;

// adding units from uom

// custom therminol component imports
use crate::therminol_component::custom_therminol_component::
DowthermACustomComponent;

use crate::therminol_component::
StandardCustomComponentProperties;

use crate::therminol_component::therminol_pipe::
DowthermAPipe;

use super::StandardPipeProperties;

/// CTAH branch items 
/// I'm still in the midst of rearranging things though...
pub struct Flowmeter40 {
    // ctah line flowmeter 40
    // label 14a on simulation diagram
    // fldk = 18.0+93000/Re
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
                6.11e-4, // cross sectional area in meters sq
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                       // in millimeters
                90.0, //incline angle in degrees
                &Flowmeter40::custom_darcy,
                &Flowmeter40::custom_k);

        return flowmeter_40_14a;
    }
}

pub struct Pipe13 {
    // pipe 13 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42, 
    // F-40 and F-41 on CIET diagram
}

impl Pipe13 {

    pub fn get() -> DowthermAPipe {
        let pipe_13: DowthermAPipe 
            = StandardPipeProperties::new( 
                "pipe_13".to_string(),
                2.79e-2, // component diameter in meters
                1.273175, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                       // in millimeters
                0.0, // angle in degrees
                12.95 // form loss K value
                );

        return pipe_13;
    }

}

pub struct Pipe14 {
    // pipe 14 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42, 
    // F-40 and F-41 on CIET diagram
    // it is inclined 90 degrees upwards in direction
    // of flow
    // 
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also 90 degrees
}

impl Pipe14 {

    pub fn get() -> DowthermAPipe {
        let pipe_14: DowthermAPipe 
            = StandardPipeProperties::new( 
                "pipe_14".to_string(),
                2.79e-2, // component diameter in meters
                0.6687, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                       // in millimeters
                90.0, // angle in degrees
                2.4 // form loss K value
                );

        return pipe_14;
    }
}

pub struct Pipe15 {
    // pipe 15 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    // 
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also 
    // -49.36983 degrees
}

impl Pipe15 {

    pub fn get() -> DowthermAPipe {
        let pipe_15: DowthermAPipe 
            = StandardPipeProperties::new( 
                "pipe_15".to_string(),
                2.79e-2, // component diameter in meters
                0.3556, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                       // in millimeters
                -49.36983, // angle in degrees
                0.8 // form loss K value
                );

        return pipe_15;
    }
}

pub struct Pipe16 {
    // pipe 16 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    // 
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also 
    // -49.36983 degrees
}

impl Pipe16 {

    pub fn get() -> DowthermAPipe {
        let pipe_16: DowthermAPipe 
            = StandardPipeProperties::new( 
                "pipe_16".to_string(),
                2.79e-2, // component diameter in meters
                0.644525, // component length in meters
                0.015, // estimated component wall roughness 
                       // (doesn't matter here,
                       // but i need to fill in
                       // millimeters
                -90.0, // angle in degrees
                1.9 // form loss K value
                );

        return pipe_16;
    }
}
/// bypass line times in ciet,
/// tbd, still organising
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
                6.11e-4, // cross sectional area in meters sq
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0 -180.0, //incline angle in degrees
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
                "flowmeter_20_21a".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, // cross sectional area in meters sq
                0.36, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0 - 180.0, //incline angle in degrees
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
                "flowmeter_60_37a".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, // cross sectional area in meters sq
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
    // just after CTAH (AKA IHX)
    // from top to bottom
    // label 8 on diagram
    //
    // forced convection flow direction is same as top to bottom
    //
    // has a fldk of 21+4000/Re
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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_40: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_40_label_8".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
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
    // just before CTAH (AKA IHX)
    // from top to bottom
    //
    // label 6 on diagram
}

impl StaticMixer41 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_41: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_41_label_6".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                51.526384, //incline angle in degrees
                &StaticMixer41::custom_darcy,
                &StaticMixer41::custom_k);

        return static_mixer_41;
    }
}

pub struct StaticMixer10 {
    // static mixer 10 (MX-10) on CIET diagram
    // just before the heater in the heater branch
    // from top to bottom
    // label 2 on diagram (fig A-1 on Nico Zweibaum thesis)
    // pg 125 on pdf viewer, pg 110 on printed page number on bottom right
    //
    // though in reality flow goes from bottom to
    // top in forced convection
    // so from a flow perspective it is before the 
    // heater
    //
}
impl StaticMixer10 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_10: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_41_label_2".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0-180.0, //incline angle in degrees
                &StaticMixer10::custom_darcy,
                &StaticMixer10::custom_k);

        return static_mixer_10;
    }
}

pub struct StaticMixer20 {
    // static mixer 20 (MX-20) on CIET diagram
    // in the DRACS branch in primary loop
    // just after the DRACS heat exchanger
    // from top to bottom
    // label 23
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // also in the DRACS
    // loop there are flow diodes to make 
    // it such that flow going from bottom to top
    // encounters more resistance
    //
    // original angle is is 90 degrees 
    // but i orientate from top to bottom
}
impl StaticMixer20 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_20: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_20_label_23".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0-180.0, //incline angle in degrees
                &StaticMixer20::custom_darcy,
                &StaticMixer20::custom_k);

        return static_mixer_20;
    }
}

pub struct StaticMixer21 {
    // static mixer 21 (MX-21) on CIET diagram
    // in the DRACS branch in primary loop
    // just before the DRACS heat exchanger
    // from top to bottom
    // label 25
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // also in the DRACS
    // loop there are flow diodes to make 
    // it such that flow going from bottom to top
    // encounters more resistance
    //
}
impl StaticMixer21 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_21: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_21_label_25".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0-180.0, //incline angle in degrees
                &StaticMixer21::custom_darcy,
                &StaticMixer21::custom_k);

        return static_mixer_21;
    }
}

pub struct StaticMixer60 {
    // static mixer 60 (MX-60) on CIET diagram
    // in the NDHX branch in secondary DRACS loop
    // just after the NDHX heat exchanger
    // from top to bottom
    // ie this is where hot fluid gets cooled by a fan
    // label 36
    //
    // in reality flow goes from top to
    // bottom in natural convection
    //
}
impl StaticMixer60 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_60: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_60_label_36".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -58.99728, //incline angle in degrees
                &StaticMixer60::custom_darcy,
                &StaticMixer60::custom_k);

        return static_mixer_60;
    }
}

pub struct StaticMixer61 {
    // static mixer 61 (MX-61) on CIET diagram
    // in the DHX branch in secondary DRACS loop
    // just before the DHX heat exchanger
    // from top to bottom
    // ie this is where cool fluid gets heated by the 
    // primary loop heat exchanger
    // label 31
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // so it is actually after the DHX from perspective of flow
    //
}
impl StaticMixer61 {


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
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let static_mixer_61: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "static_mixer_61_label_31".to_string(),
                2.79e-2, // component diameter in meters
                6.11e-4, //component area in sq meters
                0.33, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                90.0 - 180.0, //incline angle in degrees
                &StaticMixer61::custom_darcy,
                &StaticMixer61::custom_k);

        return static_mixer_61;
    }
}

pub struct CTAHHorizontal {

    // coiled tube air heater
    // has fldk = 400 + 52,000/Re
    // 
    // label is 7b
    // empirical data in page 48 on pdf viewer in Dr
    // Zweibaum thesis shows reverse flow has same
    // pressure drop characteristics as forward flow
}

impl CTAHHorizontal {


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
            400.0 + 52000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let ctah_horizontal: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "ctah_horizontal_label_7b".to_string(),
                1.19e-2, // component diameter in meters
                1.33e-3, //component area in sq meters
                1.2342, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                0.0, //incline angle in degrees
                &CTAHHorizontal::custom_darcy,
                &CTAHHorizontal::custom_k);

        return ctah_horizontal;
    }
}

pub struct CTAHVertical {

    // coiled tube air heater,
    // uses pipe friction factors but has a constant K value
    // also pipe isn't circular 
    // so we'll have to use custom fldk to help
    // label 7b
}

impl CTAHVertical {


    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use crate::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 3.9;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    pub fn get() -> DowthermACustomComponent {

        let ctah_vertical: DowthermACustomComponent 
            = StandardCustomComponentProperties::new(
                "ctah_vertical_label_7a".to_string(),
                1.19e-2, // component diameter in meters
                1.33e-3, //component area in sq meters
                0.3302, // component length in meters
                0.015, // estimated component wall roughness (doesn't matter here,
                       // but i need to fill in
                -90.0, //incline angle in degrees
                &CTAHVertical::custom_darcy,
                &CTAHVertical::custom_k);

        return ctah_vertical;
    }
}














