// Modeling State Transitions with Enums
// Run: cargo run --bin state_machine

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// NOTE: Takes &TrafficLight (borrow) — no need to consume the current state to produce the next.
fn next_state(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red    => TrafficLight::Green,
        TrafficLight::Green  => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
    }
}

// NOTE: Returns &'static str — string literals live forever, no allocation needed.
fn name(light: &TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red    => "Red",
        TrafficLight::Green  => "Green",
        TrafficLight::Yellow => "Yellow",
    }
}

fn simulate(steps: usize) -> Vec<String> {
    // NOTE: Collect the initial state, then transition `steps` times.
    // Total entries = steps + 1 (initial + one per transition).
    let mut light = TrafficLight::Red;
    let mut result = Vec::with_capacity(steps + 1);
    result.push(name(&light).to_string());
    for _ in 0..steps {
        light = next_state(&light);
        result.push(name(&light).to_string());
    }
    result
}

fn main() {
    println!("{:?}", simulate(6)); // ["Red", "Green", "Yellow", "Red", "Green", "Yellow"]
    println!("{:?}", simulate(1)); // ["Red"]
    println!("{:?}", simulate(0)); // []
}
