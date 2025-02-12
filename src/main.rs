use epic_auto_towers_simulations::{simulator::Simulator, strategies::spirit_flower_switch::SpiritFlowerSwitch};

fn main() {
    let sim = Simulator::new();
    println!("Switch,Round,Level,Attack,Life,EXP,Levelup_EXP,Chakra");
    for switch in 22..68 {
        print!("{switch},");
        sim.simulate(SpiritFlowerSwitch { round: switch });
    }
}
