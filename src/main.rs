//! Demonstrates if additional (random) data is used when the crate::enr creates a
//! deterministic ECDSA signature.

use enr::EnrBuilder;
use hex_literal::hex;
use std::net::Ipv4Addr;

fn main() {
    // Here we use the example record from the ENR spec:
    // https://github.com/ethereum/devp2p/blob/master/enr.md
    let key_data = hex!("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291");
    let ip = Ipv4Addr::from(hex!("7f000001"));
    let udp = u16::from_be_bytes(hex!("765f"));

    // The feature "k256" uses additional data -- each run gives a different result.
    let key = k256::ecdsa::SigningKey::from_bytes(&key_data).unwrap();
    let enr = EnrBuilder::new("v4").ip4(ip).udp4(udp).build(&key).unwrap();
    println!("# k256\n{}\n", enr.to_base64());

    // The feature "secp256k1" does not use additional data -- each run gives a fixed result,
    // the exact one documented in the ENR spec.
    // enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8
    let key = secp256k1::SecretKey::from_slice(&key_data).unwrap();
    let enr = EnrBuilder::new("v4").ip4(ip).udp4(udp).build(&key).unwrap();
    println!("# secp256k1\n{}", enr.to_base64());
}
