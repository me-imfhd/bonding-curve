use std::f64::consts::E;

fn calculate_new_initial_price(current_supply: u64, initial_price: f64) -> f64 {
    let k = 2.8e-9;
    let base_supply = 800_000_000;
    // Add exponential scaling factor that increases as supply decreases
    let supply_ratio = (base_supply - current_supply) as f64 / base_supply as f64;
    let scaling_factor = 1.0 + supply_ratio.powf(2.0); // Quadratic scaling

    // Combine base exponential growth with scaling factor
    initial_price * E.powf(k * (base_supply - current_supply) as f64 * scaling_factor)
}

fn cal_cost_effecient(x: u64, initial: &f64) -> f64 {
    let k = 2.8e-9;
    let r = E.powf(k); // common ratio
    let a = initial * r; // first term
    a * (1.0 - r.powf(x as f64)) / (1.0 - r)
}

fn main() {
    let amts: Vec<u64> = (1..800_000_000).step_by(10_000_000).collect();
    let purchases: Vec<u64> = (1..800_000_000).step_by(10_000_000).collect();
    let mut supply = 800_000_000;
    let mut initial_price = 2.8e-8;

    println!("Starting supply: {}", supply);

    for (index, &purchase_amount) in purchases.iter().enumerate() {
        println!("\n=== Purchase #{} ===", index + 1);
        println!("Current supply: {}", supply);
        println!("Current initial price: {:.11}", initial_price);

        for &lot_size in &amts {
            let tot_price = cal_cost_effecient(lot_size, &initial_price);
            println!(
                "Lot: {:>9} | Total cost: {:.11} | Cost per token: {:.11}",
                lot_size,
                tot_price,
                tot_price / lot_size as f64
            );
        }

        let purchase_cost = cal_cost_effecient(purchase_amount, &initial_price);
        supply -= purchase_amount;
        initial_price = calculate_new_initial_price(supply, 2.8e-8);

        println!("\nPurchase executed:");
        println!("Tokens purchased: {}", purchase_amount);
        println!("Purchase cost: {:.11}", purchase_cost);
        println!("New supply: {}", supply);
        println!("New initial price: {:.11}", initial_price);
        println!("=============================");
    }
}
