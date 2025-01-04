use anchor_lang::prelude::*;
use crate::utils::math::Q64_64;
use crate::error::ErrorCode;

pub(crate) trait CpAmmCalculate {
    const LP_MINT_INITIAL_DECIMALS: u8 = 5;
    // 0.0001% f64 = 0.000001
    const SWAP_CONSTANT_PRODUCT_TOLERANCE: Q64_64 = Q64_64::new(18446744073710);
    // 0.0001% f64 = 0.000001
    const ADJUST_LIQUIDITY_RATIO_TOLERANCE: Q64_64 = Q64_64::new(18446744073710);
    const FEE_MAX_BASIS_POINTS: u128 = 10000;
    fn constant_product_sqrt(&self) -> Q64_64;
    fn base_quote_ratio(&self) -> Q64_64;
    fn base_liquidity(&self) -> u64;
    fn quote_liquidity(&self) -> u64;
    fn lp_tokens_supply(&self) -> u64;
    fn providers_fee_rate_basis_points(&self) -> u16;
    fn protocol_fee_rate_basis_points(&self) -> u16;
    fn calculate_launch_lp_tokens(constant_product_sqrt: Q64_64) -> Result<(u64, u64)>{
        let lp_tokens_supply = constant_product_sqrt.to_u64();
        require!(lp_tokens_supply > 0, ErrorCode::LpTokensCalculationFailed);

        let initial_locked_liquidity = 10_u64.pow(Self::LP_MINT_INITIAL_DECIMALS as u32);

        let difference = lp_tokens_supply.checked_sub(initial_locked_liquidity).ok_or(ErrorCode::LaunchLiquidityTooSmall)?;
        require!(difference >= initial_locked_liquidity << 2, ErrorCode::LaunchLiquidityTooSmall);
        Ok((lp_tokens_supply, difference))
    }
    fn calculate_lp_mint_for_provided_liquidity(&self, new_constant_product_sqrt: Q64_64) -> Option<u64> {
        let provided_liquidity = new_constant_product_sqrt.checked_sub(self.constant_product_sqrt())?;

        let share_from_current_liquidity = provided_liquidity.checked_div(self.constant_product_sqrt())?;
        let tokens_to_mint = share_from_current_liquidity.checked_mul(Q64_64::from_u64(self.lp_tokens_supply()))?.to_u64();
        if tokens_to_mint == 0{
            return None;
        }
        Some(tokens_to_mint)
    }
    fn calculate_liquidity_from_share(&self, lp_tokens: u64) -> Option<(u64, u64)>{
        if lp_tokens == 0{
            return None;
        }
        let liquidity_share = Q64_64::from_u64(lp_tokens).checked_div(Q64_64::from_u64(self.lp_tokens_supply()))?;
        let constant_product_sqrt_share = self.constant_product_sqrt().checked_mul(liquidity_share)?;

        // Sqrt liquidity?
        let base_withdraw_square = constant_product_sqrt_share.checked_square_mul_as_u128(self.base_quote_ratio())?;
        let quote_withdraw_square = constant_product_sqrt_share.checked_square_div_as_u128(self.base_quote_ratio())?;
        
        let base_withdraw = Q64_64::sqrt_from_u128(base_withdraw_square).to_u64();
        let quote_withdraw = Q64_64::sqrt_from_u128(quote_withdraw_square).to_u64();
        
        if base_withdraw == 0 || quote_withdraw == 0{
            return None;
        }
        Some((base_withdraw, quote_withdraw))
    }
    fn calculate_afterswap_liquidity(&self, swap_amount: u64, is_in_out: bool) -> Option<(u64, u64)>{
        let mut new_base_liquidity = 0;
        let mut new_quote_liquidity = 0;
        if is_in_out {
            new_base_liquidity = self.base_liquidity().checked_add(swap_amount)?;
            new_quote_liquidity = self.calculate_opposite_liquidity(new_base_liquidity)?;
        }
        else{
            new_quote_liquidity = self.quote_liquidity().checked_add(swap_amount)?;
            new_base_liquidity = self.calculate_opposite_liquidity(new_quote_liquidity)?;
        }
        Some((new_base_liquidity, new_quote_liquidity))
    }

