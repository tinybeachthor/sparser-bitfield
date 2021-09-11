use sparser_bitfield::Bitfield;

use tempfile::NamedTempFile;
use std::io::prelude::*;

#[test]
fn write_then_read_from_file() -> std::io::Result<()> {
    let mut bitfield = Bitfield::new();
    bitfield.set(2);

    let mut file = NamedTempFile::new()?;
    let mut file2 = file.reopen()?;

    let bytes = bitfield.to_bytes()?;
    file.write_all(bytes.as_ref())?;

    let bitfield2 = Bitfield::from_file(&mut file2, None)?;

    assert_eq!(
        bitfield.get(2),
        bitfield2.get(2));
    assert_eq!(
        bitfield.get(20),
        bitfield2.get(20));
    assert_eq!(
        bitfield.to_bytes()?,
        bitfield2.to_bytes()?);

    Ok(())
}

#[test]
fn write_then_read_from_file_large() -> std::io::Result<()> {
    let mut bitfield = Bitfield::new();
    bitfield.set(2);
    bitfield.set(10_000_102);

    let mut file = NamedTempFile::new()?;
    let mut file2 = file.reopen()?;

    let bytes = bitfield.to_bytes()?;
    file.write_all(bytes.as_ref())?;

    let bitfield2 = Bitfield::from_file(&mut file2, None)?;

    assert_eq!(
        bitfield.get(2),
        bitfield2.get(2));
    assert_eq!(
        bitfield.get(20),
        bitfield2.get(20));
    assert_eq!(
        bitfield.get(100_102),
        bitfield2.get(100_102));
    assert_eq!(
        bitfield.to_bytes()?,
        bitfield2.to_bytes()?);

    Ok(())
}
