#![feature(extern_types)]

mod prio;

use prio::client;
use prio::config;
use prio::encrypt;
use prio::server;

fn main() {
    println!("Setting up...");
    let mut pkA: prio::encrypt::PublicKey = 0 as prio::encrypt::PublicKey;
    let mut pkB: prio::encrypt::PublicKey = 0 as prio::encrypt::PublicKey;
    let mut skA: prio::encrypt::PrivateKey = 0 as prio::encrypt::PrivateKey;
    let mut skB: prio::encrypt::PrivateKey = 0 as prio::encrypt::PrivateKey;

    let mut sA: prio::server::PrioServer = 0 as prio::server::PrioServer;
    let mut sB: prio::server::PrioServer = 0 as prio::server::PrioServer;
    let mut vA: prio::server::PrioVerifier = 0 as prio::server::PrioVerifier;
    let mut vB: prio::server::PrioVerifier = 0 as prio::server::PrioVerifier;
    let mut p1A: prio::server::PrioPacketVerify1 = 0 as prio::server::PrioPacketVerify1;
    let mut p1B: prio::server::PrioPacketVerify1 = 0 as prio::server::PrioPacketVerify1;
    let mut p2A: prio::server::PrioPacketVerify2 = 0 as prio::server::PrioPacketVerify2;
    let mut p2B: prio::server::PrioPacketVerify2 = 0 as prio::server::PrioPacketVerify2;
    let mut tA: prio::server::PrioTotalShare = 0 as prio::server::PrioTotalShare;
    let mut tB: prio::server::PrioTotalShare = 0 as prio::server::PrioTotalShare;

    unsafe {
        prio::encrypt::Keypair_new(&mut skA, &mut pkA);;
        prio::encrypt::Keypair_new(&mut skB, &mut pkB);

        println!("Configuring...");
        let cfg = prio::config::PrioConfig_new(0, pkA, pkB, &0, 0);

        // let mut serverSecret: prio::client::PrioPRGSeed = [4_u8; 16];
        // prio::client::PrioPRGSeed_randomize(&mut serverSecret);
        let serverSecret: *const u8 = 0 as *const u8;

        // Initialize two server objects. The role of the servers need not
        // be symmetric. In a deployment, we envision that:
        //   * Server A is the main telemetry server that is always online.
        //     Clients send their encrypted data packets to Server A and
        //     Server A stores them.
        //   * Server B only comes online when the two servers want to compute
        //     the final aggregate statistics.
        println!("Initializing server objects...");
        sA = prio::server::PrioServer_new(cfg, prio::client::PRIO_SERVER_A, skA, serverSecret);
        sB = prio::server::PrioServer_new(cfg, prio::client::PRIO_SERVER_B, skB, serverSecret);

        // Initialize empty verifier objects
        println!("Initializing verifier objects...");
        vA = prio::server::PrioVerifier_new(sA);
        vB = prio::server::PrioVerifier_new(sB);

        // Initialize shares of final aggregate statistics
        println!("Initializing of final aggregate statistics...");
        tA = prio::server::PrioTotalShare_new();
        tB = prio::server::PrioTotalShare_new();

        // Initialize shares of verification packets
        println!("Initializing shares of verification packets...");
        p1A = prio::server::PrioPacketVerify1_new();
        p1B = prio::server::PrioPacketVerify1_new();
        p2A = prio::server::PrioPacketVerify2_new();
        p2B = prio::server::PrioPacketVerify2_new();

        // I. CLIENT DATA SUBMISSION.
        // Initialize empty client result objects
        println!("Client data submission...");

        let mut aLen = 0;
        let mut bLen = 0;

        let mut forServerA = 0u8 as *mut u8;
        let mut forServerB = 0u8 as *mut u8;
        let mut data_items = false;
        let mut output = 0u64 as *mut u64;

        let ndata: i32 = prio::config::PrioConfig_numDataFields(cfg);
/*
        data_items = calloc(ndata, sizeof(bool));

        for (int i = 0; i < ndata; i++) {
            // Arbitrary data
            data_items[i] = (i % 3 == 1) || (i % 5 == 3);
        }
*/
        // Read in the client data packets
        let pA = prio::client::PrioPacketClient_new(cfg, prio::client::PRIO_SERVER_A);
        let pB = prio::client::PrioPacketClient_new(cfg, prio::client::PRIO_SERVER_B);

        prio::client::PrioPacketClient_set_data(cfg, &data_items, pA, pB);

        // II. VALIDATION PROTOCOL. (at servers)
        //
        // The servers now run a short 2-step protocol to check each
        // client's packet:
        //    1) Servers A and B broadcast one message (PrioPacketVerify1)
        //       to each other.
        //    2) Servers A and B broadcast another message (PrioPacketVerify2)
        //       to each other.
        //    3) Servers A and B can both determine whether the client's data
        //       submission is well-formed (in which case they add it to their
        //       running total of aggregate statistics) or ill-formed
        //       (in which case they ignore it).
        // These messages must be sent over an authenticated channel, so
        // that each server is assured that every received message came
        // from its peer.

        // Set up a Prio verifier object.
        prio::server::PrioVerifier_set_data(vA, forServerA, aLen);
        prio::server::PrioVerifier_set_data(vB, forServerB, bLen);

        // Both servers produce a packet1. Server A sends p1A to Server B
        // and vice versa.
        prio::server::PrioPacketVerify1_set_data(p1A, vA);
        prio::server::PrioPacketVerify1_set_data(p1B, vB);

        // Both servers produce a packet2. Server A sends p2A to Server B
        // and vice versa.
        prio::server::PrioPacketVerify2_set_data(p2A, vA, p1A, p1B);
        prio::server::PrioPacketVerify2_set_data(p2B, vB, p1A, p1B);

        // Using p2A and p2B, the servers can determine whether the request
        // is valid. (In fact, only Server A needs to perform this
        // check, since Server A can just tell Server B whether the check
        // succeeded or failed.)
        prio::server::PrioVerifier_isValid(vA, p2A, p2B);
        prio::server::PrioVerifier_isValid(vB, p2A, p2B);

        // If we get here, the client packet is valid, so add it to the aggregate
        // statistic counter for both servers.
        prio::server::PrioServer_aggregate(sA, vA);
        prio::server::PrioServer_aggregate(sB, vB);

        // The servers repeat the steps above for each client submission.

        // III. PRODUCTION OF AGGREGATE STATISTICS.
        //
        // After collecting aggregates from MANY clients, the servers can compute
        // their shares of the aggregate statistics.
        //
        // Server B can send tB to Server A.
        prio::server::PrioTotalShare_set_data(tA, sA);
        prio::server::PrioTotalShare_set_data(tB, sB);

        // Once Server A has tA and tB, it can learn the aggregate statistics
        // in the clear.
        prio::server::PrioTotalShare_final(cfg, output, tA, tB);

/*
        for (int i = 0; i < ndata; i++) {
            ASSERT_TRUE(output[i] == dataItems[i]);
        }
*/
        prio::server::PrioTotalShare_clear(tA);
        prio::server::PrioTotalShare_clear(tB);

        prio::server::PrioPacketVerify2_clear(p2A);
        prio::server::PrioPacketVerify2_clear(p2B);

        prio::server::PrioPacketVerify1_clear(p1A);
        prio::server::PrioPacketVerify1_clear(p1B);

        prio::server::PrioVerifier_clear(vA);
        prio::server::PrioVerifier_clear(vB);

        prio::server::PrioServer_clear(sA);
        prio::server::PrioServer_clear(sB);
        prio::config::PrioConfig_clear(cfg);

        prio::encrypt::PublicKey_clear(pkA);
        prio::encrypt::PublicKey_clear(pkB);

        prio::encrypt::PrivateKey_clear(skA);
        prio::encrypt::PrivateKey_clear(skB);

        prio::config::Prio_clear();
    }

    println!("Done.");
}