    fn validate_and_calculate_liquidity_ratio(&self, new_base_liquidity: u64, new_quote_liquidity: u64) -> Result<Q64_64>{
        let new_base_quote_ratio_sqrt = Self::calculate_base_quote_ratio(new_base_liquidity, new_quote_liquidity).ok_or(ErrorCode::BaseQuoteRatioCalculationFailed)?;
        let difference = self.base_quote_ratio().abs_diff(new_base_quote_ratio_sqrt);
        let allowed_difference = self.base_quote_ratio() * Self::ADJUST_LIQUIDITY_RATIO_TOLERANCE;
        require!(difference <= allowed_difference, ErrorCode::LiquidityRatioToleranceExceeded);
        Ok(new_base_quote_ratio_sqrt)
    }
    fn validate_swap_constant_product(&self, new_base_liquidity: u64, new_quote_liquidity: u64) -> Result<()>{
        let new_constant_product_sqrt = Self::calculate_constant_product_sqrt(new_base_liquidity, new_quote_liquidity).ok_or(ErrorCode::ConstantProductCalculationFailed)?;
        let difference = self.constant_product_sqrt().abs_diff(new_constant_product_sqrt);
        let allowed_difference = self.constant_product_sqrt() * Self::SWAP_CONSTANT_PRODUCT_TOLERANCE;
        require!(difference <= allowed_difference, ErrorCode::ConstantProductToleranceExceeded);
        Ok(())
    }
    fn calculate_protocol_fee_amount(&self, swap_amount: u64) -> u64{
        ((swap_amount as u128) * (self.protocol_fee_rate_basis_points() as u128) / Self::FEE_MAX_BASIS_POINTS) as u64
    }
    fn calculate_providers_fee_amount(&self, swap_amount: u64) -> u64{
        ((swap_amount as u128) * (self.providers_fee_rate_basis_points() as u128) / Self::FEE_MAX_BASIS_POINTS) as u64
    }
    fn calculate_opposite_liquidity(&self, x_liquidity: u64) -> Option<u64>{
        let opposite_liquidity = self.constant_product_sqrt().checked_square_div_as_u64(Q64_64::from_u64(x_liquidity))?;
        if opposite_liquidity == 0 {
            return None;
        }
        Some(opposite_liquidity)
    }

