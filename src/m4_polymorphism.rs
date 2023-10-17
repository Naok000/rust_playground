#[cfg(test)]
mod test {
    use ethers::types::Address;
    use std::str::FromStr;

    trait EthereumAddress {
        fn convert_address(&self) -> Result<Address, &'static str>;
    }

    impl EthereumAddress for &str {
        fn convert_address(&self) -> Result<Address, &'static str> {
            match Address::from_str(self) {
                Ok(address) => Ok(address),
                Err(_) => Err("Invalid Ethereum Address String"),
            }
        }
    }

    impl EthereumAddress for Address {
        fn convert_address(&self) -> Result<Address, &'static str> {
            Ok(*self)
        }
    }

    fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
        let converted_address: Address = address.convert_address().unwrap();

        // do something else

        converted_address
    }

    #[test]
    fn tests_poly() {
        // for_str : Creates a hash type instance from the given string
        let addr: Address =
            Address::from_str("0x4De5025AFa40A7b13ef23b6e3a6Bb99b8b580159").unwrap();

        let new_addr: Address = get_ethereum_data(addr);

        assert_eq!(
            new_addr,
            Address::from_str("0x4De5025AFa40A7b13ef23b6e3a6Bb99b8b580159").unwrap()
        );

        let new_addr: Address = get_ethereum_data("0x4De5025AFa40A7b13ef23b6e3a6Bb99b8b580159");
        assert_eq!(
            new_addr,
            Address::from_str("0x4De5025AFa40A7b13ef23b6e3a6Bb99b8b580159").unwrap()
        );
    }
}
