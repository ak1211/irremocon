#[cfg(test)]
mod tests {
    use irremocon::lib::InfraredCodes;

    #[test]
    fn hexdump() {
        let x = "840042000E0010000E0010000C0032000E0032000C0012000C0032000E0010000E0010000C0012000E0030000E0010000E0010000E0030000E0010000E0032000C0010000E0032000C0012000A0010000E0032000C0012000C00100010000E000E0010000C0034000C0032000E0032000E0030000E0010000E0030000E0010000E0010000E0010000E0030000E0032000C0012000C0010000E0032000E0010000E000E000E008E02";
        let y = InfraredCodes::from_hexdump(x).unwrap().to_hexdump();
        assert_eq!(x, y);
    }
}
