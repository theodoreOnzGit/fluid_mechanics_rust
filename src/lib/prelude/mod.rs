//! Welcome to the fluid mechanics rust
//!
//! here is a library of traits meant to help calculate friction losses 
//!
//! in pipes and fluid components, as well as some of these components
//! connected in series or parallel configuration
//!
//! 

/// These traits help you create pipes
pub use crate::fluid_component_calculation::FluidComponent;
pub use crate::fluid_component_calculation::standard_pipe_calc::FluidPipeCalcPressureChange;
pub use crate::fluid_component_calculation::standard_pipe_calc::FluidPipeCalcPressureLoss;

/// these traits help you create components which do not exhibit pipe pressure losses
pub use crate::fluid_component_calculation::custom_component_calc::
{FluidCustomComponentCalcPressureLoss,FluidCustomComponentCalcPressureChange};

/// If you want to connect components or pipes in series or parallel, this
/// import is good for that
///
/// in fact, if you wanted to make a system with 4 parallel branches
/// and each branch has a series of fluid components,
/// this trait is meant to help with that
pub use crate::fluid_component_collection::*;


/// Last but not least, we also have fluid thermophsyical properties as
/// a trait
pub use crate::fluid_thermophysical_properties::*;
