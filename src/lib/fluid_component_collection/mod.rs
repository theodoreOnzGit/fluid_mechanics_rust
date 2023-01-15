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
