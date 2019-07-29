mod errors;
mod tis100;

fn main() {
    let sequence = tis100::sample_code();
    println!("TIS Sequence is:\n{}", sequence);

    let nodes = tis100::parse(sequence).expect("error while parsing sequence");
    println!("Starting state:");
    for (i, node) in nodes.iter().enumerate()  {
        println!("Node {}: {:?}", i, node);
    }

    let ticks = 3;
    let nodes = tis100::emu::tick_all_n(nodes, ticks);
    println!("State after {} ticks:", ticks);
    for (i, node) in nodes.iter().enumerate()  {
        println!("Node {}: {:?}", i, node);
    }
}