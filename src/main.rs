#![feature(extern_types)]

mod prio;

use prio::config;
use prio::encrypt;

fn main() {
    let mut pkA: prio::encrypt::PublicKey = 0 as prio::encrypt::PublicKey;
    let mut pkB: prio::encrypt::PublicKey = 0 as prio::encrypt::PublicKey;
    let mut skA: prio::encrypt::PrivateKey = 0 as prio::encrypt::PrivateKey;
    let mut skB: prio::encrypt::PrivateKey = 0 as prio::encrypt::PrivateKey;

    unsafe {
        prio::encrypt::Keypair_new(&mut skA, &mut pkA);;
        prio::encrypt::Keypair_new(&mut skB, &mut pkB);

        let cfg = prio::config::PrioConfig_new(0, pkA, pkB, &0, 0);
    }
}
