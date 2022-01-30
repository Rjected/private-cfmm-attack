use ethers_addressbook::Contract;
use ethers::types::U256;

// TODO: change types to something more readable and unambiguous, like sets or maps

/// A CFMMOracle implements price and validity functions for a CFMM whose state may be changing.
/// This is the only information the adversary can get about the CFMM reserves, note that none of
/// the signatures include reserve information.
pub trait CFMMOracle<const N: usize> {
    /// price gets the current marginal price of the given market
    fn price(pair: [Contract; N]) -> [U256; N];

    /// validity gets the validity of a given market at the current reserves
    fn validity(pair: [Contract; N], input: [U256; N], output: [U256; N]) -> bool;
}
