use std::sync::TryLockError::Poisoned;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if other.rh_factor == RhFactor::Positive && self.rh_factor == RhFactor::Negative {
            return false
        }
        if other.rh_factor == RhFactor::Negative
        match &self.antigen {
            Antigen::A => {
                match &self.rh_factor {
                    RhFactor::Positive => {

                    },
                    RhFactor::Negative => {

                    }
                }
            },

        }
    }

    pub fn donors(&self) -> Vec<Self> {
    }

    pub fn recipients(&self) -> Vec<Self> {
    }
}