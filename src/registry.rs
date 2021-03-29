use std::io;
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE, REG_BINARY},
    RegKey, RegValue,
};

pub const PROD_SERVERS: [&str; 26] = [
    "USWest",
    "USWest2",
    "USWest3",
    "USWest4",
    "USSouth",
    "USSouth2",
    "USSouth3",
    "USSouthWest",
    "USNorthWest",
    "USMidWest",
    "USMidWest2",
    "USEast",
    "USEast2",
    "USEast3",
    "USEast4",
    "EUWest",
    "EUWest2",
    "EUSouthWest",
    "EUSouth",
    "EUNorth",
    "EUNorth2",
    "EUEast",
    "EUEast2",
    "Australia",
    "AsiaSouthEast",
    "AsiaEast",
];

pub const TESTING_SERVERS: [&str; 3] = ["USEastT1", "EUWestT1", "AsiaSouthEastT1"];

const REGISTRY_PATH: &'static str = "SOFTWARE\\DECA Live Operations GmbH\\RotMGExalt";

pub fn get_server_value() -> io::Result<(String, RegValue)> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let exalt = hkcu.open_subkey(REGISTRY_PATH)?;

    for name in exalt.enum_values() {
        let (key, value) = name?;
        if key.starts_with("preferredServer") {
            return Ok((key, value));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No preferredServer registry entry found",
    ))
}

pub fn set_server_value(name: &str, value: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let exalt = hkcu.open_subkey_with_flags(REGISTRY_PATH, KEY_WRITE)?;

    let null_terminated = value.to_owned() + "\u{0}";
    let data = RegValue {
        vtype: REG_BINARY,
        bytes: null_terminated.into_bytes(),
    };
    exalt.set_raw_value(name, &data)?;

    Ok(())
}
