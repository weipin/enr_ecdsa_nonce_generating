use enr::Enr;
use hex_literal::hex;
use std::net::Ipv4Addr;

type DefaultEnr = Enr<k256::ecdsa::SigningKey>;

fn main() {
    let data = [
        // the example from the spec
        // [
        //     "0x7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c",
        //     "0x01",
        //     "0x6964",
        //     "0x7634",
        //     "0x6970",
        //     "0x7f000001",
        //     "0x736563703235366b31",
        //     "0x03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138",
        //     "0x756470",
        //     "0x765f"
        // ]
        "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",

        // Replaces seq 0x01 with 0x0001
        // [
        //     "0x7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c",
        //     "0x0001",
        //     "0x6964",
        //     "0x7634",
        //     "0x6970",
        //     "0x7f000001",
        //     "0x736563703235366b31",
        //     "0x03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138",
        //     "0x756470",
        //     "0x765f"
        // ]
        "enr:-Ia4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5yCAAGCaWSCdjSCaXCEfwAAAYlzZWNwMjU2azGhA8pjTK4NSay0Adikxrb-jFW3DRFb9AB2nMFADzJYzTE4g3VkcIJ2Xw",

        // Replaces seq 0x01 with 0x000001
        // [
        //     "0x7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c",
        //     "0x000001",
        //     "0x6964",
        //     "0x7634",
        //     "0x6970",
        //     "0x7f000001",
        //     "0x736563703235366b31",
        //     "0x03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138",
        //     "0x756470",
        //     "0x765f"
        // ]
        "enr:-Ie4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5yDAAABgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
    ];

    let ip = Ipv4Addr::from(hex!("7f000001"));
    let udp = u16::from_be_bytes(hex!("765f"));

    for text in data {
        let enr = text.parse::<DefaultEnr>().unwrap();
        assert_eq!(enr.seq(), 1);
        assert_eq!(enr.ip4().unwrap(), ip);
        assert_eq!(enr.udp4().unwrap(), udp);
    }
}