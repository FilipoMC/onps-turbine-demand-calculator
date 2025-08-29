use std::cmp;

use crate::constants;

pub fn calc_turbine_demand(demand: u32) -> u32 {
    let demand = cmp::min(demand + 24, constants::MAX_DEMAND);

    demand / 2
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
