use std::boxed::Box;

pub fn pop_amount(list: &mut Vec<u8>, amount: usize) -> Vec<u8> {
    return list[0..amount - 1].to_vec();
}