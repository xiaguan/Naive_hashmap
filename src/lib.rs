#[cfg(test)]
extern crate quickcheck;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

use std::{cmp, mem};

#[derive(Default)]
pub struct HashMap<K, V, S = RandomState>
where
    K: Eq,
    V: ::std::fmt::Debug,
{
    hash_builder: S,
    data: Vec<(u64, K, V)>,
}

impl<K: Eq, V> HashMap<K, V, RandomState>
where
    K: Eq + Hash,
    V: ::std::fmt::Debug,
{
    pub fn new() -> HashMap<K, V> {
        HashMap {
            hash_builder: RandomState::new(),
            data: Vec::new(),
        }
    }
}

fn make_hash<T: ?Sized, S>(hash_builder: &S, t: &T) -> u64
where
    T: Hash,
    S: BuildHasher,
{
    let mut state = hash_builder.build_hasher();
    t.hash(&mut state);
    state.finish()
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
    V: ::std::fmt::Debug,
{
    pub fn with_haser(hash_builder: S) -> HashMap<K, V, S> {
        HashMap {
            hash_builder,
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash(&self.hash_builder, &k);
        let end = self.data.len();

        for idx in 0..end {
            match self.data[idx].0.cmp(&hash) {
                cmp::Ordering::Greater => {
                    self.data.insert(idx, (hash, k, v));
                    return None;
                }
                cmp::Ordering::Less => {
                    continue;
                }
                cmp::Ordering::Equal => {
                    let old_value = mem::replace(&mut self.data[idx].2, v);
                    return Some(old_value);
                }
            }
        }
        assert_eq!(0, self.data.len());
        self.data.push((hash, k, v));
        return None;
    }

    pub fn lookup(&self, k: K) -> Option<&V> {
        let hash = make_hash(&self.hash_builder, &k);
        let find_result = self
            .data
            .iter()
            .find(move |tuple| match tuple.0.cmp(&hash) {
                cmp::Ordering::Equal => true,
                _ => false,
            });
        match find_result {
            Some(tuple) => Some(&tuple.2),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;

    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

    #[test]
    fn get_what_you_give() {
        fn property(k: u16, v: u16) -> TestResult {
            let mut system_under_test = HashMap::new();

            assert_eq!(None, system_under_test.insert(k, v));
            assert_eq!(Some(&v), system_under_test.lookup(k));
            TestResult::passed()
        }
        QuickCheck::new().quickcheck(property as fn(u16, u16) -> TestResult);
    }

    #[derive(Clone, Debug)]
    enum Action<T>
    where
        T: Arbitrary,
    {
        Insert(T, u16),
        Lookup(T),
    }
}
