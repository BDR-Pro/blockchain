pub mod libp2p;

fn main(){
    let result = libp2p::main();

    println!("{:?}", result);
}