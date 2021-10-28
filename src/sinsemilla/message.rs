//! Gadget and chips for the Sinsemilla hash function.
use crate::utilities::{CellValue, Var};
use ff::PrimeFieldBits;
use halo2::{arithmetic::FieldExt, circuit::Cell};
use std::fmt::Debug;

/// A [`Message`] composed of several [`MessagePiece`]s.
#[derive(Clone, Debug)]
pub struct Message<F: FieldExt, const K: usize, const MAX_WORDS: usize>(Vec<MessagePiece<F, K>>);

impl<F: FieldExt + PrimeFieldBits, const K: usize, const MAX_WORDS: usize>
    From<Vec<MessagePiece<F, K>>> for Message<F, K, MAX_WORDS>
{
    fn from(pieces: Vec<MessagePiece<F, K>>) -> Self {
        // A message cannot contain more than `MAX_WORDS` words.
        assert!(pieces.iter().map(|piece| piece.num_words()).sum::<usize>() < MAX_WORDS);
        Message(pieces)
    }
}

impl<F: FieldExt + PrimeFieldBits, const K: usize, const MAX_WORDS: usize> std::ops::Deref
    for Message<F, K, MAX_WORDS>
{
    type Target = [MessagePiece<F, K>];

    fn deref(&self) -> &[MessagePiece<F, K>] {
        &self.0
    }
}

/// A [`MessagePiece`] of some bitlength.
///
/// The piece must fit within a base field element, which means its length
/// cannot exceed the base field's `NUM_BITS`.
#[derive(Copy, Clone, Debug)]
pub struct MessagePiece<F: FieldExt, const K: usize> {
    cell_value: CellValue<F>,
    /// The number of K-bit words in this message piece.
    num_words: usize,
}

impl<F: FieldExt + PrimeFieldBits, const K: usize> MessagePiece<F, K> {
    pub fn new(cell: Cell, field_elem: Option<F>, num_words: usize) -> Self {
        assert!(num_words * K < F::NUM_BITS as usize);
        let cell_value = CellValue::new(cell, field_elem);
        Self {
            cell_value,
            num_words,
        }
    }

    pub fn num_words(&self) -> usize {
        self.num_words
    }

    pub fn cell(&self) -> Cell {
        self.cell_value.cell()
    }

    pub fn field_elem(&self) -> Option<F> {
        self.cell_value.value()
    }

    pub fn cell_value(&self) -> CellValue<F> {
        self.cell_value
    }
}
