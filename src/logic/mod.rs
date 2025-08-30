use std::cmp;

use crate::constants;

pub fn calc_turbine_demand(demand: u32) -> u32 {
    let optimal_demand = cmp::min(demand + constants::MOE, constants::MAX_DEMAND);

    optimal_demand / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demands() {
        assert_eq!(calc_turbine_demand(1200), (1200 + 24) / 2);
        assert_eq!(calc_turbine_demand(1475), (1475 + 24) / 2);
        assert_eq!(calc_turbine_demand(1476), (1476 + 24) / 2);
        assert_eq!(calc_turbine_demand(1477), 1500 / 2);
        assert_eq!(calc_turbine_demand(1500), 1500 / 2);
    }
}
