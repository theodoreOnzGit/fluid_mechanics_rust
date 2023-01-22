#![warn(missing_docs)]
extern crate peroxide;
use peroxide::prelude::*;

// This library was developed for use in my PhD thesis under supervision 
// of Professor Per F. Peterson. It is part of a thermal hydraulics
// library in Rust that is released under the GNU General Public License
// v 3.0. This is partly due to the fact that some of the libraries 
// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
// Public License v3.0.
//
// As such, the entire library is released under GNU GPL v3.0. It is a strong 
// copyleft license which means you cannot use it in proprietary software.
//
//
// License
//    This file is part of fluid_mechanics_rust, a partial library of the
//    thermal hydraulics library written in rust meant to help with the
//    fluid mechanics aspects of the calculations
//     
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    fluid_mechanics_rust is free software; you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    fluid_mechanics_rust is distributed in the hope that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    This library is part of a thermal hydraulics library in rust
//    and contains some code copied from GeN-Foam, and OpenFOAM derivative.
//    This offering is not approved or endorsed by the OpenFOAM Foundation nor
//    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
//    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
//    Nor is it endorsed by the authors and owners of GeN-Foam.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// © All rights reserved. Theodore Kay Chen Ong,
// Singapore Nuclear Research and Safety Initiative,
// Per F. Peterson,
// University of California, Berkeley Thermal Hydraulics Laboratory
//
// Main author of the code: Theodore Kay Chen Ong, supervised by
// Professor Per F. Peterson

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
/// calculates darcy friction factor using churchill correlation
pub fn darcy(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}

#[allow(non_snake_case)]
/// calculates moody friction factor using churchill correlation
/// basically same as darcy
pub fn moody(ReynoldsNumber: f64, roughnessRatio: f64) -> f64 {
    return 4.0*fanning(ReynoldsNumber, roughnessRatio);
}


#[allow(non_snake_case)]

/// calculates fLDK using churchill correlation
/// and a user defined form loss K value
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
/// calculates a nondimensional pressure loss (Be_D)
/// from the nondimensionalised flowrate (Re_D)
pub fn getBe(mut ReynoldsNumber: f64,
             roughnessRatio: f64,
             lengthToDiameterRatio: f64,
             K: f64) -> f64{

    if ReynoldsNumber == 0.0 {
        return 0.0;
    }

    let mut isNegative = false;

    if ReynoldsNumber < 0.0 {
        isNegative = true;
        ReynoldsNumber = ReynoldsNumber * -1.0;
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if lengthToDiameterRatio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if K < 0.0 {
        panic!("Form loss coefficient K < 0.0");
    }

    let f = darcy(ReynoldsNumber, roughnessRatio);

    let fLDK = f*lengthToDiameterRatio + K;

    let mut Be = 0.5*fLDK*ReynoldsNumber.powf(2.0);

    if isNegative {
        Be = Be * -1.0;
        return Be;
    }

    return Be;
}

#[allow(non_snake_case)]
/// calculates Re given a Be_D 
///
/// it is basically calculating nondimensionalised
/// flowrate from nondimensionalised pressure loss
pub fn getRe(mut Be_D: f64,
             roughnessRatio: f64,
             lengthToDiameter: f64,
             formLossK: f64) -> f64 {

    if lengthToDiameter <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if roughnessRatio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if formLossK < 0.0 {
        panic!("formLossK<0.0");
    }

    // this part deals with negative Be_L values
    // invalid Be_L values
    let mut isNegative = false;
    if Be_D < 0.0 {
        Be_D = Be_D * -1.0;
        isNegative = true;
    }

    let maxRe = 1.0e12;

    // i calculate the Be_D corresponding to 
    // Re = 1e12
    let maxBe_D = getBe(maxRe,roughnessRatio, 
                        lengthToDiameter,formLossK);

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
        let fLDKterm = getBe(reynoldsDouble, roughnessRatio,
                             lengthToDiameter,
                             formLossK);

        return AD0(Be_D - fLDKterm);

    };

    let ReynoldsNumberResult = bisection(pressureDropRoot,
                                         (0.0,maxRe),
                                         100,
                                         1e-8);



    // the unwrap turns the result into f64
    let mut ReynoldsNumber = ReynoldsNumberResult.unwrap();


    if isNegative
    {
        ReynoldsNumber = ReynoldsNumber * -1.0;
        return ReynoldsNumber;
    }

    return ReynoldsNumber;
}