    fn check_swap_result(swap_result: u64, estimated_swap_result: u64, allowed_slippage:u64) -> Result<()>{
        require!(swap_result > 0, ErrorCode::SwapResultIsZero);
        require!(swap_result.abs_diff(estimated_swap_result) <= allowed_slippage, ErrorCode::SwapSlippageExceeded);
        Ok(())
    }
    fn calculate_base_quote_ratio(base_liquidity: u64, quote_liquidity: u64) -> Option<Q64_64>{
        if base_liquidity == 0 || quote_liquidity == 0 {
            return None
        }
        let ratio = Q64_64::from_u64(base_liquidity) / Q64_64::from_u64(quote_liquidity);
        if ratio.is_zero(){
            return None
        }
        Some(ratio)
    }
    fn calculate_constant_product_sqrt(base_liquidity: u64, quote_liquidity: u64) -> Option<Q64_64>{
        if base_liquidity == 0 || quote_liquidity == 0 {
            return None
        }
        let constant_product_sqrt = Q64_64::sqrt_from_u128(base_liquidity as u128 * quote_liquidity as u128);
        if constant_product_sqrt.is_zero(){
            return None
        }
        Some(constant_product_sqrt)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::cp_amm::CpAmmCalculate;
    use crate::utils::math::Q64_64;

    struct TestCpAmm{
        base_liquidity: u64,
        quote_liquidity: u64,
        constant_product_sqrt: Q64_64,
        base_quote_ratio: Q64_64,
        lp_tokens_supply: u64,
        providers_fee_rate_basis_points: u16,
        protocol_fee_rate_basis_points: u16,
    }
    impl CpAmmCalculate for TestCpAmm {
        fn constant_product_sqrt(&self) -> Q64_64 {
            self.constant_product_sqrt
        }

        fn base_quote_ratio(&self) -> Q64_64 {
            self.base_quote_ratio
        }

        fn base_liquidity(&self) -> u64 {
            self.base_liquidity
        }

        fn quote_liquidity(&self) -> u64 {
            self.quote_liquidity
        }

        fn lp_tokens_supply(&self) -> u64 {
            self.lp_tokens_supply
        }

        fn providers_fee_rate_basis_points(&self) -> u16 {
            self.providers_fee_rate_basis_points
        }

        fn protocol_fee_rate_basis_points(&self) -> u16 {
            self.protocol_fee_rate_basis_points
        }
    }

    mod unit_tests {
        use super::*;
        #[test]
        fn test_calculate_none_base_quote_ratio_extreme() {
            let liquidity1: u64 = 0;
            let liquidity2: u64 = 0;
            let liquidity3: u64 = 1;
            let liquidity4: u64 = u64::MAX;
            let result1 = 5.421010862427522e-20;
            let result2 = 1.8446744073709552e19;
            assert_eq!(TestCpAmm::calculate_base_quote_ratio(liquidity3, liquidity4).unwrap().to_f64(), result1);
            assert_eq!(TestCpAmm::calculate_base_quote_ratio(liquidity4, liquidity3).unwrap().to_f64(), result2);
            assert!(TestCpAmm::calculate_base_quote_ratio(liquidity1, liquidity2).is_none());
        }
        #[test]
        fn test_calculate_base_quote_ratio_sqrt() {
            let liquidity1: u64 = 250;
            let liquidity2: u64 = 100;
            let result1 = 2.5;
            let result2 = 0.4;
            assert_eq!(TestCpAmm::calculate_base_quote_ratio(liquidity1, liquidity2).unwrap().to_f64(), result1);
            assert_eq!(TestCpAmm::calculate_base_quote_ratio(liquidity2, liquidity1).unwrap().to_f64(), result2);
        }
        #[test]
        fn test_calculate_constant_product_sqrt_extreme() {
            let liquidity1: u64 = 0;
            let liquidity2: u64 = u64::MAX;
            let liquidity3: u64 = 1;
            assert!(TestCpAmm::calculate_constant_product_sqrt(liquidity1, liquidity2).is_none());
            assert_eq!(TestCpAmm::calculate_constant_product_sqrt(liquidity2, liquidity2).unwrap().raw_value(), (u64::MAX as u128) << 64);
            assert_eq!(TestCpAmm::calculate_constant_product_sqrt(liquidity3, liquidity3).unwrap().raw_value(), 1 << 64);
        }
        #[test]
        fn test_calculate_constant_product_sqrt() {
            let liquidity1: u64 = 250;
            let liquidity2: u64 = 100;
            let result = ((liquidity1 * liquidity2) as f64).sqrt();
            assert_eq!(TestCpAmm::calculate_constant_product_sqrt(liquidity1, liquidity2).unwrap().to_f64(), result);
        }
        
    }
    mod fuzz_tests {
        use super::*;
        use proptest::prelude::*;
        fn arbitrary_u128() -> impl Strategy<Value = u128> {
            prop_oneof![
            0..=u128::MAX,
            Just(0),
            Just(1),
            Just(2),
            Just(u128::MAX),
            Just(u128::MAX / 2),
            Just(u128::MAX - 1),
            Just(1024),
            Just(4095),
            Just(8191),
            Just(10000),
            Just(12321),
            Just(65535),
            Just(12345),
            Just(54321),
            Just(99999),
            Just(45678),
            Just(87654),
            Just(10001),
            Just(9999),
            Just(2047),
            Just(65534)
            ]
        }
    }
}