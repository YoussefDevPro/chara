so uh, i feel like i fucked up the architecture of the models, and also the services, so im going to make a standard architecture that i have to follow when writing this code 
# models 
models are the blueprint of the tables, they must have simple and intuitive design so its ez to use it in developpement, 
the code im going to write must be the standard for a model 
```rust
pub struct ITEM {
    id: ItemID,
    // is the identifiant of the item
    created_at: Date, 
    // (never changes, its only to know the creation date of the item, therefor, no action will be executed on this field)
    updated_at: Date, 
    // each action done on the item must be logged, we know the last time the item got updated, so we know if the item is not used etc
    is_deleted: bool, 
    // its a soft delet, in this platform we will never delet anything, so data will be easly recoverable
    // (other field here)
}
```
those are the important part of an item, when writing a model we must make sure abt the relation between the items, and add functions to help in developpement, and also add struct helpers, to ensure security like
`InsertITEM` to create a data, as well as a `from_insert()` function to create an `ITEM` from the insert, and a `ITEMPatch` for updating the data, so like that we ignore the data like created at and those kind of types, and also security checks when making the patch, so we dont count admin only fields if the actor is not a user     
and also, we will move all the other modules into a `core`  module, as what we're writing is only the code of the platform, the models are only public for the crate.

other reconsideration will be made when the models are done...
