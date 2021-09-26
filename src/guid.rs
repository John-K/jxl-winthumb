use windows::Guid;

pub const JXLWINTHUMB_THUMBNAILPROVIDER_CLSID: Guid =
    get_guid_from_u128(0xdf52deb1_9d07_4520_b606_97c6ecb069a2);

pub const JXLWINTHUMB_VENDOR_CLSID: Guid =
    get_guid_from_u128(0x448d5eb7_6555_476b_a840_034cca9afe6e);

pub const fn get_guid_from_u128(uuid: u128) -> Guid {
    const fn get_byte(uuid: u128, index: u8) -> u8 {
        (uuid >> (8 * index) & 0xff) as u8
    }

    Guid::from_values(
        (uuid >> (8 * 12)) as u32,
        (uuid >> (8 * 10) & 0xffff) as u16,
        (uuid >> (8 * 8) & 0xffff) as u16,
        [
            get_byte(uuid, 7),
            get_byte(uuid, 6),
            get_byte(uuid, 5),
            get_byte(uuid, 4),
            get_byte(uuid, 3),
            get_byte(uuid, 2),
            get_byte(uuid, 1),
            get_byte(uuid, 0),
        ],
    )
}

pub fn guid_to_string(guid: &Guid) -> String {
    format!("{{{:?}}}", guid)
}
