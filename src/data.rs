use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};
use std::any::Any;
use std::hash::{Hash, Hasher};

//
use tokio::sync::broadcast;

pub struct TemperatureData {
    days: Vec<i32>,
    temps: Vec<f64>,
}

use std::error::Error;

///////////////////////////////////////////////////////////////////
// Dyn Dispatching
//
impl<H: Eq + Any> DynEq for H {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<H>() {
            self == other
        } else {
            false
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<H: Hash + DynEq> DynHash for H {
    fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
        H::hash(self, &mut hasher)
    }
    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }
}
impl PartialEq for dyn DynHash {
    fn eq(&self, other: &dyn DynHash) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}
impl Eq for dyn DynHash {}
impl Hash for dyn DynHash {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.dyn_hash(hasher)
    }
}




{}        
