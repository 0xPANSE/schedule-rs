mod cluster;

use async_raft::raft::Raft;
use async_raft::storage::RaftStorage;
use async_raft::RaftNetwork;
use async_raft::State;
use async_raft::{Config, ConfigBuilder, NodeId};

#[tokio::main]
async fn main() {
    // // Create a network instance.
    // let network = Network::new();
    //
    // // Create a configuration for the Raft node.
    // let config = Config::build("schedule-rs-cluster".into())
    //     .election_timeout_min(100)
    //     .election_timeout_max(300)
    //     .heartbeat_interval(50)
    //     .validate()
    //     .expect("failed to build Raft config");
    //
    // // Create the Raft storage.
    // let storage = MemStorage::new_with_config(1);
    //
    // // Create a new Raft node.
    // let mut raft = Raft::new(config, storage, Box::new(network.clone())).unwrap();
    //
    // // Start the Raft node.
    // raft.start().await.unwrap();
    //
    // // Main loop to simulate cluster activities.
    // loop {
    //     // Process Raft events and transitions.
    //     raft.poll().await.unwrap();
    //
    //     // Check the current state of the Raft node.
    //     match raft.state() {
    //         State::Leader => {
    //             // Perform leader-specific tasks.
    //             // For example, handle client requests and replicate logs.
    //             // You will implement your scheduling logic here.
    //         }
    //         State::Follower => {
    //             // Perform follower-specific tasks.
    //             // For example, handle log replication from the leader.
    //         }
    //         State::Candidate => {
    //             // Perform candidate-specific tasks.
    //             // For example, participate in leader election.
    //         }
    //     }
    //
    //     // Simulate some time passing before the next iteration.
    //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    // }
}
