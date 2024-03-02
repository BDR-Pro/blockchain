use libp2p::{
    swarm::{SwarmBuilder, SwarmEvent},
    identity, mdns::{Mdns, MdnsConfig}, tcp::TokioTcpConfig,
    yamux::YamuxConfig, noise,
    Multiaddr, PeerId, Transport,
    core::upgrade,
    mplex, NetworkBehaviour,
};
use async_std::task;
use std::{error::Error, task::{Context, Poll}};

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    floodsub: libp2p::floodsub::Floodsub,
    mdns: Mdns,
}

#[async_std::main]
async fn accept() -> Result<(), Box<dyn Error>> {
    // Generate a keypair for local identity
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Set up the transport layer
    let transport = libp2p::development_transport(id_keys).await?;

    // Create a Floodsub topic
    let topic = libp2p::floodsub::Topic::new("file-sharing");

    // Set up the swarm
    let behaviour = MyBehaviour {
        floodsub: libp2p::floodsub::Floodsub::new(peer_id),
        mdns: Mdns::new(MdnsConfig::default()).await?,
    };

    let mut swarm = SwarmBuilder::build(behaviour, peer_id, transport);

    // Start listening on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Main event loop
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(event)) => {
                // Handle mDNS events
                
            },
            SwarmEvent::Behaviour(MyBehaviourEvent::Floodsub(message)) => {
                // Handle incoming messages
                
            },
            _ => {}
        }
    }
}
