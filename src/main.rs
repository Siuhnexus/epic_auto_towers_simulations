use epic_auto_towers_simulations::{csv_logger, logger::CSVLogger, simulator::{LoggingStrategy, Simulator}, strategies::spirit_ladder_switch::SpiritLadderSwitch};

fn main() {
    let sim = Simulator::new(LoggingStrategy::JustResults);
    let mut logger: CSVLogger = csv_logger!("Switch", "Gold", "Round", "Level", "XSum", "Chakra");
    for switch in 22..105 {
        logger.write(format!("{},", switch));
        sim.simulate(&mut logger, &SpiritLadderSwitch { round: switch });
    }
    logger.print();
}
