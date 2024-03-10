use crate::math::complex::Complex64;
use std::mem::replace;
use std::ops::Add;

use godot::engine::{Node, RefCounted};
use godot::prelude::*;

type Frequency = f64;

// An AcNode represents a node in the electrical diagram.
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcNode {
    #[base]
    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcComponent {
    #[base]
    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcBridge {
    #[base]
    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AcSystem {
    #[base]
    base: Base<Node>,
}

#[derive(Clone)]
enum InternalAcVoltage {
    None,
    One(InternalAcVoltageElement),
    Many(InternalAcVoltageCollection),
}

#[derive(Clone)]
struct InternalAcVoltageCollection {
    // Invariant: The frequency is strictly increasing
    // Invariant: The frequencies are positive
    voltage_elements: Vec<InternalAcVoltageElement>,
}

#[derive(Clone, Copy, PartialEq)]
/// Represents a sinusoidal AC voltage with a given frequency and of the form a + jb
struct InternalAcVoltageElement {
    frequency: Frequency,
    complex_representation: Complex64,
}

pub trait Differentiable {
    type Output;
    fn derivative(&self) -> Self::Output;
}

pub trait AcVoltage {
    fn get_voltage_at_time(&self, time: f64) -> f64;
}

impl Differentiable for InternalAcVoltageElement {
    type Output = InternalAcVoltageElement;
    fn derivative(&self) -> Self::Output {
        Self {
            frequency: self.frequency,
            complex_representation: self.complex_representation.times_imaginary() * self.frequency,
        }
    }
}

impl AcVoltage for InternalAcVoltageElement {
    fn get_voltage_at_time(&self, time: f64) -> f64 {
        (self.complex_representation * Complex64::from_modulus_argument(time, 1.0)).re
    }
}

impl Differentiable for InternalAcVoltageCollection {
    type Output = InternalAcVoltageCollection;
    fn derivative(&self) -> Self::Output {
        let result = self.clone();
        let v: Vec<_> = result
            .voltage_elements
            .iter()
            .map(|x| x.derivative())
            .collect();
        InternalAcVoltageCollection {
            voltage_elements: v,
        }
    }
}

impl InternalAcVoltageCollection {
    /// Panics if the inputs have the same frequency
    fn from_unequal_pair(a: InternalAcVoltageElement, b: InternalAcVoltageElement) -> Self {
        let mut result = InternalAcVoltageCollection {
            voltage_elements: Vec::with_capacity(2),
        };
        if a.frequency == b.frequency {
            panic!("Invariant breached: frequencies must not be equal");
        }
        if a.frequency < b.frequency {
            result.voltage_elements[0] = a;
            result.voltage_elements[1] = b;
        } else {
            result.voltage_elements[0] = b;
            result.voltage_elements[1] = a;
        }
        result.assert_invariants();
        result
    }
    fn insert(&mut self, inserted_element: InternalAcVoltageElement) {
        for (index, iter_element) in self.voltage_elements.iter_mut().enumerate() {
            if iter_element.frequency == inserted_element.frequency {
                // If the frequency matches, add to that frequency
                iter_element.complex_representation += inserted_element.complex_representation;
                return;
            } else if iter_element.frequency > inserted_element.frequency {
                // If we find an element where the frequency is lesser, insert before it
                self.voltage_elements.insert(index, inserted_element);
                return;
            }
        }
        // We didn't insert previously, so this has the greatest frequency and we append it.
        self.voltage_elements.push(inserted_element);
        self.assert_invariants();
    }
    fn merge(&mut self, other: InternalAcVoltageCollection) {
        let maximal_merged_length = self.voltage_elements.len() + other.voltage_elements.len();
        let old_element_list = replace(
            &mut self.voltage_elements,
            Vec::with_capacity(maximal_merged_length),
        );

        // basically a merge operation
        let list_a = old_element_list;
        let list_b = other.voltage_elements;
        let mut index_a: usize = 0;
        let mut index_b: usize = 0;
        let output_list = &mut self.voltage_elements;

        let remaining_slice = loop {
            let front_a = list_a[index_a];
            let front_b = list_b[index_b];

            match front_a
                .frequency
                .partial_cmp(&front_b.frequency)
                .expect("InternalAcVoltageCollection invariant: frequencies must not be NaN")
            {
                std::cmp::Ordering::Less => {
                    index_a += 1;

                    output_list.push(front_a);
                }
                std::cmp::Ordering::Greater => {
                    index_b += 1;

                    output_list.push(front_b);
                }
                std::cmp::Ordering::Equal => {
                    index_a += 1;
                    index_b += 1;

                    let mut append_element = front_a;
                    append_element.complex_representation += front_b.complex_representation;
                    output_list.push(append_element);
                }
            }

            if index_a == list_a.len() {
                break &list_b[index_b..];
            }
            if index_b == list_b.len() {
                break &list_a[index_a..];
            }
        };

        output_list.extend_from_slice(remaining_slice);
        self.assert_invariants();
    }

    /// Can panic if invariants fail
    fn assert_invariants(&self) {
        // Invariant: The frequency is strictly increasing
        // Invariant: The frequencies are positive
        // Invariant: The frequencies are finite and not NaN
        let mut least_seen_frequency: Frequency = 0.0;
        for element in self.voltage_elements.iter() {
            if element.frequency > least_seen_frequency {
                least_seen_frequency = element.frequency;
            } else {
                if element.frequency <= 0.0 {
                    panic!("InternalAcVoltageCollection invariant breached: element frequency is not positive");
                } else {
                    panic!("InternalAcVoltageCollection invariant breached: element frequency is not strictly increasing");
                }
            }

            if element.frequency.is_nan() {
                panic!("InternalAcVoltageCollection invariant breached: element frequency is NaN");
            }
            if element.frequency.is_infinite() {
                panic!(
                    "InternalAcVoltageCollection invariant breached: element frequency is infinite"
                );
            }
        }
    }
}

impl Differentiable for InternalAcVoltage {
    type Output = InternalAcVoltage;
    fn derivative(&self) -> Self::Output {
        match self {
            InternalAcVoltage::None => InternalAcVoltage::None,
            InternalAcVoltage::One(e) => InternalAcVoltage::One(e.derivative()),
            InternalAcVoltage::Many(e) => InternalAcVoltage::Many(e.derivative()),
        }
    }
}

impl Add for InternalAcVoltage {
    type Output = InternalAcVoltage;
    fn add(self, rhs: InternalAcVoltage) -> Self::Output {
        match self {
            InternalAcVoltage::None => rhs,
            InternalAcVoltage::One(lhs_element) => match rhs {
                InternalAcVoltage::None => InternalAcVoltage::One(lhs_element),
                InternalAcVoltage::One(rhs_element) => {
                    if lhs_element.frequency == rhs_element.frequency {
                        InternalAcVoltage::One(InternalAcVoltageElement {
                            frequency: lhs_element.frequency,
                            complex_representation: lhs_element.complex_representation
                                + rhs_element.complex_representation,
                        })
                    } else {
                        InternalAcVoltage::Many(InternalAcVoltageCollection::from_unequal_pair(
                            lhs_element,
                            rhs_element,
                        ))
                    }
                }
                InternalAcVoltage::Many(mut rhs_collection) => {
                    rhs_collection.insert(lhs_element);
                    InternalAcVoltage::Many(rhs_collection)
                }
            },
            InternalAcVoltage::Many(mut lhs_collection) => match rhs {
                InternalAcVoltage::None => InternalAcVoltage::Many(lhs_collection),
                InternalAcVoltage::One(rhs_element) => {
                    lhs_collection.insert(rhs_element);
                    InternalAcVoltage::Many(lhs_collection)
                }
                InternalAcVoltage::Many(rhs_collection) => {
                    let (mut merging, consumed) = if lhs_collection.voltage_elements.len()
                        > rhs_collection.voltage_elements.len()
                    {
                        (rhs_collection, lhs_collection)
                    } else {
                        (lhs_collection, rhs_collection)
                    };
                    merging.merge(consumed);
                    InternalAcVoltage::Many(merging)
                }
            },
        }
    }
}

impl From<()> for InternalAcVoltage {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<InternalAcVoltageElement> for InternalAcVoltage {
    fn from(value: InternalAcVoltageElement) -> Self {
        Self::One(value)
    }
}

impl Default for InternalAcVoltage {
    fn default() -> Self {
        Self::None
    }
}
