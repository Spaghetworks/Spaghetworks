use std::mem::replace;
use std::ops::Add;

use super::complex::Complex64;
use super::derivative::Differentiable;

type Frequency = f64;

#[derive(Clone)]
pub struct FourierSeries {
    // Invariant: The frequency is strictly increasing
    // Invariant: The frequencies are positive
    elements: Vec<FourierSeriesElement>,
}

#[derive(Clone, Copy, PartialEq)]
/// Represents a sinusoidal AC voltage with a given frequency and of the form a + jb
pub struct FourierSeriesElement {
    frequency: Frequency,
    complex_representation: Complex64,
}

impl Differentiable for FourierSeriesElement {
    type Output = FourierSeriesElement;
    fn derivative(&self) -> Self::Output {
        Self {
            frequency: self.frequency,
            complex_representation: self.complex_representation.times_imaginary() * self.frequency,
        }
    }
}

impl Differentiable for FourierSeries {
    type Output = FourierSeries;
    fn derivative(&self) -> Self::Output {
        let v: Vec<_> = self.elements.iter().map(|x| x.derivative()).collect();
        FourierSeries { elements: v }
    }
}

impl FourierSeries {
    pub fn new() -> Self {
        FourierSeries {
            elements: Vec::new(),
        }
    }

    pub fn get(&self, time: f64) -> Complex64 {
        let mut sum = Complex64::ZERO;
        for element in self.elements.iter() {
            sum += Complex64::from_modulus_argument(element.frequency * time, 1.0)
                * element.complex_representation
        }
        sum
    }

    pub fn insert(&mut self, inserted_element: FourierSeriesElement) {
        for (index, iter_element) in self.elements.iter_mut().enumerate() {
            if iter_element.frequency == inserted_element.frequency {
                // If the frequency matches, add to that frequency
                iter_element.complex_representation += inserted_element.complex_representation;
                return;
            } else if iter_element.frequency > inserted_element.frequency {
                // If we find an element where the frequency is lesser, insert before it
                self.elements.insert(index, inserted_element);
                return;
            }
        }
        // We didn't insert previously, so this has the greatest frequency and we append it.
        self.elements.push(inserted_element);
        self.assert_invariants();
    }

    pub fn clean_zero_elements(&mut self) {
        self.elements
            .retain(|element| element.complex_representation != Complex64::ZERO)
    }

    /// Can panic if invariants fail
    fn assert_invariants(&self) {
        // Invariant: The frequency is strictly increasing
        // Invariant: The frequencies are positive
        // Invariant: The frequencies are finite and not NaN
        let mut least_seen_frequency: Frequency = 0.0;
        for element in self.elements.iter() {
            if element.frequency > least_seen_frequency {
                least_seen_frequency = element.frequency;
            } else {
                if element.frequency <= 0.0 {
                    panic!("FourierSeries invariant breached: element frequency is not positive");
                } else {
                    panic!("FourierSeries invariant breached: element frequency is not strictly increasing");
                }
            }

            if element.frequency.is_nan() {
                panic!("FourierSeries invariant breached: element frequency is NaN");
            }
            if element.frequency.is_infinite() {
                panic!("FourierSeries invariant breached: element frequency is infinite");
            }
        }
    }
}

impl Default for FourierSeries {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for FourierSeries {
    type Output = FourierSeries;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.clean_zero_elements();
        rhs.clean_zero_elements();

        let maximal_merged_length = self.elements.len() + rhs.elements.len();
        let old_element_list = replace(
            &mut self.elements,
            Vec::with_capacity(maximal_merged_length),
        );

        // basically a merge operation
        let list_a = old_element_list;
        let list_b = rhs.elements;
        let mut index_a: usize = 0;
        let mut index_b: usize = 0;
        let output_list = &mut self.elements;

        let remaining_slice = loop {
            let front_a = list_a[index_a];
            let front_b = list_b[index_b];

            match front_a
                .frequency
                .partial_cmp(&front_b.frequency)
                .expect("FourierSeries invariant: frequencies must not be NaN")
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
        self
    }
}
