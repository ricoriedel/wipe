use rand::prelude::IteratorRandom;
use rand::Rng;

/// A trait to get all values of an enum.
pub trait Collection {
    /// Returns a list of all enum values.
    fn all() -> Vec<Self>  where Self: Sized;
}

/// Choose a enum from a list of options.
pub struct Chooser<TRng> {
    rng: TRng
}

impl<TRng: Rng> Chooser<TRng> {
    pub fn new(rng: TRng) -> Self {
        Self { rng }
    }

    /// Choose an enum item from the provided [Vec].
    /// If none are provided, a random one of all enum values is chosen.
    pub fn choose<TValue: Collection>(&mut self, selection: Vec<TValue>) -> TValue {
        let options = if selection.is_empty() {
            TValue::all()
        } else {
            selection
        };
        options.into_iter().choose_stable(&mut self.rng).unwrap()
    }
}

#[cfg(test)]
mod test {
    use rand::rngs::mock::StepRng;
    use crate::{Chooser, Collection};

    enum MockOptions {
        First,
        Second,
        Third
    }

    impl Collection for MockOptions {
        fn all() -> Vec<Self> where Self: Sized {
            use MockOptions::*;

            vec![First, Second, Third]
        }
    }

    #[test]
    fn choose() {
        let rng = StepRng::new(0, 1);
        let mut chooser = Chooser::new(rng);

        assert!(matches!(chooser.choose(vec![MockOptions::First, MockOptions::Second]), MockOptions::Second));
    }

    #[test]
    fn choose_empty() {
        let rng = StepRng::new(0, 1);
        let mut chooser = Chooser::new(rng);

        assert!(matches!(chooser.choose(Vec::new()), MockOptions::Third));
    }
}