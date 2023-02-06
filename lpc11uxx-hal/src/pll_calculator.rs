use embedded_time::rate::Kilohertz;

const FCCO_MIN: Kilohertz = Kilohertz(156);
const FCCO_MAX: Kilohertz = Kilohertz(320);

fn calculate_pll_params(target: Kilohertz, input: Kilohertz) -> (u8, u8) {
    let m = target / input;

    
}
