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
/// fluid_component_collection is a module containing
/// traits for vectors of fluid component trait objects
///
pub mod fluid_component_collection;
pub use fluid_component_collection::*;

/// fluid_component_super_collection is a module 
/// containing traits for vectors of fluid component collections
///
/// For example, a pipe is a fluid component
/// a series of pipes is a fluid component collection
/// three branches each containing a series of pipes becomes a super collection
/// 
pub mod fluid_component_super_collection;
pub use fluid_component_super_collection::*;

/// This module contains associated functions and algorithms
/// for calculating pressure changes and mass flowrates
/// of fluid components in series and parallel
///
/// note: multithreaded operations not included here
pub mod collection_series_and_parallel_functions;
pub use collection_series_and_parallel_functions::*;

/// This module contains associated functions and algorithms
/// for calculating pressure changes and mass flowrates
/// of fluid components in series and parallel
///
/// note: multithreaded operations not included here
pub mod super_collection_series_and_parallel_functions;
pub use super_collection_series_and_parallel_functions::*;

/// This module contains tests and examples for the fluid component
/// collections and super collection traits
pub mod tests_and_examples;
