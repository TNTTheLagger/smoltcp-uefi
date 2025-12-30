use uefi::proto::network::EfiMacAddr;

/// Convert a [`uefi::proto::network::EfiMacAddr`] into a [`smoltcp::wire::EthernetAddress`].
pub fn u2s_mac_address(a: &EfiMacAddr) -> smoltcp::wire::EthernetAddress {
    let mut out = smoltcp::wire::EthernetAddress([0; 6]);
    out.0.copy_from_slice(&a.0[..6]); // only take first 6 bytes
    out
}

