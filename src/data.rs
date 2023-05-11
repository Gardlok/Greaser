use crate::craft::{DataCraft::*, EdgeCraft::*, NodeCraft::*};
use std::any::Any;
use std::hash::{Hash, Hasher};

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

// mod tests {
//     use crate::{DataCraft::*, EdgeCraft::*};
//     use hashbrown::{HashMap, HashSet};

//     use super::DynHash;
//     //
//     type DHashMap = HashMap<Box<dyn DynHash>, Box<dyn DynHash>>;
//     type DHashSet = HashSet<Box<dyn DynHash>>;
//     fn new<T: DynHash>(value: T) -> Box<dyn DynHash> {
//         Box::new(value)
//     }

// #[test]
// fn dyn_hashset() {
//     let mut hset = DHashSet::new();
//     assert!(hset.insert(new(Matridex::new(0u8))));
//     assert_eq!(Matr, hset.pop().unwrap());
// }
// #[test]
// fn dyn_hashmap() {
//     let mut hmap = DHashMap::new();
//     assert!(hmap.insert(Matridex::new(0u8), EdgeType::Buffr));
//     assert_eq!(EdgeType::Buffr, hmap.get(&Matridex::new(0u8)).unwrap());
// }
// }

//let mut map = HashMap::<Box<dyn DynHash>, _>::new();
// type newb: DynHash = ();
// type pro<T> = T;

// assert!(false);
// assert!(new(1u8) != new(1i8));
// map.insert(new(-1i32), "-1i32");
// map.insert(new("hello"), "the string \"hello\"");
// map.insert(new(49u128), "4u128");
// map.insert(new(()), "()");
// assert_eq!(map.get(&6u128 as &dyn DynHash), Some(&"4u128"));
// assert_eq!(
//     map.remove(&"hello" as &dyn DynHash),
//     Some("the string \"hello\"")
// );

/*
int _mm_cmpestra (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestri

int _mm_cmpestrc (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestri

int _mm_cmpestri (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestrm

__m128i _mm_cmpestrm (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestri

int _mm_cmpestro (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestri

int _mm_cmpestrs (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpestri

int _mm_cmpestrz (__m128i a, int la, __m128i b, int lb, const int imm8)
pcmpgtq

__m128i _mm_cmpgt_epi64 (__m128i a, __m128i b)
pcmpistri

int _mm_cmpistra (__m128i a, __m128i b, const int imm8)
pcmpistri

int _mm_cmpistrc (__m128i a, __m128i b, const int imm8)
pcmpistri

int _mm_cmpistri (__m128i a, __m128i b, const int imm8)
pcmpistrm

__m128i _mm_cmpistrm (__m128i a, __m128i b, const int imm8)
pcmpistri

int _mm_cmpistro (__m128i a, __m128i b, const int imm8)
pcmpistri

int _mm_cmpistrs (__m128i a, __m128i b, const int imm8)
pcmpistri

int _mm_cmpistrz (__m128i a, __m128i b, const int imm8)
crc32

unsigned int _mm_crc32_u16 (unsigned int crc, unsigned short v)
crc32

unsigned int _mm_crc32_u32 (unsigned int crc, unsigned int v)
crc32

unsigned __int64 _mm_crc32_u64 (unsigned __int64 crc, unsigned __int64 v)
crc32

unsigned int _mm_crc32_u8 (unsigned int crc, unsigned char v)
*/
