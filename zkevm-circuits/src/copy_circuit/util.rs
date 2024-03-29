use crate::util::word::WordLoHi;
use bus_mapping::circuit_input_builder::NumberOrHash;
use eth_types::Field;
use halo2_proofs::circuit::Value;

/// Encode the type `NumberOrHash` into a field element
pub fn number_or_hash_to_word<F: Field>(v: &NumberOrHash) -> WordLoHi<Value<F>> {
    match v {
        NumberOrHash::Number(n) => WordLoHi::from(*n as u64).into_value(),
        NumberOrHash::Hash(h) => WordLoHi::from(*h).into_value(),
    }
}
