use crate::oracles::CFMMOracle;
use ethers_addressbook::Contract;
use ethers::types::U256;
use std::{collections::HashMap, ops::{Mul, MulAssign}, hash::Hash};

/// The CFMM trait should be implemented for
pub trait CFMM<const N: usize> {
    /// trade_function returns the output values for the given trade function.
    fn trade_output(&self, input: HashMap<Contract, U256>) -> HashMap<Contract, U256>;
}

/// FeelessCPMM is a constant product market maker with no fees.
pub struct FeelessCPMM {
    /// The CPMM reserves map each token to the amount in reserves.
    reserves: HashMap<Contract, U256>,

    /// This constant is updated with each modification of the reserves.
    constant: U256,
}

/// A reserve describes a CFMM's reserves for a specific token.
pub struct Reserve {
    /// The amount of the specified token held in the reserves.
    amount: U256,

    /// The token held in reserves.
    token: Contract,
}

impl FeelessCPMM {
    /// Creates a new FeelessCPMM based on the reserves passed. This calculates and stores the
    /// constant product of the CPMM
    pub fn new(reserves: HashMap<Contract, U256>) -> Self {
        let product = reserves.values().fold(U256::one(), |acc, &item| acc * item);
        FeelessCPMM { reserves, constant: product }
    }

    /// Calculates the current product based on the reserves.
    fn calculate_product(&self) -> U256 {
        self.reserves.values().fold(U256::one(), |acc, &item| acc * item)
    }

    /// Calculates the tokens and amounts that will be received given a valid trade is
    /// made using the input tokens.
    fn trade_output(&self, input: HashMap<Contract, U256>) -> HashMap<Contract, U256> {
        // TODO: this might only be possible with some conditions on the input, for example if we
        // only have two assets and only have input for one, it's trivial to find the output.
        // However, if we have three assets and don't specify output, there are multiple solutions.
        todo!()
    }

    /// Performs a trade given the input tokens, updating the reserves.
    fn make_trade(&mut self, input: HashMap<Contract, U256>, output: HashMap<Contract, U256>) {
        // TODO: Determine output type for this
        todo!()
    }

    /// Determines whether or not the given inputs and outputs preserve the constant product.
    fn is_trade_valid(&self, input: HashMap<Contract, U256>, output: HashMap<Contract, U256>) -> bool {
        // compute the new reserves given the
        let mut new_reserves = self.reserves.clone();
        for (token, value) in input {
            let token_value = new_reserves.get_mut(&token).unwrap();
            *token_value += value;
            *token_value -= *output.get(&token).unwrap();
        }
        let new_product = new_reserves.values().fold(U256::one(), |acc, &item| acc * item);
        new_product == self.constant
    }
}

impl<const N: usize> CFMMOracle<N> for FeelessCPMM {
    fn price(pair: [Contract; N]) -> [U256; N] {
        todo!()
    }

    fn validity(pair: [Contract; N], input: [U256; N], output: [U256; N]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers_addressbook::contract;

    #[test]
    fn test_two_asset_valid_trade() {
        let mut assets: HashMap<Contract, U256> = HashMap::new();
        let usdc = contract("usdc").unwrap();
        let dai = contract("dai").unwrap();
        assets.insert(usdc.clone(), 100.into());
        assets.insert(dai.clone(), 101.into());
        // product = 10100
        let cpmm = FeelessCPMM::new(assets);

        // let's move it to the other side - take 1 dai, add 1 usdc
        let mut inputs: HashMap<Contract, U256> = HashMap::new();
        inputs.insert(usdc.clone(), 1.into());
        inputs.insert(dai.clone(), 0.into());

        let mut outputs: HashMap<Contract, U256> = HashMap::new();
        outputs.insert(usdc, 0.into());
        outputs.insert(dai, 1.into());
        assert!(cpmm.is_trade_valid(inputs, outputs));
    }
}
