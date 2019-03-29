#![feature(extern_types)]

mod prio;

use prio::client::{PrioPRGSeed_randomize};
use prio::config::{Prio_init, PrioConfig_new, PrioConfig_numDataFields, PrioConfig_clear, Prio_clear};
use prio::encrypt::{PrivateKey, PrivateKey_clear, PublicKey, PublicKey_clear};
use prio::server::{
    PrioServer, PrioServer_aggregate, PrioServer_clear, PrioServer_new, PrioVerifier_set_data,
    PrioPacketVerify1, PrioPacketVerify1_clear, PrioPacketVerify1_new, PrioPacketVerify1_set_data,
    PrioPacketVerify2, PrioPacketVerify2_clear, PrioPacketVerify2_new, PrioPacketVerify2_set_data,
    PrioTotalShare_final, PrioVerifier, PrioVerifier_clear, PrioVerifier_new, PrioVerifier_isValid,
    PrioTotalShare, PrioTotalShare_clear, PrioTotalShare_new, PrioTotalShare_set_data
};

fn main() {
    println!("Setting up...");
    let mut pkA: PublicKey = 0 as PublicKey;
    let mut pkB: PublicKey = 0 as PublicKey;
    let mut skA: PrivateKey = 0 as PrivateKey;
    let mut skB: PrivateKey = 0 as PrivateKey;

    let mut sA: PrioServer = 0 as PrioServer;
    let mut sB: PrioServer = 0 as PrioServer;
    let mut vA: PrioVerifier = 0 as PrioVerifier;
    let mut vB: PrioVerifier = 0 as PrioVerifier;
    let mut p1A: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut p1B: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut p2A: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut p2B: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut tA: PrioTotalShare = 0 as PrioTotalShare;
    let mut tB: PrioTotalShare = 0 as PrioTotalShare;

    unsafe {
        let mut rv = Prio_init();

        rv = prio::encrypt::Keypair_new(&mut skA, &mut pkA);
        println!("{:?}, {:?}, {:?}", rv, skA, pkA);

        rv = prio::encrypt::Keypair_new(&mut skB, &mut pkB);
        println!("{:?}, {:?}, {:?}", rv, skB, pkB);

        println!("Configuring...");
        let cfg = PrioConfig_new(214i32, pkA, pkB, b"testbatch\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_uchar, 9i32 as libc::c_uint);
        println!("{:?}, {:?}", rv, cfg);

        let mut serverSecret: prio::client::PrioPRGSeed = [4_u8; 16];
        rv = PrioPRGSeed_randomize(&mut serverSecret);
        println!("{:?}, {:?}", rv, serverSecret);

        // Initialize two server objects. The role of the servers need not
        // be symmetric. In a deployment, we envision that:
        //   * Server A is the main telemetry server that is always online.
        //     Clients send their encrypted data packets to Server A and
        //     Server A stores them.
        //   * Server B only comes online when the two servers want to compute
        //     the final aggregate statistics.
        println!("Initializing server objects...");
        sA = PrioServer_new(cfg, prio::client::PRIO_SERVER_A, skA, serverSecret.as_ptr());
        println!("{:?}", sA);
        sB = PrioServer_new(cfg, prio::client::PRIO_SERVER_B, skB, serverSecret.as_ptr());
        println!("{:?}", sA);

        // Initialize empty verifier objects
        println!("Initializing verifier objects...");
        vA = PrioVerifier_new(sA);
        println!("{:?}", vA);
        vB = PrioVerifier_new(sB);
        println!("{:?}", sB);

        // Initialize shares of final aggregate statistics
        println!("Initializing of final aggregate statistics...");
        tA = PrioTotalShare_new();
        tB = PrioTotalShare_new();

        // Initialize shares of verification packets
        println!("Initializing shares of verification packets...");
        p1A = PrioPacketVerify1_new();
        p1B = PrioPacketVerify1_new();
        p2A = PrioPacketVerify2_new();
        p2B = PrioPacketVerify2_new();

        // I. CLIENT DATA SUBMISSION.
        // Initialize empty client result objects
        println!("Client data submission...");

        let mut aLen = 0;
        let mut bLen = 0;

        let mut forServerA = 0u8 as *mut u8;
        let mut forServerB = 0u8 as *mut u8;
        let mut data_items = false;
        let mut output = 0u64 as *mut u64;

        let ndata: i32 = PrioConfig_numDataFields(cfg);
/*
        data_items = calloc(ndata, sizeof(bool));

        for (int i = 0; i < ndata; i++) {
            // Arbitrary data
            data_items[i] = (i % 3 == 1) || (i % 5 == 3);
        }
*/
        // Read in the client data packets
        println!("Read in client data packets...");

        let pA = prio::client::PrioPacketClient_new(cfg, prio::client::PRIO_SERVER_A);
        let pB = prio::client::PrioPacketClient_new(cfg, prio::client::PRIO_SERVER_B);

        rv = prio::client::PrioPacketClient_set_data(cfg, &data_items, pA, pB);
        println!("{:?}, {:?}, {:?}, {:?}", rv, data_items, pA, pB);

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
        println!("Set up a Prio verifier object...");

        rv = PrioVerifier_set_data(vA, forServerA, aLen);
        println!("{:?}, {:?}, {:?}, {:?}", rv, vA, forServerA, aLen);

        rv = PrioVerifier_set_data(vB, forServerB, bLen);
        println!("{:?}, {:?}, {:?}, {:?}", rv, vA, forServerA, aLen);

        // Both servers produce a packet1. Server A sends p1A to Server B
        // and vice versa.
        println!("Set data for verifying packet 1");
        rv = PrioPacketVerify1_set_data(p1A, vA);
        println!("{:?}, {:?}, {:?}", rv, p1A, vA);
        rv = PrioPacketVerify1_set_data(p1B, vB);
        println!("{:?}, {:?}, {:?}", rv, p1B, vB);

        // Both servers produce a packet2. Server A sends p2A to Server B
        // and vice versa.
        println!("Set data for verifying packet 2");
        rv = PrioPacketVerify2_set_data(p2A, vA, p1A, p1B);
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", rv, p2A, vA, p1A, p1B);
        rv = PrioPacketVerify2_set_data(p2B, vB, p1A, p1B);
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", rv, p2A, vA, p1A, p1B);

        // Using p2A and p2B, the servers can determine whether the request
        // is valid. (In fact, only Server A needs to perform this
        // check, since Server A can just tell Server B whether the check
        // succeeded or failed.)
        println!("Determine if request is valid...");
        rv = PrioVerifier_isValid(vA, p2A, p2B);
        println!("{:?}, {:?}, {:?}, {:?}", rv, vA, p2A, p2B);
        rv = PrioVerifier_isValid(vB, p2A, p2B);
        println!("{:?}, {:?}, {:?}, {:?}", rv, vB, p2A, p2B);

        // If we get here, the client packet is valid, so add it to the aggregate
        // statistic counter for both servers.
        println!("Determine if client packet is valid...");
        rv = PrioServer_aggregate(sA, vA);
        println!("{:?}, {:?}, {:?}", rv, sA, vA);
        rv = PrioServer_aggregate(sB, vB);
        println!("{:?}, {:?}, {:?}", rv, sB, vB);

        // The servers repeat the steps above for each client submission.

        // III. PRODUCTION OF AGGREGATE STATISTICS.
        //
        // After collecting aggregates from MANY clients, the servers can compute
        // their shares of the aggregate statistics.
        //
        // Server B can send tB to Server A.
        println!("Producte aggregate statistics...");

        println!("Set total share data...");
        rv = PrioTotalShare_set_data(tA, sA);
        println!("{:?}, {:?}, {:?}", rv, tA, sA);
        rv = PrioTotalShare_set_data(tB, sB);
        println!("{:?}, {:?}, {:?}", rv, tB, sB);

        // Once Server A has tA and tB, it can learn the aggregate statistics
        // in the clear.
        println!("Final aggregate statistics in the clear...");
        rv = PrioTotalShare_final(cfg, output, tA, tB);
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", rv, cfg, output, tA, tB);

/*
        for (int i = 0; i < ndata; i++) {
            ASSERT_TRUE(output[i] == dataItems[i]);
        }
*/

        println!("Cleaning up...");
        PrioTotalShare_clear(tA);
        PrioTotalShare_clear(tB);

        PrioPacketVerify2_clear(p2A);
        PrioPacketVerify2_clear(p2B);

        PrioPacketVerify1_clear(p1A);
        PrioPacketVerify1_clear(p1B);

        PrioVerifier_clear(vA);
        PrioVerifier_clear(vB);

        PrioServer_clear(sA);
        PrioServer_clear(sB);
        PrioConfig_clear(cfg);

        PublicKey_clear(pkA);
        PublicKey_clear(pkB);

        PrivateKey_clear(skA);
        PrivateKey_clear(skB);

        Prio_clear();
    }

    println!("Done.");
}
