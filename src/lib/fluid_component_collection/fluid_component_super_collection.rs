use crate::fluid_component_collection::FluidComponentCollectionMethods;

/// a fluid component super collection
/// which contains fluid components stored into a vector
/// and should contain some methods for CRUD operations
///
/// Create
/// Read
/// Update
/// Delete
pub trait FluidComponentSuperCollection<'trait_lifetime> : 
FluidComponentCollectionMethods{

    /// returns a copy of the fluid component collection vector
    /// containing immutable elements
    ///
    /// you'll probably need some legwork to create a fresh
    /// object
    ///
    /// the trait object which is given is the FluidComponentCollectionMethods
    /// trait objects
    ///
    /// as such the fluid component super collection may have 
    /// fluid component collections 
    /// and even super collections
    /// in series or parallel or whatever arrangement is desired
    /// as long as it fulfils the fluid component collection methods
    /// trait
    ///
    /// even a single fluid component can behave like a fluid component
    /// collection of 1 item if it fulfils this trait
    ///
    fn get_immutable_vector(&self) 
        -> &Vec<&'trait_lifetime dyn FluidComponentCollectionMethods>;

    /// sets the fluid component collection vector to a specific value
    fn set_vector(
        &mut self,
        fluid_component_super_vector: 
        Vec<&'trait_lifetime dyn FluidComponentCollectionMethods>);


    /// adds a fluid component collection to the super collection

    fn add_collection_to_vector(
        &mut self,
        fluid_component_super_vector: 
        Vec<&'trait_lifetime dyn FluidComponentCollectionMethods>,
        fluid_component_vector_pointer: 
        &'trait_lifetime dyn FluidComponentCollectionMethods){

        // first i make a mutable version of the fluid component super vector
        let mut fluid_component_super_vector_mutable =
            fluid_component_super_vector;

        // then i push the pointer to this mutable copy
        fluid_component_super_vector_mutable.push(fluid_component_vector_pointer);

        // next i set the fluid component vector
        self.set_vector(fluid_component_super_vector_mutable);

    }

    /// removes a fluid component collection by index from the super collection

    fn remove_collection_by_index(&mut self,
              fluid_component_super_vector: 
              Vec<&'trait_lifetime dyn FluidComponentCollectionMethods>,
              component_index: usize){

        // first i make a mutable copy of the component vector
        let mut fluid_component_super_vector_mutable =
            fluid_component_super_vector;

        // i remove the index from the vector 
        // (note that there may be a case where the vector is smaller than
        // the given index),
        // however, the remove method already has a panic if the 
        // vector is shorter than the given index

        fluid_component_super_vector_mutable.remove(component_index);

        // next i set the fluid component vector
        self.set_vector(fluid_component_super_vector_mutable);
    }

    /// returns read only a pointer of the fluid component collection
    /// given an index

    fn get_collection_by_index(
        &'trait_lifetime mut self,
        component_index: usize) -> 
        &'trait_lifetime dyn FluidComponentCollectionMethods {

        // first let's access the fluid component super vector

        let fluid_component_super_vector =
            self.get_immutable_vector();

        // then i access a particular super collection

        let fluid_component_collection_pointer = 
            fluid_component_super_vector[component_index];

        return fluid_component_collection_pointer;

    }


    /// updates the fluid component collection at the specified
    /// index with a fluid component collection supplied by the user

    fn update_collection_by_index(
        &mut self,
        component_index: usize,
        fluid_component_super_vector: Vec<&'trait_lifetime dyn FluidComponentCollectionMethods>,
        fluid_component_collection_pointer: &'trait_lifetime dyn FluidComponentCollectionMethods){

        // first i make a mutable copy of the component vector
        let mut fluid_component_super_vector_mutable =
            fluid_component_super_vector;

        // then i change the pointer in this mutable copy
        fluid_component_super_vector_mutable[component_index]
            = fluid_component_collection_pointer;

        // next i set the fluid component vector
        self.set_vector(fluid_component_super_vector_mutable);
    }



}
