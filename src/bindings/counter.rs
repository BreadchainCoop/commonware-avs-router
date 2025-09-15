///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct G1Point { uint256 X; uint256 Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct G2Point { uint256[2] X; uint256[2] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G2Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                    .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                    .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.X
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.Y
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.X, out
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.Y, out
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureCheckerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerPubkeys:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub quorumApks:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            BN254::G2Point,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonSignerStakesAndSignature> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerStakesAndSignature) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.nonSignerPubkeys,
                    value.quorumApks,
                    value.apkG2,
                    value.sigma,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerStakesAndSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    nonSignerPubkeys: tuple.1,
                    quorumApks: tuple.2,
                    apkG2: tuple.3,
                    sigma: tuple.4,
                    quorumApkIndices: tuple.5,
                    totalStakeIndices: tuple.6,
                    nonSignerStakeIndices: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonSignerStakesAndSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for NonSignerStakesAndSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerPubkeys),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApks),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerPubkeys,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumApks)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apkG2,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigma,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonSignerStakesAndSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerPubkeys,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApks,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkG2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigma,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerPubkeys,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApks,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkG2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigma,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        #[allow(missing_docs)]
        pub signedStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        #[allow(missing_docs)]
        pub totalStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumStakeTotals> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumStakeTotals) -> Self {
                (value.signedStakeForQuorum, value.totalStakeForQuorum)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumStakeTotals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signedStakeForQuorum: tuple.0,
                    totalStakeForQuorum: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumStakeTotals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumStakeTotals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedStakeForQuorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeForQuorum),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QuorumStakeTotals {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedStakeForQuorum,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeForQuorum,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumStakeTotals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedStakeForQuorum,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeForQuorum,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedStakeForQuorum,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeForQuorum,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

    See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
        IBLSSignatureCheckerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureCheckerTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IBLSSignatureCheckerTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
            IBLSSignatureCheckerTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature {
        uint32[] nonSignerQuorumBitmapIndices;
        BN254.G1Point[] nonSignerPubkeys;
        BN254.G1Point[] quorumApks;
        BN254.G2Point apkG2;
        BN254.G1Point sigma;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct QuorumStakeTotals {
        uint96[] signedStakeForQuorum;
        uint96[] totalStakeForQuorum;
    }
}

interface Counter {
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error ECAddFailed();
    error ECMulFailed();
    error ExpModFailed();
    error FutureBlockNumber();
    error InputArrayLengthMismatch();
    error InputEmptyQuorumNumbers();
    error InputNonSignerLengthMismatch();
    error InsufficientQuorumThreshold();
    error InvalidBLSPairingKey();
    error InvalidBLSSignature();
    error InvalidHash();
    error InvalidQuorumApkHash();
    error InvalidReferenceBlocknumber();
    error NonSignerPubkeysNotSorted();
    error OnlyRegistryCoordinatorOwner();
    error ScalarTooLarge();
    error StaleBlockNumber();

    constructor(address _registryCoordinator);

    function BLOCK_STALE_MEASURE() external view returns (uint32);
    function QUORUM_THRESHOLD() external view returns (uint256);
    function THRESHOLD_DENOMINATOR() external view returns (uint256);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    function delegation() external view returns (address);
    function increment(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external;
    function number() external view returns (uint256);
    function registryCoordinator() external view returns (address);
    function stakeRegistry() external view returns (address);
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "BLOCK_STALE_MEASURE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "QUORUM_THRESHOLD",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "THRESHOLD_DENOMINATOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.QuorumStakeTotals",
        "components": [
          {
            "name": "signedStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          },
          {
            "name": "totalStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "increment",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "number",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "trySignatureAndApkVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "siganatureIsValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECAddFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECMulFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FutureBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputEmptyQuorumNumbers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputNonSignerLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientQuorumThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSPairingKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorumApkHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlocknumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonSignerPubkeysNotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ScalarTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StaleBlockNumber",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Counter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61010060405261012c60335f6101000a81548163ffffffff021916908363ffffffff1602179055506042603455606460355534801561003c575f5ffd5b506040516136af3803806136af833981810160405281019061005e91906102ed565b80808073ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff1663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100dd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101019190610353565b73ffffffffffffffffffffffffffffffffffffffff1660a08173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff16635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa15801561017d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101a191906103b9565b73ffffffffffffffffffffffffffffffffffffffff1660c08173ffffffffffffffffffffffffffffffffffffffff168152505060a05173ffffffffffffffffffffffffffffffffffffffff1663df5cf7236040518163ffffffff1660e01b8152600401602060405180830381865afa15801561021f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610243919061041f565b73ffffffffffffffffffffffffffffffffffffffff1660e08173ffffffffffffffffffffffffffffffffffffffff168152505050505061044a565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6102ab82610282565b9050919050565b5f6102bc826102a1565b9050919050565b6102cc816102b2565b81146102d6575f5ffd5b50565b5f815190506102e7816102c3565b92915050565b5f602082840312156103025761030161027e565b5b5f61030f848285016102d9565b91505092915050565b5f610322826102a1565b9050919050565b61033281610318565b811461033c575f5ffd5b50565b5f8151905061034d81610329565b92915050565b5f602082840312156103685761036761027e565b5b5f6103758482850161033f565b91505092915050565b5f610388826102a1565b9050919050565b6103988161037e565b81146103a2575f5ffd5b50565b5f815190506103b38161038f565b92915050565b5f602082840312156103ce576103cd61027e565b5b5f6103db848285016103a5565b91505092915050565b5f6103ee826102a1565b9050919050565b6103fe816103e4565b8114610408575f5ffd5b50565b5f81519050610419816103f5565b92915050565b5f602082840312156104345761043361027e565b5b5f6104418482850161040b565b91505092915050565b60805160a05160c05160e05161320f6104a05f395f61122d01525f818161037e0152610a3301525f81816103bd01528181610bb30152610d8d01525f81816103e10152818161070d015261088a015261320f5ff3fe608060405234801561000f575f5ffd5b50600436106100a7575f3560e01c80636d14a9871161006f5780636d14a987146101545780636efb4636146101725780638381f58a146101a357806385047a49146101c1578063df5cf723146101dd578063ef024458146101fb576100a7565b8063171f1d5b146100ab5780635df45946146100dc5780635e510b60146100fa5780635e8b3f2d146101185780636830483514610136575b5f5ffd5b6100c560048036038101906100c091906121d7565b610219565b6040516100d3929190612256565b60405180910390f35b6100e461037c565b6040516100f191906122f7565b60405180910390f35b6101026103a0565b60405161010f919061231f565b60405180910390f35b6101206103a6565b60405161012d9190612356565b60405180910390f35b61013e6103bb565b60405161014b919061238f565b60405180910390f35b61015c6103df565b60405161016991906123c8565b60405180910390f35b61018c60048036038101906101879190612834565b610403565b60405161019a9291906129f2565b60405180910390f35b6101ab610fdf565b6040516101b8919061231f565b60405180910390f35b6101db60048036038101906101d69190612834565b610fe5565b005b6101e561122b565b6040516101f29190612a40565b60405180910390f35b61020361124f565b604051610210919061231f565b60405180910390f35b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f6002811061025d5761025c612a59565b5b6020020151895f015160016002811061027957610278612a59565b5b60200201518a602001515f6002811061029557610294612a59565b5b60200201518b602001516001600281106102b2576102b1612a59565b5b60200201518b5f01518c602001516040516020016102d899989796959493929190612ac6565b604051602081830303815290604052805190602001205f1c6102fa9190612b95565b905061036a610324610315838961125590919063ffffffff16565b8661132090919063ffffffff16565b61032c611410565b6103606103498561033b6114da565b61125590919063ffffffff16565b6103528c6114fe565b61132090919063ffffffff16565b886201d4c0611609565b80935081945050505094509492505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b60345481565b60335f9054906101000a900463ffffffff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b61040b611e41565b5f5f8686905003610448576040517f1f0405a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8260400151518686905014801561046657508260a001515186869050145b801561047957508260c001515186869050145b801561048c57508260e001515186869050145b6104c2576040517f43714afd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f01515183602001515114610504576040517f5f832f4100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4363ffffffff168463ffffffff1610610549576040517f4b874f4500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f60405180604001604052805f81526020015f8152509050610569611e41565b8787905067ffffffffffffffff81111561058657610585611fd8565b5b6040519080825280602002602001820160405280156105b45781602001602082028036833780820191505090505b5081602001819052508787905067ffffffffffffffff8111156105da576105d9611fd8565b5b6040519080825280602002602001820160405280156106085781602001602082028036833780820191505090505b50815f0181905250610618611e5b565b85602001515167ffffffffffffffff81111561063757610636611fd8565b5b6040519080825280602002602001820160405280156106655781602001602082028036833780820191505090505b50815f018190525085602001515167ffffffffffffffff81111561068c5761068b611fd8565b5b6040519080825280602002602001820160405280156106ba5781602001602082028036833780820191505090505b5081602001819052505f61079d8a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610774573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107989190612bfb565b6118a9565b90505f5f90505b876020015151811015610a14576107d8886020015182815181106107cb576107ca612a59565b5b6020026020010151611900565b836020015182815181106107ef576107ee612a59565b5b6020026020010181815250505f81146108885782602001516001826108149190612c53565b8151811061082557610824612a59565b5b60200260200101515f1c8360200151828151811061084657610845612a59565b5b60200260200101515f1c11610887576040517fff71941400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166304ec6351846020015183815181106108db576108da612a59565b5b60200260200101518b8b5f015185815181106108fa576108f9612a59565b5b60200260200101516040518463ffffffff1660e01b815260040161092093929190612cb6565b602060405180830381865afa15801561093b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061095f9190612d38565b77ffffffffffffffffffffffffffffffffffffffffffffffff16835f0151828151811061098f5761098e612a59565b5b602002602001018181525050610a056109f66109c984865f015185815181106109bb576109ba612a59565b5b602002602001015116611918565b8a6020015184815181106109e0576109df612a59565b5b602002602001015161195390919063ffffffff16565b8661132090919063ffffffff16565b945080806001019150506107a4565b5050610a1f83611a38565b92505f5f90505b89899050811015610f10577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166368bccaac8b8b84818110610a8057610a7f612a59565b5b9050013560f81c60f81b60f81c8a8a60a001518581518110610aa557610aa4612a59565b5b60200260200101516040518463ffffffff1660e01b8152600401610acb93929190612d72565b602060405180830381865afa158015610ae6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b0a9190612dfc565b67ffffffffffffffff1916610b3c88604001518381518110610b2f57610b2e612a59565b5b6020026020010151611900565b67ffffffffffffffff191614610b7e576040517fe1310aed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610baf87604001518281518110610b9857610b97612a59565b5b60200260200101518561132090919063ffffffff16565b93507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663c8294c568b8b84818110610c0057610bff612a59565b5b9050013560f81c60f81b60f81c8a8a60c001518581518110610c2557610c24612a59565b5b60200260200101516040518463ffffffff1660e01b8152600401610c4b93929190612d72565b602060405180830381865afa158015610c66573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c8a9190612e51565b83602001518281518110610ca157610ca0612a59565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff168152505082602001518181518110610ce257610ce1612a59565b5b6020026020010151835f01518281518110610d0057610cff612a59565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250505f5f90505f5f90505b886020015151811015610f0157610d86845f01518281518110610d5957610d58612a59565b5b60200260200101518d8d86818110610d7457610d73612a59565b5b9050013560f81c60f81b60f81c611af0565b15610ef4577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f2be94ae8d8d86818110610dda57610dd9612a59565b5b9050013560f81c60f81b60f81c8c87602001518581518110610dff57610dfe612a59565b5b60200260200101518d60e001518881518110610e1e57610e1d612a59565b5b60200260200101518781518110610e3857610e37612a59565b5b60200260200101516040518563ffffffff1660e01b8152600401610e5f9493929190612e7c565b602060405180830381865afa158015610e7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9e9190612e51565b855f01518481518110610eb457610eb3612a59565b5b60200260200101818151610ec89190612ebf565b9150906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250508160010191505b8080600101915050610d33565b50508080600101915050610a26565b505f5f610f278c868a606001518b60800151610219565b9150915081610f62576040517f67988d3300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80610f99576040517fab1b236b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505f878260200151604051602001610fb3929190612fe3565b604051602081830303815290604052805190602001209050828195509550505050509550959350505050565b60325481565b438263ffffffff1610611024576040517f252f8a0e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4363ffffffff1660335f9054906101000a900463ffffffff1683611048919061300a565b63ffffffff161015611086576040517f305c3e9300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f600260325460405160200161109c919061231f565b6040516020818303038152906040526040516110b89190613093565b602060405180830381855afa1580156110d3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906110f691906130bd565b9050808614611131576040517f0af806e000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61113f8787878787610403565b5090505f5f90505b8686905081101561120a576034548260200151828151811061116c5761116b612a59565b5b60200260200101516bffffffffffffffffffffffff1661118c91906130e8565b603554835f015183815181106111a5576111a4612a59565b5b60200260200101516bffffffffffffffffffffffff166111c591906130e8565b10156111fd576040517f6d8605db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8080600101915050611147565b5060325f81548092919061121d90613129565b919050555050505050505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b60355481565b61125d611e75565b611265611e8d565b835f0151815f6003811061127c5761127b612a59565b5b60200201818152505083602001518160016003811061129e5761129d612a59565b5b60200201818152505082816002600381106112bc576112bb612a59565b5b6020020181815250505f60408360608460076107d05a03fa9050805f81036112e057fe5b5080611318576040517f4633be3200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b611328611e75565b611330611eaf565b835f0151815f6004811061134757611346612a59565b5b60200201818152505083602001518160016004811061136957611368612a59565b5b602002018181525050825f01518160026004811061138a57611389612a59565b5b6020020181815250508260200151816003600481106113ac576113ab612a59565b5b6020020181815250505f60408360808460066107d05a03fa9050805f81036113d057fe5b5080611408576040517fd4b68fd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b611418611ed1565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b6114e2611e75565b6040518060400160405280600181526020016002815250905090565b611506611e75565b5f5f90505f5f90505f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47855f1c61153d9190612b95565b90505b6001156115e95761155081611b06565b80935081945050507f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061158757611586612b68565b5b82830983036115af576040518060400160405280828152602001838152509350505050611604565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47806115de576115dd612b68565b5b600182089050611540565b60405180604001604052805f81526020015f81525093505050505b919050565b5f5f5f60405180604001604052808981526020018781525090505f6040518060400160405280898152602001878152509050611643611ef7565b5f5f90505b6002811015611861575f60068261165f91906130e8565b905084826002811061167457611673612a59565b5b60200201515f0151835f836116899190613170565b600c811061169a57611699612a59565b5b6020020181815250508482600281106116b6576116b5612a59565b5b602002015160200151836001836116cd9190613170565b600c81106116de576116dd612a59565b5b6020020181815250508382600281106116fa576116f9612a59565b5b60200201515f01515f6002811061171457611713612a59565b5b6020020151836002836117279190613170565b600c811061173857611737612a59565b5b60200201818152505083826002811061175457611753612a59565b5b60200201515f015160016002811061176f5761176e612a59565b5b6020020151836003836117829190613170565b600c811061179357611792612a59565b5b6020020181815250508382600281106117af576117ae612a59565b5b6020020151602001515f600281106117ca576117c9612a59565b5b6020020151836004836117dd9190613170565b600c81106117ee576117ed612a59565b5b60200201818152505083826002811061180a57611809612a59565b5b60200201516020015160016002811061182657611825612a59565b5b6020020151836005836118399190613170565b600c811061184a57611849612a59565b5b602002018181525050508080600101915050611648565b5061186a611f1a565b5f6020826020600c028560088cfa9050805f835f6001811061188f5761188e612a59565b5b602002015114159650965050505050509550959350505050565b5f5f6118b484611bfb565b9050808360ff166001901b116118f6576040517fca95733300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8091505092915050565b5f81515f52816020015160205260405f209050919050565b5f5f5f90505b5f83111561194a576001836119339190612c53565b831692508080611942906131b0565b91505061191e565b80915050919050565b61195b611e75565b6102008261ffff161061199a576040517fff89d4fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018261ffff16036119ae57829050611a32565b5f60405180604001604052805f81526020015f81525090505f8490505f600190505f5f90505b8161ffff168661ffff1610611a2a576001808260ff168861ffff16901c1661ffff1603611a0857611a058484611320565b93505b611a128384611320565b925060018261ffff16901b91508060010190506119d4565b839450505050505b92915050565b611a40611e75565b5f825f0151148015611a5557505f8260200151145b15611a765760405180604001604052805f81526020015f8152509050611aeb565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611aba9190612b95565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611ae59190612c53565b81525090505b919050565b5f60018260ff1684901c16600114905092915050565b5f5f5f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b3857611b37612b68565b5b60037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b6957611b68612b68565b5b867f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b9957611b98612b68565b5b888909090890505f611bec827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f527f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611d03565b90508181935093505050915091565b5f61010082511115611c39576040517ffb4a9c8e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f825103611c49575f9050611cfe565b5f5f835f81518110611c5e57611c5d612a59565b5b602001015160f81c60f81b60f81c60ff166001901b91505f600190505b8451811015611cf757848181518110611c9757611c96612a59565b5b602001015160f81c60f81b60f81c60ff166001901b9150828211611ce7576040517f80c8834800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183179250806001019050611c7b565b5081925050505b919050565b5f5f611d0d611f1a565b611d15611f3c565b6020815f60068110611d2a57611d29612a59565b5b602002018181525050602081600160068110611d4957611d48612a59565b5b602002018181525050602081600260068110611d6857611d67612a59565b5b6020020181815250508681600360068110611d8657611d85612a59565b5b6020020181815250508581600460068110611da457611da3612a59565b5b6020020181815250508481600560068110611dc257611dc1612a59565b5b60200201818152505060208260c08360056107d05a03fa9250825f8103611de557fe5b5082611e1d576040517fd51edae300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815f60018110611e3057611e2f612a59565b5b602002015193505050509392505050565b604051806040016040528060608152602001606081525090565b604051806040016040528060608152602001606081525090565b60405180604001604052805f81526020015f81525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611ee4611f5e565b8152602001611ef1611f5e565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060c00160405280600690602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b611fa381611f91565b8114611fad575f5ffd5b50565b5f81359050611fbe81611f9a565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61200e82611fc8565b810181811067ffffffffffffffff8211171561202d5761202c611fd8565b5b80604052505050565b5f61203f611f80565b905061204b8282612005565b919050565b5f5ffd5b5f819050919050565b61206681612054565b8114612070575f5ffd5b50565b5f813590506120818161205d565b92915050565b5f6040828403121561209c5761209b611fc4565b5b6120a66040612036565b90505f6120b584828501612073565b5f8301525060206120c884828501612073565b60208301525092915050565b5f5ffd5b5f67ffffffffffffffff8211156120f2576120f1611fd8565b5b602082029050919050565b5f5ffd5b5f61211361210e846120d8565b612036565b9050806020840283018581111561212d5761212c6120fd565b5b835b8181101561215657806121428882612073565b84526020840193505060208101905061212f565b5050509392505050565b5f82601f830112612174576121736120d4565b5b6002612181848285612101565b91505092915050565b5f6080828403121561219f5761219e611fc4565b5b6121a96040612036565b90505f6121b884828501612160565b5f8301525060406121cb84828501612160565b60208301525092915050565b5f5f5f5f61012085870312156121f0576121ef611f89565b5b5f6121fd87828801611fb0565b945050602061220e87828801612087565b935050606061221f8782880161218a565b92505060e061223087828801612087565b91505092959194509250565b5f8115159050919050565b6122508161223c565b82525050565b5f6040820190506122695f830185612247565b6122766020830184612247565b9392505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f6122bf6122ba6122b58461227d565b61229c565b61227d565b9050919050565b5f6122d0826122a5565b9050919050565b5f6122e1826122c6565b9050919050565b6122f1816122d7565b82525050565b5f60208201905061230a5f8301846122e8565b92915050565b61231981612054565b82525050565b5f6020820190506123325f830184612310565b92915050565b5f63ffffffff82169050919050565b61235081612338565b82525050565b5f6020820190506123695f830184612347565b92915050565b5f612379826122c6565b9050919050565b6123898161236f565b82525050565b5f6020820190506123a25f830184612380565b92915050565b5f6123b2826122c6565b9050919050565b6123c2816123a8565b82525050565b5f6020820190506123db5f8301846123b9565b92915050565b5f5ffd5b5f5f83601f8401126123fa576123f96120d4565b5b8235905067ffffffffffffffff811115612417576124166123e1565b5b602083019150836001820283011115612433576124326120fd565b5b9250929050565b61244381612338565b811461244d575f5ffd5b50565b5f8135905061245e8161243a565b92915050565b5f67ffffffffffffffff82111561247e5761247d611fd8565b5b602082029050602081019050919050565b5f6124a161249c84612464565b612036565b905080838252602082019050602084028301858111156124c4576124c36120fd565b5b835b818110156124ed57806124d98882612450565b8452602084019350506020810190506124c6565b5050509392505050565b5f82601f83011261250b5761250a6120d4565b5b813561251b84826020860161248f565b91505092915050565b5f67ffffffffffffffff82111561253e5761253d611fd8565b5b602082029050602081019050919050565b5f61256161255c84612524565b612036565b90508083825260208201905060408402830185811115612584576125836120fd565b5b835b818110156125ad57806125998882612087565b845260208401935050604081019050612586565b5050509392505050565b5f82601f8301126125cb576125ca6120d4565b5b81356125db84826020860161254f565b91505092915050565b5f67ffffffffffffffff8211156125fe576125fd611fd8565b5b602082029050602081019050919050565b5f61262161261c846125e4565b612036565b90508083825260208201905060208402830185811115612644576126436120fd565b5b835b8181101561268b57803567ffffffffffffffff811115612669576126686120d4565b5b80860161267689826124f7565b85526020850194505050602081019050612646565b5050509392505050565b5f82601f8301126126a9576126a86120d4565b5b81356126b984826020860161260f565b91505092915050565b5f61018082840312156126d8576126d7611fc4565b5b6126e3610100612036565b90505f82013567ffffffffffffffff81111561270257612701612050565b5b61270e848285016124f7565b5f83015250602082013567ffffffffffffffff81111561273157612730612050565b5b61273d848285016125b7565b602083015250604082013567ffffffffffffffff81111561276157612760612050565b5b61276d848285016125b7565b60408301525060606127818482850161218a565b60608301525060e061279584828501612087565b60808301525061012082013567ffffffffffffffff8111156127ba576127b9612050565b5b6127c6848285016124f7565b60a08301525061014082013567ffffffffffffffff8111156127eb576127ea612050565b5b6127f7848285016124f7565b60c08301525061016082013567ffffffffffffffff81111561281c5761281b612050565b5b61282884828501612695565b60e08301525092915050565b5f5f5f5f5f6080868803121561284d5761284c611f89565b5b5f61285a88828901611fb0565b955050602086013567ffffffffffffffff81111561287b5761287a611f8d565b5b612887888289016123e5565b9450945050604061289a88828901612450565b925050606086013567ffffffffffffffff8111156128bb576128ba611f8d565b5b6128c7888289016126c2565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6bffffffffffffffffffffffff82169050919050565b61291d816128fd565b82525050565b5f61292e8383612914565b60208301905092915050565b5f602082019050919050565b5f612950826128d4565b61295a81856128de565b9350612965836128ee565b805f5b8381101561299557815161297c8882612923565b97506129878361293a565b925050600181019050612968565b5085935050505092915050565b5f604083015f8301518482035f8601526129bc8282612946565b915050602083015184820360208601526129d68282612946565b9150508091505092915050565b6129ec81611f91565b82525050565b5f6040820190508181035f830152612a0a81856129a2565b9050612a1960208301846129e3565b9392505050565b5f612a2a826122c6565b9050919050565b612a3a81612a20565b82525050565b5f602082019050612a535f830184612a31565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b612aa0612a9b82611f91565b612a86565b82525050565b5f819050919050565b612ac0612abb82612054565b612aa6565b82525050565b5f612ad1828c612a8f565b602082019150612ae1828b612aaf565b602082019150612af1828a612aaf565b602082019150612b018289612aaf565b602082019150612b118288612aaf565b602082019150612b218287612aaf565b602082019150612b318286612aaf565b602082019150612b418285612aaf565b602082019150612b518284612aaf565b6020820191508190509a9950505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612b9f82612054565b9150612baa83612054565b925082612bba57612bb9612b68565b5b828206905092915050565b5f60ff82169050919050565b612bda81612bc5565b8114612be4575f5ffd5b50565b5f81519050612bf581612bd1565b92915050565b5f60208284031215612c1057612c0f611f89565b5b5f612c1d84828501612be7565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612c5d82612054565b9150612c6883612054565b9250828203905081811115612c8057612c7f612c26565b5b92915050565b5f612ca0612c9b612c9684612338565b61229c565b612054565b9050919050565b612cb081612c86565b82525050565b5f606082019050612cc95f8301866129e3565b612cd66020830185612347565b612ce36040830184612ca7565b949350505050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b612d1781612ceb565b8114612d21575f5ffd5b50565b5f81519050612d3281612d0e565b92915050565b5f60208284031215612d4d57612d4c611f89565b5b5f612d5a84828501612d24565b91505092915050565b612d6c81612bc5565b82525050565b5f606082019050612d855f830186612d63565b612d926020830185612347565b612d9f6040830184612ca7565b949350505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b612ddb81612da7565b8114612de5575f5ffd5b50565b5f81519050612df681612dd2565b92915050565b5f60208284031215612e1157612e10611f89565b5b5f612e1e84828501612de8565b91505092915050565b612e30816128fd565b8114612e3a575f5ffd5b50565b5f81519050612e4b81612e27565b92915050565b5f60208284031215612e6657612e65611f89565b5b5f612e7384828501612e3d565b91505092915050565b5f608082019050612e8f5f830187612d63565b612e9c6020830186612347565b612ea960408301856129e3565b612eb66060830184612ca7565b95945050505050565b5f612ec9826128fd565b9150612ed4836128fd565b925082820390506bffffffffffffffffffffffff811115612ef857612ef7612c26565b5b92915050565b5f8160e01b9050919050565b5f612f1482612efe565b9050919050565b612f2c612f2782612338565b612f0a565b82525050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b612f5e81611f91565b82525050565b5f612f6f8383612f55565b60208301905092915050565b5f602082019050919050565b5f612f9182612f32565b612f9b8185612f3c565b9350612fa683612f46565b805f5b83811015612fd6578151612fbd8882612f64565b9750612fc883612f7b565b925050600181019050612fa9565b5085935050505092915050565b5f612fee8285612f1b565b600482019150612ffe8284612f87565b91508190509392505050565b5f61301482612338565b915061301f83612338565b9250828201905063ffffffff81111561303b5761303a612c26565b5b92915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f61306d82613041565b613077818561304b565b9350613087818560208601613055565b80840191505092915050565b5f61309e8284613063565b915081905092915050565b5f815190506130b781611f9a565b92915050565b5f602082840312156130d2576130d1611f89565b5b5f6130df848285016130a9565b91505092915050565b5f6130f282612054565b91506130fd83612054565b925082820261310b81612054565b9150828204841483151761312257613121612c26565b5b5092915050565b5f61313382612054565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361316557613164612c26565b5b600182019050919050565b5f61317a82612054565b915061318583612054565b925082820190508082111561319d5761319c612c26565b5b92915050565b5f61ffff82169050919050565b5f6131ba826131a3565b915061ffff82036131ce576131cd612c26565b5b60018201905091905056fea26469706673582212207d80a96ec44efa5c51139b37f0ba1382535f81e3e0a5c2216b2847c0a8c6c1f864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@Ra\x01,`3_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`B`4U`d`5U4\x80\x15a\0<W__\xFD[P`@Qa6\xAF8\x03\x80a6\xAF\x839\x81\x81\x01`@R\x81\x01\x90a\0^\x91\x90a\x02\xEDV[\x80\x80\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x01\x91\x90a\x03SV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xA1\x91\x90a\x03\xB9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\xA0Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02C\x91\x90a\x04\x1FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPa\x04JV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x02\xAB\x82a\x02\x82V[\x90P\x91\x90PV[_a\x02\xBC\x82a\x02\xA1V[\x90P\x91\x90PV[a\x02\xCC\x81a\x02\xB2V[\x81\x14a\x02\xD6W__\xFD[PV[_\x81Q\x90Pa\x02\xE7\x81a\x02\xC3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03\x02Wa\x03\x01a\x02~V[[_a\x03\x0F\x84\x82\x85\x01a\x02\xD9V[\x91PP\x92\x91PPV[_a\x03\"\x82a\x02\xA1V[\x90P\x91\x90PV[a\x032\x81a\x03\x18V[\x81\x14a\x03<W__\xFD[PV[_\x81Q\x90Pa\x03M\x81a\x03)V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03hWa\x03ga\x02~V[[_a\x03u\x84\x82\x85\x01a\x03?V[\x91PP\x92\x91PPV[_a\x03\x88\x82a\x02\xA1V[\x90P\x91\x90PV[a\x03\x98\x81a\x03~V[\x81\x14a\x03\xA2W__\xFD[PV[_\x81Q\x90Pa\x03\xB3\x81a\x03\x8FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03\xCEWa\x03\xCDa\x02~V[[_a\x03\xDB\x84\x82\x85\x01a\x03\xA5V[\x91PP\x92\x91PPV[_a\x03\xEE\x82a\x02\xA1V[\x90P\x91\x90PV[a\x03\xFE\x81a\x03\xE4V[\x81\x14a\x04\x08W__\xFD[PV[_\x81Q\x90Pa\x04\x19\x81a\x03\xF5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x044Wa\x043a\x02~V[[_a\x04A\x84\x82\x85\x01a\x04\x0BV[\x91PP\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa2\x0Fa\x04\xA0_9_a\x12-\x01R_\x81\x81a\x03~\x01Ra\n3\x01R_\x81\x81a\x03\xBD\x01R\x81\x81a\x0B\xB3\x01Ra\r\x8D\x01R_\x81\x81a\x03\xE1\x01R\x81\x81a\x07\r\x01Ra\x08\x8A\x01Ra2\x0F_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA7W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0oW\x80cm\x14\xA9\x87\x14a\x01TW\x80cn\xFBF6\x14a\x01rW\x80c\x83\x81\xF5\x8A\x14a\x01\xA3W\x80c\x85\x04zI\x14a\x01\xC1W\x80c\xDF\\\xF7#\x14a\x01\xDDW\x80c\xEF\x02DX\x14a\x01\xFBWa\0\xA7V[\x80c\x17\x1F\x1D[\x14a\0\xABW\x80c]\xF4YF\x14a\0\xDCW\x80c^Q\x0B`\x14a\0\xFAW\x80c^\x8B?-\x14a\x01\x18W\x80ch0H5\x14a\x016W[__\xFD[a\0\xC5`\x04\x806\x03\x81\x01\x90a\0\xC0\x91\x90a!\xD7V[a\x02\x19V[`@Qa\0\xD3\x92\x91\x90a\"VV[`@Q\x80\x91\x03\x90\xF3[a\0\xE4a\x03|V[`@Qa\0\xF1\x91\x90a\"\xF7V[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x03\xA0V[`@Qa\x01\x0F\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x03\xA6V[`@Qa\x01-\x91\x90a#VV[`@Q\x80\x91\x03\x90\xF3[a\x01>a\x03\xBBV[`@Qa\x01K\x91\x90a#\x8FV[`@Q\x80\x91\x03\x90\xF3[a\x01\\a\x03\xDFV[`@Qa\x01i\x91\x90a#\xC8V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8C`\x04\x806\x03\x81\x01\x90a\x01\x87\x91\x90a(4V[a\x04\x03V[`@Qa\x01\x9A\x92\x91\x90a)\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x0F\xDFV[`@Qa\x01\xB8\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a(4V[a\x0F\xE5V[\0[a\x01\xE5a\x12+V[`@Qa\x01\xF2\x91\x90a*@V[`@Q\x80\x91\x03\x90\xF3[a\x02\x03a\x12OV[`@Qa\x02\x10\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x02]Wa\x02\\a*YV[[` \x02\x01Q\x89_\x01Q`\x01`\x02\x81\x10a\x02yWa\x02xa*YV[[` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02\x95Wa\x02\x94a*YV[[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02\xB2Wa\x02\xB1a*YV[[` \x02\x01Q\x8B_\x01Q\x8C` \x01Q`@Q` \x01a\x02\xD8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a*\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\xFA\x91\x90a+\x95V[\x90Pa\x03ja\x03$a\x03\x15\x83\x89a\x12U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03,a\x14\x10V[a\x03`a\x03I\x85a\x03;a\x14\xDAV[a\x12U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03R\x8Ca\x14\xFEV[a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88b\x01\xD4\xC0a\x16\tV[\x80\x93P\x81\x94PPPP\x94P\x94\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`4T\x81V[`3_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x0Ba\x1EAV[__\x86\x86\x90P\x03a\x04HW`@Q\x7F\x1F\x04\x05\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x86\x86\x90P\x14\x80\x15a\x04fWP\x82`\xA0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04yWP\x82`\xC0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04\x8CWP\x82`\xE0\x01QQ\x86\x86\x90P\x14[a\x04\xC2W`@Q\x7FCqJ\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_\x01QQ\x83` \x01QQ\x14a\x05\x04W`@Q\x7F_\x83/A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x05IW`@Q\x7FK\x87OE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x05ia\x1EAV[\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x86Wa\x05\x85a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xB4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xDAWa\x05\xD9a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x08W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RPa\x06\x18a\x1E[V[\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x067Wa\x066a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06eW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RP\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x8CWa\x06\x8Ba\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP_a\x07\x9D\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x98\x91\x90a+\xFBV[a\x18\xA9V[\x90P__\x90P[\x87` \x01QQ\x81\x10\x15a\n\x14Wa\x07\xD8\x88` \x01Q\x82\x81Q\x81\x10a\x07\xCBWa\x07\xCAa*YV[[` \x02` \x01\x01Qa\x19\0V[\x83` \x01Q\x82\x81Q\x81\x10a\x07\xEFWa\x07\xEEa*YV[[` \x02` \x01\x01\x81\x81RPP_\x81\x14a\x08\x88W\x82` \x01Q`\x01\x82a\x08\x14\x91\x90a,SV[\x81Q\x81\x10a\x08%Wa\x08$a*YV[[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\x08FWa\x08Ea*YV[[` \x02` \x01\x01Q_\x1C\x11a\x08\x87W`@Q\x7F\xFFq\x94\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\x08\xDBWa\x08\xDAa*YV[[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\x08\xFAWa\x08\xF9a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t \x93\x92\x91\x90a,\xB6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t_\x91\x90a-8V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01Q\x82\x81Q\x81\x10a\t\x8FWa\t\x8Ea*YV[[` \x02` \x01\x01\x81\x81RPPa\n\x05a\t\xF6a\t\xC9\x84\x86_\x01Q\x85\x81Q\x81\x10a\t\xBBWa\t\xBAa*YV[[` \x02` \x01\x01Q\x16a\x19\x18V[\x8A` \x01Q\x84\x81Q\x81\x10a\t\xE0Wa\t\xDFa*YV[[` \x02` \x01\x01Qa\x19S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80\x80`\x01\x01\x91PPa\x07\xA4V[PPa\n\x1F\x83a\x1A8V[\x92P__\x90P[\x89\x89\x90P\x81\x10\x15a\x0F\x10W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xBC\xCA\xAC\x8B\x8B\x84\x81\x81\x10a\n\x80Wa\n\x7Fa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8A\x8A`\xA0\x01Q\x85\x81Q\x81\x10a\n\xA5Wa\n\xA4a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xCB\x93\x92\x91\x90a-rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\n\x91\x90a-\xFCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16a\x0B<\x88`@\x01Q\x83\x81Q\x81\x10a\x0B/Wa\x0B.a*YV[[` \x02` \x01\x01Qa\x19\0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0B~W`@Q\x7F\xE11\n\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xAF\x87`@\x01Q\x82\x81Q\x81\x10a\x0B\x98Wa\x0B\x97a*YV[[` \x02` \x01\x01Q\x85a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC8)LV\x8B\x8B\x84\x81\x81\x10a\x0C\0Wa\x0B\xFFa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8A\x8A`\xC0\x01Q\x85\x81Q\x81\x10a\x0C%Wa\x0C$a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CK\x93\x92\x91\x90a-rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8A\x91\x90a.QV[\x83` \x01Q\x82\x81Q\x81\x10a\x0C\xA1Wa\x0C\xA0a*YV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82` \x01Q\x81\x81Q\x81\x10a\x0C\xE2Wa\x0C\xE1a*YV[[` \x02` \x01\x01Q\x83_\x01Q\x82\x81Q\x81\x10a\r\0Wa\x0C\xFFa*YV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x90P__\x90P[\x88` \x01QQ\x81\x10\x15a\x0F\x01Wa\r\x86\x84_\x01Q\x82\x81Q\x81\x10a\rYWa\rXa*YV[[` \x02` \x01\x01Q\x8D\x8D\x86\x81\x81\x10a\rtWa\rsa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1Ca\x1A\xF0V[\x15a\x0E\xF4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xBE\x94\xAE\x8D\x8D\x86\x81\x81\x10a\r\xDAWa\r\xD9a*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x87` \x01Q\x85\x81Q\x81\x10a\r\xFFWa\r\xFEa*YV[[` \x02` \x01\x01Q\x8D`\xE0\x01Q\x88\x81Q\x81\x10a\x0E\x1EWa\x0E\x1Da*YV[[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x0E8Wa\x0E7a*YV[[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E_\x94\x93\x92\x91\x90a.|V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9E\x91\x90a.QV[\x85_\x01Q\x84\x81Q\x81\x10a\x0E\xB4Wa\x0E\xB3a*YV[[` \x02` \x01\x01\x81\x81Qa\x0E\xC8\x91\x90a.\xBFV[\x91P\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81`\x01\x01\x91P[\x80\x80`\x01\x01\x91PPa\r3V[PP\x80\x80`\x01\x01\x91PPa\n&V[P__a\x0F'\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x02\x19V[\x91P\x91P\x81a\x0FbW`@Q\x7Fg\x98\x8D3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x0F\x99W`@Q\x7F\xAB\x1B#k\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_\x87\x82` \x01Q`@Q` \x01a\x0F\xB3\x92\x91\x90a/\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82\x81\x95P\x95PPPPP\x95P\x95\x93PPPPV[`2T\x81V[C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x10$W`@Q\x7F%/\x8A\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16`3_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x83a\x10H\x91\x90a0\nV[c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10\x86W`@Q\x7F0\\>\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x02`2T`@Q` \x01a\x10\x9C\x91\x90a#\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Qa\x10\xB8\x91\x90a0\x93V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x10\xD3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF6\x91\x90a0\xBDV[\x90P\x80\x86\x14a\x111W`@Q\x7F\n\xF8\x06\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11?\x87\x87\x87\x87\x87a\x04\x03V[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\x12\nW`4T\x82` \x01Q\x82\x81Q\x81\x10a\x11lWa\x11ka*YV[[` \x02` \x01\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x8C\x91\x90a0\xE8V[`5T\x83_\x01Q\x83\x81Q\x81\x10a\x11\xA5Wa\x11\xA4a*YV[[` \x02` \x01\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\xC5\x91\x90a0\xE8V[\x10\x15a\x11\xFDW`@Q\x7Fm\x86\x05\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x11GV[P`2_\x81T\x80\x92\x91\x90a\x12\x1D\x90a1)V[\x91\x90PUPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`5T\x81V[a\x12]a\x1EuV[a\x12ea\x1E\x8DV[\x83_\x01Q\x81_`\x03\x81\x10a\x12|Wa\x12{a*YV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x12\x9EWa\x12\x9Da*YV[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x12\xBCWa\x12\xBBa*YV[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x12\xE0W\xFE[P\x80a\x13\x18W`@Q\x7FF3\xBE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x13(a\x1EuV[a\x130a\x1E\xAFV[\x83_\x01Q\x81_`\x04\x81\x10a\x13GWa\x13Fa*YV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x13iWa\x13ha*YV[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x13\x8AWa\x13\x89a*YV[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x13\xACWa\x13\xABa*YV[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x13\xD0W\xFE[P\x80a\x14\x08W`@Q\x7F\xD4\xB6\x8F\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x14\x18a\x1E\xD1V[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x14\xE2a\x1EuV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[a\x15\x06a\x1EuV[__\x90P__\x90P_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x85_\x1Ca\x15=\x91\x90a+\x95V[\x90P[`\x01\x15a\x15\xE9Wa\x15P\x81a\x1B\x06V[\x80\x93P\x81\x94PPP\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\x87Wa\x15\x86a+hV[[\x82\x83\t\x83\x03a\x15\xAFW`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP\x93PPPPa\x16\x04V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\xDEWa\x15\xDDa+hV[[`\x01\x82\x08\x90Pa\x15@V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x93PPPP[\x91\x90PV[___`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90Pa\x16Ca\x1E\xF7V[__\x90P[`\x02\x81\x10\x15a\x18aW_`\x06\x82a\x16_\x91\x90a0\xE8V[\x90P\x84\x82`\x02\x81\x10a\x16tWa\x16sa*YV[[` \x02\x01Q_\x01Q\x83_\x83a\x16\x89\x91\x90a1pV[`\x0C\x81\x10a\x16\x9AWa\x16\x99a*YV[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x16\xB6Wa\x16\xB5a*YV[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x16\xCD\x91\x90a1pV[`\x0C\x81\x10a\x16\xDEWa\x16\xDDa*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x16\xFAWa\x16\xF9a*YV[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x17\x14Wa\x17\x13a*YV[[` \x02\x01Q\x83`\x02\x83a\x17'\x91\x90a1pV[`\x0C\x81\x10a\x178Wa\x177a*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17TWa\x17Sa*YV[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x17oWa\x17na*YV[[` \x02\x01Q\x83`\x03\x83a\x17\x82\x91\x90a1pV[`\x0C\x81\x10a\x17\x93Wa\x17\x92a*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\xAFWa\x17\xAEa*YV[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17\xCAWa\x17\xC9a*YV[[` \x02\x01Q\x83`\x04\x83a\x17\xDD\x91\x90a1pV[`\x0C\x81\x10a\x17\xEEWa\x17\xEDa*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x18\nWa\x18\ta*YV[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x18&Wa\x18%a*YV[[` \x02\x01Q\x83`\x05\x83a\x189\x91\x90a1pV[`\x0C\x81\x10a\x18JWa\x18Ia*YV[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x16HV[Pa\x18ja\x1F\x1AV[_` \x82` `\x0C\x02\x85`\x08\x8C\xFA\x90P\x80_\x83_`\x01\x81\x10a\x18\x8FWa\x18\x8Ea*YV[[` \x02\x01Q\x14\x15\x96P\x96PPPPPP\x95P\x95\x93PPPPV[__a\x18\xB4\x84a\x1B\xFBV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x18\xF6W`@Q\x7F\xCA\x95s3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[___\x90P[_\x83\x11\x15a\x19JW`\x01\x83a\x193\x91\x90a,SV[\x83\x16\x92P\x80\x80a\x19B\x90a1\xB0V[\x91PPa\x19\x1EV[\x80\x91PP\x91\x90PV[a\x19[a\x1EuV[a\x02\0\x82a\xFF\xFF\x16\x10a\x19\x9AW`@Q\x7F\xFF\x89\xD4\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82a\xFF\xFF\x16\x03a\x19\xAEW\x82\x90Pa\x1A2V[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90P_\x84\x90P_`\x01\x90P__\x90P[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A*W`\x01\x80\x82`\xFF\x16\x88a\xFF\xFF\x16\x90\x1C\x16a\xFF\xFF\x16\x03a\x1A\x08Wa\x1A\x05\x84\x84a\x13 V[\x93P[a\x1A\x12\x83\x84a\x13 V[\x92P`\x01\x82a\xFF\xFF\x16\x90\x1B\x91P\x80`\x01\x01\x90Pa\x19\xD4V[\x83\x94PPPPP[\x92\x91PPV[a\x1A@a\x1EuV[_\x82_\x01Q\x14\x80\x15a\x1AUWP_\x82` \x01Q\x14[\x15a\x1AvW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1A\xEBV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1A\xBA\x91\x90a+\x95V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1A\xE5\x91\x90a,SV[\x81RP\x90P[\x91\x90PV[_`\x01\x82`\xFF\x16\x84\x90\x1C\x16`\x01\x14\x90P\x92\x91PPV[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B8Wa\x1B7a+hV[[`\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1BiWa\x1Bha+hV[[\x86\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B\x99Wa\x1B\x98a+hV[[\x88\x89\t\t\x08\x90P_a\x1B\xEC\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D\x03V[\x90P\x81\x81\x93P\x93PPP\x91P\x91V[_a\x01\0\x82Q\x11\x15a\x1C9W`@Q\x7F\xFBJ\x9C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82Q\x03a\x1CIW_\x90Pa\x1C\xFEV[__\x83_\x81Q\x81\x10a\x1C^Wa\x1C]a*YV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P_`\x01\x90P[\x84Q\x81\x10\x15a\x1C\xF7W\x84\x81\x81Q\x81\x10a\x1C\x97Wa\x1C\x96a*YV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P\x82\x82\x11a\x1C\xE7W`@Q\x7F\x80\xC8\x83H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x17\x92P\x80`\x01\x01\x90Pa\x1C{V[P\x81\x92PPP[\x91\x90PV[__a\x1D\ra\x1F\x1AV[a\x1D\x15a\x1F<V[` \x81_`\x06\x81\x10a\x1D*Wa\x1D)a*YV[[` \x02\x01\x81\x81RPP` \x81`\x01`\x06\x81\x10a\x1DIWa\x1DHa*YV[[` \x02\x01\x81\x81RPP` \x81`\x02`\x06\x81\x10a\x1DhWa\x1Dga*YV[[` \x02\x01\x81\x81RPP\x86\x81`\x03`\x06\x81\x10a\x1D\x86Wa\x1D\x85a*YV[[` \x02\x01\x81\x81RPP\x85\x81`\x04`\x06\x81\x10a\x1D\xA4Wa\x1D\xA3a*YV[[` \x02\x01\x81\x81RPP\x84\x81`\x05`\x06\x81\x10a\x1D\xC2Wa\x1D\xC1a*YV[[` \x02\x01\x81\x81RPP` \x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82_\x81\x03a\x1D\xE5W\xFE[P\x82a\x1E\x1DW`@Q\x7F\xD5\x1E\xDA\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_`\x01\x81\x10a\x1E0Wa\x1E/a*YV[[` \x02\x01Q\x93PPPP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1E\xE4a\x1F^V[\x81R` \x01a\x1E\xF1a\x1F^V[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x1F\xA3\x81a\x1F\x91V[\x81\x14a\x1F\xADW__\xFD[PV[_\x815\x90Pa\x1F\xBE\x81a\x1F\x9AV[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a \x0E\x82a\x1F\xC8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a -Wa ,a\x1F\xD8V[[\x80`@RPPPV[_a ?a\x1F\x80V[\x90Pa K\x82\x82a \x05V[\x91\x90PV[__\xFD[_\x81\x90P\x91\x90PV[a f\x81a TV[\x81\x14a pW__\xFD[PV[_\x815\x90Pa \x81\x81a ]V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a \x9CWa \x9Ba\x1F\xC4V[[a \xA6`@a 6V[\x90P_a \xB5\x84\x82\x85\x01a sV[_\x83\x01RP` a \xC8\x84\x82\x85\x01a sV[` \x83\x01RP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a \xF2Wa \xF1a\x1F\xD8V[[` \x82\x02\x90P\x91\x90PV[__\xFD[_a!\x13a!\x0E\x84a \xD8V[a 6V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a!-Wa!,a \xFDV[[\x83[\x81\x81\x10\x15a!VW\x80a!B\x88\x82a sV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa!/V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a!tWa!sa \xD4V[[`\x02a!\x81\x84\x82\x85a!\x01V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a!\x9FWa!\x9Ea\x1F\xC4V[[a!\xA9`@a 6V[\x90P_a!\xB8\x84\x82\x85\x01a!`V[_\x83\x01RP`@a!\xCB\x84\x82\x85\x01a!`V[` \x83\x01RP\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a!\xF0Wa!\xEFa\x1F\x89V[[_a!\xFD\x87\x82\x88\x01a\x1F\xB0V[\x94PP` a\"\x0E\x87\x82\x88\x01a \x87V[\x93PP``a\"\x1F\x87\x82\x88\x01a!\x8AV[\x92PP`\xE0a\"0\x87\x82\x88\x01a \x87V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\"P\x81a\"<V[\x82RPPV[_`@\x82\x01\x90Pa\"i_\x83\x01\x85a\"GV[a\"v` \x83\x01\x84a\"GV[\x93\x92PPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\"\xBFa\"\xBAa\"\xB5\x84a\"}V[a\"\x9CV[a\"}V[\x90P\x91\x90PV[_a\"\xD0\x82a\"\xA5V[\x90P\x91\x90PV[_a\"\xE1\x82a\"\xC6V[\x90P\x91\x90PV[a\"\xF1\x81a\"\xD7V[\x82RPPV[_` \x82\x01\x90Pa#\n_\x83\x01\x84a\"\xE8V[\x92\x91PPV[a#\x19\x81a TV[\x82RPPV[_` \x82\x01\x90Pa#2_\x83\x01\x84a#\x10V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a#P\x81a#8V[\x82RPPV[_` \x82\x01\x90Pa#i_\x83\x01\x84a#GV[\x92\x91PPV[_a#y\x82a\"\xC6V[\x90P\x91\x90PV[a#\x89\x81a#oV[\x82RPPV[_` \x82\x01\x90Pa#\xA2_\x83\x01\x84a#\x80V[\x92\x91PPV[_a#\xB2\x82a\"\xC6V[\x90P\x91\x90PV[a#\xC2\x81a#\xA8V[\x82RPPV[_` \x82\x01\x90Pa#\xDB_\x83\x01\x84a#\xB9V[\x92\x91PPV[__\xFD[__\x83`\x1F\x84\x01\x12a#\xFAWa#\xF9a \xD4V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x17Wa$\x16a#\xE1V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a$3Wa$2a \xFDV[[\x92P\x92\x90PV[a$C\x81a#8V[\x81\x14a$MW__\xFD[PV[_\x815\x90Pa$^\x81a$:V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$~Wa$}a\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a$\xA1a$\x9C\x84a$dV[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a$\xC4Wa$\xC3a \xFDV[[\x83[\x81\x81\x10\x15a$\xEDW\x80a$\xD9\x88\x82a$PV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa$\xC6V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\x0BWa%\na \xD4V[[\x815a%\x1B\x84\x82` \x86\x01a$\x8FV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%>Wa%=a\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a%aa%\\\x84a%$V[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a%\x84Wa%\x83a \xFDV[[\x83[\x81\x81\x10\x15a%\xADW\x80a%\x99\x88\x82a \x87V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa%\x86V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\xCBWa%\xCAa \xD4V[[\x815a%\xDB\x84\x82` \x86\x01a%OV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xFEWa%\xFDa\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a&!a&\x1C\x84a%\xE4V[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&DWa&Ca \xFDV[[\x83[\x81\x81\x10\x15a&\x8BW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&iWa&ha \xD4V[[\x80\x86\x01a&v\x89\x82a$\xF7V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa&FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&\xA9Wa&\xA8a \xD4V[[\x815a&\xB9\x84\x82` \x86\x01a&\x0FV[\x91PP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a&\xD8Wa&\xD7a\x1F\xC4V[[a&\xE3a\x01\0a 6V[\x90P_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x02Wa'\x01a PV[[a'\x0E\x84\x82\x85\x01a$\xF7V[_\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'1Wa'0a PV[[a'=\x84\x82\x85\x01a%\xB7V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'aWa'`a PV[[a'm\x84\x82\x85\x01a%\xB7V[`@\x83\x01RP``a'\x81\x84\x82\x85\x01a!\x8AV[``\x83\x01RP`\xE0a'\x95\x84\x82\x85\x01a \x87V[`\x80\x83\x01RPa\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xBAWa'\xB9a PV[[a'\xC6\x84\x82\x85\x01a$\xF7V[`\xA0\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xEBWa'\xEAa PV[[a'\xF7\x84\x82\x85\x01a$\xF7V[`\xC0\x83\x01RPa\x01`\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1CWa(\x1Ba PV[[a((\x84\x82\x85\x01a&\x95V[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a(MWa(La\x1F\x89V[[_a(Z\x88\x82\x89\x01a\x1F\xB0V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a({Wa(za\x1F\x8DV[[a(\x87\x88\x82\x89\x01a#\xE5V[\x94P\x94PP`@a(\x9A\x88\x82\x89\x01a$PV[\x92PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xBBWa(\xBAa\x1F\x8DV[[a(\xC7\x88\x82\x89\x01a&\xC2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)\x1D\x81a(\xFDV[\x82RPPV[_a).\x83\x83a)\x14V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a)P\x82a(\xD4V[a)Z\x81\x85a(\xDEV[\x93Pa)e\x83a(\xEEV[\x80_[\x83\x81\x10\x15a)\x95W\x81Qa)|\x88\x82a)#V[\x97Pa)\x87\x83a):V[\x92PP`\x01\x81\x01\x90Pa)hV[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra)\xBC\x82\x82a)FV[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra)\xD6\x82\x82a)FV[\x91PP\x80\x91PP\x92\x91PPV[a)\xEC\x81a\x1F\x91V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\n\x81\x85a)\xA2V[\x90Pa*\x19` \x83\x01\x84a)\xE3V[\x93\x92PPPV[_a**\x82a\"\xC6V[\x90P\x91\x90PV[a*:\x81a* V[\x82RPPV[_` \x82\x01\x90Pa*S_\x83\x01\x84a*1V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a*\xA0a*\x9B\x82a\x1F\x91V[a*\x86V[\x82RPPV[_\x81\x90P\x91\x90PV[a*\xC0a*\xBB\x82a TV[a*\xA6V[\x82RPPV[_a*\xD1\x82\x8Ca*\x8FV[` \x82\x01\x91Pa*\xE1\x82\x8Ba*\xAFV[` \x82\x01\x91Pa*\xF1\x82\x8Aa*\xAFV[` \x82\x01\x91Pa+\x01\x82\x89a*\xAFV[` \x82\x01\x91Pa+\x11\x82\x88a*\xAFV[` \x82\x01\x91Pa+!\x82\x87a*\xAFV[` \x82\x01\x91Pa+1\x82\x86a*\xAFV[` \x82\x01\x91Pa+A\x82\x85a*\xAFV[` \x82\x01\x91Pa+Q\x82\x84a*\xAFV[` \x82\x01\x91P\x81\x90P\x9A\x99PPPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\x9F\x82a TV[\x91Pa+\xAA\x83a TV[\x92P\x82a+\xBAWa+\xB9a+hV[[\x82\x82\x06\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a+\xDA\x81a+\xC5V[\x81\x14a+\xE4W__\xFD[PV[_\x81Q\x90Pa+\xF5\x81a+\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,\x10Wa,\x0Fa\x1F\x89V[[_a,\x1D\x84\x82\x85\x01a+\xE7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a,]\x82a TV[\x91Pa,h\x83a TV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a,\x80Wa,\x7Fa,&V[[\x92\x91PPV[_a,\xA0a,\x9Ba,\x96\x84a#8V[a\"\x9CV[a TV[\x90P\x91\x90PV[a,\xB0\x81a,\x86V[\x82RPPV[_``\x82\x01\x90Pa,\xC9_\x83\x01\x86a)\xE3V[a,\xD6` \x83\x01\x85a#GV[a,\xE3`@\x83\x01\x84a,\xA7V[\x94\x93PPPPV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a-\x17\x81a,\xEBV[\x81\x14a-!W__\xFD[PV[_\x81Q\x90Pa-2\x81a-\x0EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a-MWa-La\x1F\x89V[[_a-Z\x84\x82\x85\x01a-$V[\x91PP\x92\x91PPV[a-l\x81a+\xC5V[\x82RPPV[_``\x82\x01\x90Pa-\x85_\x83\x01\x86a-cV[a-\x92` \x83\x01\x85a#GV[a-\x9F`@\x83\x01\x84a,\xA7V[\x94\x93PPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a-\xDB\x81a-\xA7V[\x81\x14a-\xE5W__\xFD[PV[_\x81Q\x90Pa-\xF6\x81a-\xD2V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\x11Wa.\x10a\x1F\x89V[[_a.\x1E\x84\x82\x85\x01a-\xE8V[\x91PP\x92\x91PPV[a.0\x81a(\xFDV[\x81\x14a.:W__\xFD[PV[_\x81Q\x90Pa.K\x81a.'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.fWa.ea\x1F\x89V[[_a.s\x84\x82\x85\x01a.=V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa.\x8F_\x83\x01\x87a-cV[a.\x9C` \x83\x01\x86a#GV[a.\xA9`@\x83\x01\x85a)\xE3V[a.\xB6``\x83\x01\x84a,\xA7V[\x95\x94PPPPPV[_a.\xC9\x82a(\xFDV[\x91Pa.\xD4\x83a(\xFDV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xF8Wa.\xF7a,&V[[\x92\x91PPV[_\x81`\xE0\x1B\x90P\x91\x90PV[_a/\x14\x82a.\xFEV[\x90P\x91\x90PV[a/,a/'\x82a#8V[a/\nV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a/^\x81a\x1F\x91V[\x82RPPV[_a/o\x83\x83a/UV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a/\x91\x82a/2V[a/\x9B\x81\x85a/<V[\x93Pa/\xA6\x83a/FV[\x80_[\x83\x81\x10\x15a/\xD6W\x81Qa/\xBD\x88\x82a/dV[\x97Pa/\xC8\x83a/{V[\x92PP`\x01\x81\x01\x90Pa/\xA9V[P\x85\x93PPPP\x92\x91PPV[_a/\xEE\x82\x85a/\x1BV[`\x04\x82\x01\x91Pa/\xFE\x82\x84a/\x87V[\x91P\x81\x90P\x93\x92PPPV[_a0\x14\x82a#8V[\x91Pa0\x1F\x83a#8V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a0;Wa0:a,&V[[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a0m\x82a0AV[a0w\x81\x85a0KV[\x93Pa0\x87\x81\x85` \x86\x01a0UV[\x80\x84\x01\x91PP\x92\x91PPV[_a0\x9E\x82\x84a0cV[\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90Pa0\xB7\x81a\x1F\x9AV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a0\xD2Wa0\xD1a\x1F\x89V[[_a0\xDF\x84\x82\x85\x01a0\xA9V[\x91PP\x92\x91PPV[_a0\xF2\x82a TV[\x91Pa0\xFD\x83a TV[\x92P\x82\x82\x02a1\x0B\x81a TV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a1\"Wa1!a,&V[[P\x92\x91PPV[_a13\x82a TV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a1eWa1da,&V[[`\x01\x82\x01\x90P\x91\x90PV[_a1z\x82a TV[\x91Pa1\x85\x83a TV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a1\x9DWa1\x9Ca,&V[[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_a1\xBA\x82a1\xA3V[\x91Pa\xFF\xFF\x82\x03a1\xCEWa1\xCDa,&V[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 }\x80\xA9n\xC4N\xFA\\Q\x13\x9B7\xF0\xBA\x13\x82S_\x81\xE3\xE0\xA5\xC2!k(G\xC0\xA8\xC6\xC1\xF8dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100a7575f3560e01c80636d14a9871161006f5780636d14a987146101545780636efb4636146101725780638381f58a146101a357806385047a49146101c1578063df5cf723146101dd578063ef024458146101fb576100a7565b8063171f1d5b146100ab5780635df45946146100dc5780635e510b60146100fa5780635e8b3f2d146101185780636830483514610136575b5f5ffd5b6100c560048036038101906100c091906121d7565b610219565b6040516100d3929190612256565b60405180910390f35b6100e461037c565b6040516100f191906122f7565b60405180910390f35b6101026103a0565b60405161010f919061231f565b60405180910390f35b6101206103a6565b60405161012d9190612356565b60405180910390f35b61013e6103bb565b60405161014b919061238f565b60405180910390f35b61015c6103df565b60405161016991906123c8565b60405180910390f35b61018c60048036038101906101879190612834565b610403565b60405161019a9291906129f2565b60405180910390f35b6101ab610fdf565b6040516101b8919061231f565b60405180910390f35b6101db60048036038101906101d69190612834565b610fe5565b005b6101e561122b565b6040516101f29190612a40565b60405180910390f35b61020361124f565b604051610210919061231f565b60405180910390f35b5f5f5f7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000187875f01518860200151885f01515f6002811061025d5761025c612a59565b5b6020020151895f015160016002811061027957610278612a59565b5b60200201518a602001515f6002811061029557610294612a59565b5b60200201518b602001516001600281106102b2576102b1612a59565b5b60200201518b5f01518c602001516040516020016102d899989796959493929190612ac6565b604051602081830303815290604052805190602001205f1c6102fa9190612b95565b905061036a610324610315838961125590919063ffffffff16565b8661132090919063ffffffff16565b61032c611410565b6103606103498561033b6114da565b61125590919063ffffffff16565b6103528c6114fe565b61132090919063ffffffff16565b886201d4c0611609565b80935081945050505094509492505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b60345481565b60335f9054906101000a900463ffffffff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b61040b611e41565b5f5f8686905003610448576040517f1f0405a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8260400151518686905014801561046657508260a001515186869050145b801561047957508260c001515186869050145b801561048c57508260e001515186869050145b6104c2576040517f43714afd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f01515183602001515114610504576040517f5f832f4100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4363ffffffff168463ffffffff1610610549576040517f4b874f4500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f60405180604001604052805f81526020015f8152509050610569611e41565b8787905067ffffffffffffffff81111561058657610585611fd8565b5b6040519080825280602002602001820160405280156105b45781602001602082028036833780820191505090505b5081602001819052508787905067ffffffffffffffff8111156105da576105d9611fd8565b5b6040519080825280602002602001820160405280156106085781602001602082028036833780820191505090505b50815f0181905250610618611e5b565b85602001515167ffffffffffffffff81111561063757610636611fd8565b5b6040519080825280602002602001820160405280156106655781602001602082028036833780820191505090505b50815f018190525085602001515167ffffffffffffffff81111561068c5761068b611fd8565b5b6040519080825280602002602001820160405280156106ba5781602001602082028036833780820191505090505b5081602001819052505f61079d8a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610774573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107989190612bfb565b6118a9565b90505f5f90505b876020015151811015610a14576107d8886020015182815181106107cb576107ca612a59565b5b6020026020010151611900565b836020015182815181106107ef576107ee612a59565b5b6020026020010181815250505f81146108885782602001516001826108149190612c53565b8151811061082557610824612a59565b5b60200260200101515f1c8360200151828151811061084657610845612a59565b5b60200260200101515f1c11610887576040517fff71941400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166304ec6351846020015183815181106108db576108da612a59565b5b60200260200101518b8b5f015185815181106108fa576108f9612a59565b5b60200260200101516040518463ffffffff1660e01b815260040161092093929190612cb6565b602060405180830381865afa15801561093b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061095f9190612d38565b77ffffffffffffffffffffffffffffffffffffffffffffffff16835f0151828151811061098f5761098e612a59565b5b602002602001018181525050610a056109f66109c984865f015185815181106109bb576109ba612a59565b5b602002602001015116611918565b8a6020015184815181106109e0576109df612a59565b5b602002602001015161195390919063ffffffff16565b8661132090919063ffffffff16565b945080806001019150506107a4565b5050610a1f83611a38565b92505f5f90505b89899050811015610f10577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166368bccaac8b8b84818110610a8057610a7f612a59565b5b9050013560f81c60f81b60f81c8a8a60a001518581518110610aa557610aa4612a59565b5b60200260200101516040518463ffffffff1660e01b8152600401610acb93929190612d72565b602060405180830381865afa158015610ae6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b0a9190612dfc565b67ffffffffffffffff1916610b3c88604001518381518110610b2f57610b2e612a59565b5b6020026020010151611900565b67ffffffffffffffff191614610b7e576040517fe1310aed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610baf87604001518281518110610b9857610b97612a59565b5b60200260200101518561132090919063ffffffff16565b93507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663c8294c568b8b84818110610c0057610bff612a59565b5b9050013560f81c60f81b60f81c8a8a60c001518581518110610c2557610c24612a59565b5b60200260200101516040518463ffffffff1660e01b8152600401610c4b93929190612d72565b602060405180830381865afa158015610c66573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c8a9190612e51565b83602001518281518110610ca157610ca0612a59565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff168152505082602001518181518110610ce257610ce1612a59565b5b6020026020010151835f01518281518110610d0057610cff612a59565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250505f5f90505f5f90505b886020015151811015610f0157610d86845f01518281518110610d5957610d58612a59565b5b60200260200101518d8d86818110610d7457610d73612a59565b5b9050013560f81c60f81b60f81c611af0565b15610ef4577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f2be94ae8d8d86818110610dda57610dd9612a59565b5b9050013560f81c60f81b60f81c8c87602001518581518110610dff57610dfe612a59565b5b60200260200101518d60e001518881518110610e1e57610e1d612a59565b5b60200260200101518781518110610e3857610e37612a59565b5b60200260200101516040518563ffffffff1660e01b8152600401610e5f9493929190612e7c565b602060405180830381865afa158015610e7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9e9190612e51565b855f01518481518110610eb457610eb3612a59565b5b60200260200101818151610ec89190612ebf565b9150906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff16815250508160010191505b8080600101915050610d33565b50508080600101915050610a26565b505f5f610f278c868a606001518b60800151610219565b9150915081610f62576040517f67988d3300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80610f99576040517fab1b236b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505f878260200151604051602001610fb3929190612fe3565b604051602081830303815290604052805190602001209050828195509550505050509550959350505050565b60325481565b438263ffffffff1610611024576040517f252f8a0e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4363ffffffff1660335f9054906101000a900463ffffffff1683611048919061300a565b63ffffffff161015611086576040517f305c3e9300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f600260325460405160200161109c919061231f565b6040516020818303038152906040526040516110b89190613093565b602060405180830381855afa1580156110d3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906110f691906130bd565b9050808614611131576040517f0af806e000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61113f8787878787610403565b5090505f5f90505b8686905081101561120a576034548260200151828151811061116c5761116b612a59565b5b60200260200101516bffffffffffffffffffffffff1661118c91906130e8565b603554835f015183815181106111a5576111a4612a59565b5b60200260200101516bffffffffffffffffffffffff166111c591906130e8565b10156111fd576040517f6d8605db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8080600101915050611147565b5060325f81548092919061121d90613129565b919050555050505050505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b60355481565b61125d611e75565b611265611e8d565b835f0151815f6003811061127c5761127b612a59565b5b60200201818152505083602001518160016003811061129e5761129d612a59565b5b60200201818152505082816002600381106112bc576112bb612a59565b5b6020020181815250505f60408360608460076107d05a03fa9050805f81036112e057fe5b5080611318576040517f4633be3200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b611328611e75565b611330611eaf565b835f0151815f6004811061134757611346612a59565b5b60200201818152505083602001518160016004811061136957611368612a59565b5b602002018181525050825f01518160026004811061138a57611389612a59565b5b6020020181815250508260200151816003600481106113ac576113ab612a59565b5b6020020181815250505f60408360808460066107d05a03fa9050805f81036113d057fe5b5080611408576040517fd4b68fd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b611418611ed1565b604051806040016040528060405180604001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed815250815260200160405180604001604052807f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec81526020017f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d815250815250905090565b6114e2611e75565b6040518060400160405280600181526020016002815250905090565b611506611e75565b5f5f90505f5f90505f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47855f1c61153d9190612b95565b90505b6001156115e95761155081611b06565b80935081945050507f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478061158757611586612b68565b5b82830983036115af576040518060400160405280828152602001838152509350505050611604565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47806115de576115dd612b68565b5b600182089050611540565b60405180604001604052805f81526020015f81525093505050505b919050565b5f5f5f60405180604001604052808981526020018781525090505f6040518060400160405280898152602001878152509050611643611ef7565b5f5f90505b6002811015611861575f60068261165f91906130e8565b905084826002811061167457611673612a59565b5b60200201515f0151835f836116899190613170565b600c811061169a57611699612a59565b5b6020020181815250508482600281106116b6576116b5612a59565b5b602002015160200151836001836116cd9190613170565b600c81106116de576116dd612a59565b5b6020020181815250508382600281106116fa576116f9612a59565b5b60200201515f01515f6002811061171457611713612a59565b5b6020020151836002836117279190613170565b600c811061173857611737612a59565b5b60200201818152505083826002811061175457611753612a59565b5b60200201515f015160016002811061176f5761176e612a59565b5b6020020151836003836117829190613170565b600c811061179357611792612a59565b5b6020020181815250508382600281106117af576117ae612a59565b5b6020020151602001515f600281106117ca576117c9612a59565b5b6020020151836004836117dd9190613170565b600c81106117ee576117ed612a59565b5b60200201818152505083826002811061180a57611809612a59565b5b60200201516020015160016002811061182657611825612a59565b5b6020020151836005836118399190613170565b600c811061184a57611849612a59565b5b602002018181525050508080600101915050611648565b5061186a611f1a565b5f6020826020600c028560088cfa9050805f835f6001811061188f5761188e612a59565b5b602002015114159650965050505050509550959350505050565b5f5f6118b484611bfb565b9050808360ff166001901b116118f6576040517fca95733300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8091505092915050565b5f81515f52816020015160205260405f209050919050565b5f5f5f90505b5f83111561194a576001836119339190612c53565b831692508080611942906131b0565b91505061191e565b80915050919050565b61195b611e75565b6102008261ffff161061199a576040517fff89d4fa00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018261ffff16036119ae57829050611a32565b5f60405180604001604052805f81526020015f81525090505f8490505f600190505f5f90505b8161ffff168661ffff1610611a2a576001808260ff168861ffff16901c1661ffff1603611a0857611a058484611320565b93505b611a128384611320565b925060018261ffff16901b91508060010190506119d4565b839450505050505b92915050565b611a40611e75565b5f825f0151148015611a5557505f8260200151145b15611a765760405180604001604052805f81526020015f8152509050611aeb565b6040518060400160405280835f015181526020017f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd478460200151611aba9190612b95565b7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611ae59190612c53565b81525090505b919050565b5f60018260ff1684901c16600114905092915050565b5f5f5f7f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b3857611b37612b68565b5b60037f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b6957611b68612b68565b5b867f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4780611b9957611b98612b68565b5b888909090890505f611bec827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f527f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47611d03565b90508181935093505050915091565b5f61010082511115611c39576040517ffb4a9c8e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f825103611c49575f9050611cfe565b5f5f835f81518110611c5e57611c5d612a59565b5b602001015160f81c60f81b60f81c60ff166001901b91505f600190505b8451811015611cf757848181518110611c9757611c96612a59565b5b602001015160f81c60f81b60f81c60ff166001901b9150828211611ce7576040517f80c8834800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183179250806001019050611c7b565b5081925050505b919050565b5f5f611d0d611f1a565b611d15611f3c565b6020815f60068110611d2a57611d29612a59565b5b602002018181525050602081600160068110611d4957611d48612a59565b5b602002018181525050602081600260068110611d6857611d67612a59565b5b6020020181815250508681600360068110611d8657611d85612a59565b5b6020020181815250508581600460068110611da457611da3612a59565b5b6020020181815250508481600560068110611dc257611dc1612a59565b5b60200201818152505060208260c08360056107d05a03fa9250825f8103611de557fe5b5082611e1d576040517fd51edae300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815f60018110611e3057611e2f612a59565b5b602002015193505050509392505050565b604051806040016040528060608152602001606081525090565b604051806040016040528060608152602001606081525090565b60405180604001604052805f81526020015f81525090565b6040518060600160405280600390602082028036833780820191505090505090565b6040518060800160405280600490602082028036833780820191505090505090565b6040518060400160405280611ee4611f5e565b8152602001611ef1611f5e565b81525090565b604051806101800160405280600c90602082028036833780820191505090505090565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060c00160405280600690602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b611fa381611f91565b8114611fad575f5ffd5b50565b5f81359050611fbe81611f9a565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61200e82611fc8565b810181811067ffffffffffffffff8211171561202d5761202c611fd8565b5b80604052505050565b5f61203f611f80565b905061204b8282612005565b919050565b5f5ffd5b5f819050919050565b61206681612054565b8114612070575f5ffd5b50565b5f813590506120818161205d565b92915050565b5f6040828403121561209c5761209b611fc4565b5b6120a66040612036565b90505f6120b584828501612073565b5f8301525060206120c884828501612073565b60208301525092915050565b5f5ffd5b5f67ffffffffffffffff8211156120f2576120f1611fd8565b5b602082029050919050565b5f5ffd5b5f61211361210e846120d8565b612036565b9050806020840283018581111561212d5761212c6120fd565b5b835b8181101561215657806121428882612073565b84526020840193505060208101905061212f565b5050509392505050565b5f82601f830112612174576121736120d4565b5b6002612181848285612101565b91505092915050565b5f6080828403121561219f5761219e611fc4565b5b6121a96040612036565b90505f6121b884828501612160565b5f8301525060406121cb84828501612160565b60208301525092915050565b5f5f5f5f61012085870312156121f0576121ef611f89565b5b5f6121fd87828801611fb0565b945050602061220e87828801612087565b935050606061221f8782880161218a565b92505060e061223087828801612087565b91505092959194509250565b5f8115159050919050565b6122508161223c565b82525050565b5f6040820190506122695f830185612247565b6122766020830184612247565b9392505050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f6122bf6122ba6122b58461227d565b61229c565b61227d565b9050919050565b5f6122d0826122a5565b9050919050565b5f6122e1826122c6565b9050919050565b6122f1816122d7565b82525050565b5f60208201905061230a5f8301846122e8565b92915050565b61231981612054565b82525050565b5f6020820190506123325f830184612310565b92915050565b5f63ffffffff82169050919050565b61235081612338565b82525050565b5f6020820190506123695f830184612347565b92915050565b5f612379826122c6565b9050919050565b6123898161236f565b82525050565b5f6020820190506123a25f830184612380565b92915050565b5f6123b2826122c6565b9050919050565b6123c2816123a8565b82525050565b5f6020820190506123db5f8301846123b9565b92915050565b5f5ffd5b5f5f83601f8401126123fa576123f96120d4565b5b8235905067ffffffffffffffff811115612417576124166123e1565b5b602083019150836001820283011115612433576124326120fd565b5b9250929050565b61244381612338565b811461244d575f5ffd5b50565b5f8135905061245e8161243a565b92915050565b5f67ffffffffffffffff82111561247e5761247d611fd8565b5b602082029050602081019050919050565b5f6124a161249c84612464565b612036565b905080838252602082019050602084028301858111156124c4576124c36120fd565b5b835b818110156124ed57806124d98882612450565b8452602084019350506020810190506124c6565b5050509392505050565b5f82601f83011261250b5761250a6120d4565b5b813561251b84826020860161248f565b91505092915050565b5f67ffffffffffffffff82111561253e5761253d611fd8565b5b602082029050602081019050919050565b5f61256161255c84612524565b612036565b90508083825260208201905060408402830185811115612584576125836120fd565b5b835b818110156125ad57806125998882612087565b845260208401935050604081019050612586565b5050509392505050565b5f82601f8301126125cb576125ca6120d4565b5b81356125db84826020860161254f565b91505092915050565b5f67ffffffffffffffff8211156125fe576125fd611fd8565b5b602082029050602081019050919050565b5f61262161261c846125e4565b612036565b90508083825260208201905060208402830185811115612644576126436120fd565b5b835b8181101561268b57803567ffffffffffffffff811115612669576126686120d4565b5b80860161267689826124f7565b85526020850194505050602081019050612646565b5050509392505050565b5f82601f8301126126a9576126a86120d4565b5b81356126b984826020860161260f565b91505092915050565b5f61018082840312156126d8576126d7611fc4565b5b6126e3610100612036565b90505f82013567ffffffffffffffff81111561270257612701612050565b5b61270e848285016124f7565b5f83015250602082013567ffffffffffffffff81111561273157612730612050565b5b61273d848285016125b7565b602083015250604082013567ffffffffffffffff81111561276157612760612050565b5b61276d848285016125b7565b60408301525060606127818482850161218a565b60608301525060e061279584828501612087565b60808301525061012082013567ffffffffffffffff8111156127ba576127b9612050565b5b6127c6848285016124f7565b60a08301525061014082013567ffffffffffffffff8111156127eb576127ea612050565b5b6127f7848285016124f7565b60c08301525061016082013567ffffffffffffffff81111561281c5761281b612050565b5b61282884828501612695565b60e08301525092915050565b5f5f5f5f5f6080868803121561284d5761284c611f89565b5b5f61285a88828901611fb0565b955050602086013567ffffffffffffffff81111561287b5761287a611f8d565b5b612887888289016123e5565b9450945050604061289a88828901612450565b925050606086013567ffffffffffffffff8111156128bb576128ba611f8d565b5b6128c7888289016126c2565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6bffffffffffffffffffffffff82169050919050565b61291d816128fd565b82525050565b5f61292e8383612914565b60208301905092915050565b5f602082019050919050565b5f612950826128d4565b61295a81856128de565b9350612965836128ee565b805f5b8381101561299557815161297c8882612923565b97506129878361293a565b925050600181019050612968565b5085935050505092915050565b5f604083015f8301518482035f8601526129bc8282612946565b915050602083015184820360208601526129d68282612946565b9150508091505092915050565b6129ec81611f91565b82525050565b5f6040820190508181035f830152612a0a81856129a2565b9050612a1960208301846129e3565b9392505050565b5f612a2a826122c6565b9050919050565b612a3a81612a20565b82525050565b5f602082019050612a535f830184612a31565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b612aa0612a9b82611f91565b612a86565b82525050565b5f819050919050565b612ac0612abb82612054565b612aa6565b82525050565b5f612ad1828c612a8f565b602082019150612ae1828b612aaf565b602082019150612af1828a612aaf565b602082019150612b018289612aaf565b602082019150612b118288612aaf565b602082019150612b218287612aaf565b602082019150612b318286612aaf565b602082019150612b418285612aaf565b602082019150612b518284612aaf565b6020820191508190509a9950505050505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f612b9f82612054565b9150612baa83612054565b925082612bba57612bb9612b68565b5b828206905092915050565b5f60ff82169050919050565b612bda81612bc5565b8114612be4575f5ffd5b50565b5f81519050612bf581612bd1565b92915050565b5f60208284031215612c1057612c0f611f89565b5b5f612c1d84828501612be7565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612c5d82612054565b9150612c6883612054565b9250828203905081811115612c8057612c7f612c26565b5b92915050565b5f612ca0612c9b612c9684612338565b61229c565b612054565b9050919050565b612cb081612c86565b82525050565b5f606082019050612cc95f8301866129e3565b612cd66020830185612347565b612ce36040830184612ca7565b949350505050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b612d1781612ceb565b8114612d21575f5ffd5b50565b5f81519050612d3281612d0e565b92915050565b5f60208284031215612d4d57612d4c611f89565b5b5f612d5a84828501612d24565b91505092915050565b612d6c81612bc5565b82525050565b5f606082019050612d855f830186612d63565b612d926020830185612347565b612d9f6040830184612ca7565b949350505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000082169050919050565b612ddb81612da7565b8114612de5575f5ffd5b50565b5f81519050612df681612dd2565b92915050565b5f60208284031215612e1157612e10611f89565b5b5f612e1e84828501612de8565b91505092915050565b612e30816128fd565b8114612e3a575f5ffd5b50565b5f81519050612e4b81612e27565b92915050565b5f60208284031215612e6657612e65611f89565b5b5f612e7384828501612e3d565b91505092915050565b5f608082019050612e8f5f830187612d63565b612e9c6020830186612347565b612ea960408301856129e3565b612eb66060830184612ca7565b95945050505050565b5f612ec9826128fd565b9150612ed4836128fd565b925082820390506bffffffffffffffffffffffff811115612ef857612ef7612c26565b5b92915050565b5f8160e01b9050919050565b5f612f1482612efe565b9050919050565b612f2c612f2782612338565b612f0a565b82525050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b612f5e81611f91565b82525050565b5f612f6f8383612f55565b60208301905092915050565b5f602082019050919050565b5f612f9182612f32565b612f9b8185612f3c565b9350612fa683612f46565b805f5b83811015612fd6578151612fbd8882612f64565b9750612fc883612f7b565b925050600181019050612fa9565b5085935050505092915050565b5f612fee8285612f1b565b600482019150612ffe8284612f87565b91508190509392505050565b5f61301482612338565b915061301f83612338565b9250828201905063ffffffff81111561303b5761303a612c26565b5b92915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f61306d82613041565b613077818561304b565b9350613087818560208601613055565b80840191505092915050565b5f61309e8284613063565b915081905092915050565b5f815190506130b781611f9a565b92915050565b5f602082840312156130d2576130d1611f89565b5b5f6130df848285016130a9565b91505092915050565b5f6130f282612054565b91506130fd83612054565b925082820261310b81612054565b9150828204841483151761312257613121612c26565b5b5092915050565b5f61313382612054565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361316557613164612c26565b5b600182019050919050565b5f61317a82612054565b915061318583612054565b925082820190508082111561319d5761319c612c26565b5b92915050565b5f61ffff82169050919050565b5f6131ba826131a3565b915061ffff82036131ce576131cd612c26565b5b60018201905091905056fea26469706673582212207d80a96ec44efa5c51139b37f0ba1382535f81e3e0a5c2216b2847c0a8c6c1f864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA7W_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0oW\x80cm\x14\xA9\x87\x14a\x01TW\x80cn\xFBF6\x14a\x01rW\x80c\x83\x81\xF5\x8A\x14a\x01\xA3W\x80c\x85\x04zI\x14a\x01\xC1W\x80c\xDF\\\xF7#\x14a\x01\xDDW\x80c\xEF\x02DX\x14a\x01\xFBWa\0\xA7V[\x80c\x17\x1F\x1D[\x14a\0\xABW\x80c]\xF4YF\x14a\0\xDCW\x80c^Q\x0B`\x14a\0\xFAW\x80c^\x8B?-\x14a\x01\x18W\x80ch0H5\x14a\x016W[__\xFD[a\0\xC5`\x04\x806\x03\x81\x01\x90a\0\xC0\x91\x90a!\xD7V[a\x02\x19V[`@Qa\0\xD3\x92\x91\x90a\"VV[`@Q\x80\x91\x03\x90\xF3[a\0\xE4a\x03|V[`@Qa\0\xF1\x91\x90a\"\xF7V[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x03\xA0V[`@Qa\x01\x0F\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x03\xA6V[`@Qa\x01-\x91\x90a#VV[`@Q\x80\x91\x03\x90\xF3[a\x01>a\x03\xBBV[`@Qa\x01K\x91\x90a#\x8FV[`@Q\x80\x91\x03\x90\xF3[a\x01\\a\x03\xDFV[`@Qa\x01i\x91\x90a#\xC8V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8C`\x04\x806\x03\x81\x01\x90a\x01\x87\x91\x90a(4V[a\x04\x03V[`@Qa\x01\x9A\x92\x91\x90a)\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xABa\x0F\xDFV[`@Qa\x01\xB8\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDB`\x04\x806\x03\x81\x01\x90a\x01\xD6\x91\x90a(4V[a\x0F\xE5V[\0[a\x01\xE5a\x12+V[`@Qa\x01\xF2\x91\x90a*@V[`@Q\x80\x91\x03\x90\xF3[a\x02\x03a\x12OV[`@Qa\x02\x10\x91\x90a#\x1FV[`@Q\x80\x91\x03\x90\xF3[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87_\x01Q\x88` \x01Q\x88_\x01Q_`\x02\x81\x10a\x02]Wa\x02\\a*YV[[` \x02\x01Q\x89_\x01Q`\x01`\x02\x81\x10a\x02yWa\x02xa*YV[[` \x02\x01Q\x8A` \x01Q_`\x02\x81\x10a\x02\x95Wa\x02\x94a*YV[[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x02\xB2Wa\x02\xB1a*YV[[` \x02\x01Q\x8B_\x01Q\x8C` \x01Q`@Q` \x01a\x02\xD8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a*\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x02\xFA\x91\x90a+\x95V[\x90Pa\x03ja\x03$a\x03\x15\x83\x89a\x12U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03,a\x14\x10V[a\x03`a\x03I\x85a\x03;a\x14\xDAV[a\x12U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03R\x8Ca\x14\xFEV[a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88b\x01\xD4\xC0a\x16\tV[\x80\x93P\x81\x94PPPP\x94P\x94\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`4T\x81V[`3_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x0Ba\x1EAV[__\x86\x86\x90P\x03a\x04HW`@Q\x7F\x1F\x04\x05\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x86\x86\x90P\x14\x80\x15a\x04fWP\x82`\xA0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04yWP\x82`\xC0\x01QQ\x86\x86\x90P\x14[\x80\x15a\x04\x8CWP\x82`\xE0\x01QQ\x86\x86\x90P\x14[a\x04\xC2W`@Q\x7FCqJ\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_\x01QQ\x83` \x01QQ\x14a\x05\x04W`@Q\x7F_\x83/A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a\x05IW`@Q\x7FK\x87OE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x05ia\x1EAV[\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x86Wa\x05\x85a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xB4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP\x87\x87\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xDAWa\x05\xD9a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x08W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RPa\x06\x18a\x1E[V[\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x067Wa\x066a\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06eW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81_\x01\x81\x90RP\x85` \x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x8CWa\x06\x8Ba\x1F\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x81` \x01\x81\x90RP_a\x07\x9D\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x98\x91\x90a+\xFBV[a\x18\xA9V[\x90P__\x90P[\x87` \x01QQ\x81\x10\x15a\n\x14Wa\x07\xD8\x88` \x01Q\x82\x81Q\x81\x10a\x07\xCBWa\x07\xCAa*YV[[` \x02` \x01\x01Qa\x19\0V[\x83` \x01Q\x82\x81Q\x81\x10a\x07\xEFWa\x07\xEEa*YV[[` \x02` \x01\x01\x81\x81RPP_\x81\x14a\x08\x88W\x82` \x01Q`\x01\x82a\x08\x14\x91\x90a,SV[\x81Q\x81\x10a\x08%Wa\x08$a*YV[[` \x02` \x01\x01Q_\x1C\x83` \x01Q\x82\x81Q\x81\x10a\x08FWa\x08Ea*YV[[` \x02` \x01\x01Q_\x1C\x11a\x08\x87W`@Q\x7F\xFFq\x94\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a\x08\xDBWa\x08\xDAa*YV[[` \x02` \x01\x01Q\x8B\x8B_\x01Q\x85\x81Q\x81\x10a\x08\xFAWa\x08\xF9a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t \x93\x92\x91\x90a,\xB6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t_\x91\x90a-8V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83_\x01Q\x82\x81Q\x81\x10a\t\x8FWa\t\x8Ea*YV[[` \x02` \x01\x01\x81\x81RPPa\n\x05a\t\xF6a\t\xC9\x84\x86_\x01Q\x85\x81Q\x81\x10a\t\xBBWa\t\xBAa*YV[[` \x02` \x01\x01Q\x16a\x19\x18V[\x8A` \x01Q\x84\x81Q\x81\x10a\t\xE0Wa\t\xDFa*YV[[` \x02` \x01\x01Qa\x19S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80\x80`\x01\x01\x91PPa\x07\xA4V[PPa\n\x1F\x83a\x1A8V[\x92P__\x90P[\x89\x89\x90P\x81\x10\x15a\x0F\x10W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ch\xBC\xCA\xAC\x8B\x8B\x84\x81\x81\x10a\n\x80Wa\n\x7Fa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8A\x8A`\xA0\x01Q\x85\x81Q\x81\x10a\n\xA5Wa\n\xA4a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xCB\x93\x92\x91\x90a-rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\n\x91\x90a-\xFCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16a\x0B<\x88`@\x01Q\x83\x81Q\x81\x10a\x0B/Wa\x0B.a*YV[[` \x02` \x01\x01Qa\x19\0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x0B~W`@Q\x7F\xE11\n\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xAF\x87`@\x01Q\x82\x81Q\x81\x10a\x0B\x98Wa\x0B\x97a*YV[[` \x02` \x01\x01Q\x85a\x13 \x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC8)LV\x8B\x8B\x84\x81\x81\x10a\x0C\0Wa\x0B\xFFa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8A\x8A`\xC0\x01Q\x85\x81Q\x81\x10a\x0C%Wa\x0C$a*YV[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CK\x93\x92\x91\x90a-rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8A\x91\x90a.QV[\x83` \x01Q\x82\x81Q\x81\x10a\x0C\xA1Wa\x0C\xA0a*YV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82` \x01Q\x81\x81Q\x81\x10a\x0C\xE2Wa\x0C\xE1a*YV[[` \x02` \x01\x01Q\x83_\x01Q\x82\x81Q\x81\x10a\r\0Wa\x0C\xFFa*YV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP__\x90P__\x90P[\x88` \x01QQ\x81\x10\x15a\x0F\x01Wa\r\x86\x84_\x01Q\x82\x81Q\x81\x10a\rYWa\rXa*YV[[` \x02` \x01\x01Q\x8D\x8D\x86\x81\x81\x10a\rtWa\rsa*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1Ca\x1A\xF0V[\x15a\x0E\xF4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\xBE\x94\xAE\x8D\x8D\x86\x81\x81\x10a\r\xDAWa\r\xD9a*YV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x87` \x01Q\x85\x81Q\x81\x10a\r\xFFWa\r\xFEa*YV[[` \x02` \x01\x01Q\x8D`\xE0\x01Q\x88\x81Q\x81\x10a\x0E\x1EWa\x0E\x1Da*YV[[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x0E8Wa\x0E7a*YV[[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E_\x94\x93\x92\x91\x90a.|V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9E\x91\x90a.QV[\x85_\x01Q\x84\x81Q\x81\x10a\x0E\xB4Wa\x0E\xB3a*YV[[` \x02` \x01\x01\x81\x81Qa\x0E\xC8\x91\x90a.\xBFV[\x91P\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81`\x01\x01\x91P[\x80\x80`\x01\x01\x91PPa\r3V[PP\x80\x80`\x01\x01\x91PPa\n&V[P__a\x0F'\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x02\x19V[\x91P\x91P\x81a\x0FbW`@Q\x7Fg\x98\x8D3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x0F\x99W`@Q\x7F\xAB\x1B#k\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_\x87\x82` \x01Q`@Q` \x01a\x0F\xB3\x92\x91\x90a/\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x82\x81\x95P\x95PPPPP\x95P\x95\x93PPPPV[`2T\x81V[C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x10$W`@Q\x7F%/\x8A\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Cc\xFF\xFF\xFF\xFF\x16`3_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x83a\x10H\x91\x90a0\nV[c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10\x86W`@Q\x7F0\\>\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x02`2T`@Q` \x01a\x10\x9C\x91\x90a#\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Qa\x10\xB8\x91\x90a0\x93V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x10\xD3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF6\x91\x90a0\xBDV[\x90P\x80\x86\x14a\x111W`@Q\x7F\n\xF8\x06\xE0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11?\x87\x87\x87\x87\x87a\x04\x03V[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\x12\nW`4T\x82` \x01Q\x82\x81Q\x81\x10a\x11lWa\x11ka*YV[[` \x02` \x01\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x8C\x91\x90a0\xE8V[`5T\x83_\x01Q\x83\x81Q\x81\x10a\x11\xA5Wa\x11\xA4a*YV[[` \x02` \x01\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\xC5\x91\x90a0\xE8V[\x10\x15a\x11\xFDW`@Q\x7Fm\x86\x05\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x11GV[P`2_\x81T\x80\x92\x91\x90a\x12\x1D\x90a1)V[\x91\x90PUPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`5T\x81V[a\x12]a\x1EuV[a\x12ea\x1E\x8DV[\x83_\x01Q\x81_`\x03\x81\x10a\x12|Wa\x12{a*YV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x03\x81\x10a\x12\x9EWa\x12\x9Da*YV[[` \x02\x01\x81\x81RPP\x82\x81`\x02`\x03\x81\x10a\x12\xBCWa\x12\xBBa*YV[[` \x02\x01\x81\x81RPP_`@\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x12\xE0W\xFE[P\x80a\x13\x18W`@Q\x7FF3\xBE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x13(a\x1EuV[a\x130a\x1E\xAFV[\x83_\x01Q\x81_`\x04\x81\x10a\x13GWa\x13Fa*YV[[` \x02\x01\x81\x81RPP\x83` \x01Q\x81`\x01`\x04\x81\x10a\x13iWa\x13ha*YV[[` \x02\x01\x81\x81RPP\x82_\x01Q\x81`\x02`\x04\x81\x10a\x13\x8AWa\x13\x89a*YV[[` \x02\x01\x81\x81RPP\x82` \x01Q\x81`\x03`\x04\x81\x10a\x13\xACWa\x13\xABa*YV[[` \x02\x01\x81\x81RPP_`@\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80_\x81\x03a\x13\xD0W\xFE[P\x80a\x14\x08W`@Q\x7F\xD4\xB6\x8F\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[a\x14\x18a\x1E\xD1V[`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x81R` \x01\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D\x81RP\x81RP\x90P\x90V[a\x14\xE2a\x1EuV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x02\x81RP\x90P\x90V[a\x15\x06a\x1EuV[__\x90P__\x90P_\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x85_\x1Ca\x15=\x91\x90a+\x95V[\x90P[`\x01\x15a\x15\xE9Wa\x15P\x81a\x1B\x06V[\x80\x93P\x81\x94PPP\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\x87Wa\x15\x86a+hV[[\x82\x83\t\x83\x03a\x15\xAFW`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP\x93PPPPa\x16\x04V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x15\xDEWa\x15\xDDa+hV[[`\x01\x82\x08\x90Pa\x15@V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x93PPPP[\x91\x90PV[___`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90P_`@Q\x80`@\x01`@R\x80\x89\x81R` \x01\x87\x81RP\x90Pa\x16Ca\x1E\xF7V[__\x90P[`\x02\x81\x10\x15a\x18aW_`\x06\x82a\x16_\x91\x90a0\xE8V[\x90P\x84\x82`\x02\x81\x10a\x16tWa\x16sa*YV[[` \x02\x01Q_\x01Q\x83_\x83a\x16\x89\x91\x90a1pV[`\x0C\x81\x10a\x16\x9AWa\x16\x99a*YV[[` \x02\x01\x81\x81RPP\x84\x82`\x02\x81\x10a\x16\xB6Wa\x16\xB5a*YV[[` \x02\x01Q` \x01Q\x83`\x01\x83a\x16\xCD\x91\x90a1pV[`\x0C\x81\x10a\x16\xDEWa\x16\xDDa*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x16\xFAWa\x16\xF9a*YV[[` \x02\x01Q_\x01Q_`\x02\x81\x10a\x17\x14Wa\x17\x13a*YV[[` \x02\x01Q\x83`\x02\x83a\x17'\x91\x90a1pV[`\x0C\x81\x10a\x178Wa\x177a*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17TWa\x17Sa*YV[[` \x02\x01Q_\x01Q`\x01`\x02\x81\x10a\x17oWa\x17na*YV[[` \x02\x01Q\x83`\x03\x83a\x17\x82\x91\x90a1pV[`\x0C\x81\x10a\x17\x93Wa\x17\x92a*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x17\xAFWa\x17\xAEa*YV[[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x17\xCAWa\x17\xC9a*YV[[` \x02\x01Q\x83`\x04\x83a\x17\xDD\x91\x90a1pV[`\x0C\x81\x10a\x17\xEEWa\x17\xEDa*YV[[` \x02\x01\x81\x81RPP\x83\x82`\x02\x81\x10a\x18\nWa\x18\ta*YV[[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x18&Wa\x18%a*YV[[` \x02\x01Q\x83`\x05\x83a\x189\x91\x90a1pV[`\x0C\x81\x10a\x18JWa\x18Ia*YV[[` \x02\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x16HV[Pa\x18ja\x1F\x1AV[_` \x82` `\x0C\x02\x85`\x08\x8C\xFA\x90P\x80_\x83_`\x01\x81\x10a\x18\x8FWa\x18\x8Ea*YV[[` \x02\x01Q\x14\x15\x96P\x96PPPPPP\x95P\x95\x93PPPPV[__a\x18\xB4\x84a\x1B\xFBV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\x18\xF6W`@Q\x7F\xCA\x95s3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[_\x81Q_R\x81` \x01Q` R`@_ \x90P\x91\x90PV[___\x90P[_\x83\x11\x15a\x19JW`\x01\x83a\x193\x91\x90a,SV[\x83\x16\x92P\x80\x80a\x19B\x90a1\xB0V[\x91PPa\x19\x1EV[\x80\x91PP\x91\x90PV[a\x19[a\x1EuV[a\x02\0\x82a\xFF\xFF\x16\x10a\x19\x9AW`@Q\x7F\xFF\x89\xD4\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82a\xFF\xFF\x16\x03a\x19\xAEW\x82\x90Pa\x1A2V[_`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90P_\x84\x90P_`\x01\x90P__\x90P[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a\x1A*W`\x01\x80\x82`\xFF\x16\x88a\xFF\xFF\x16\x90\x1C\x16a\xFF\xFF\x16\x03a\x1A\x08Wa\x1A\x05\x84\x84a\x13 V[\x93P[a\x1A\x12\x83\x84a\x13 V[\x92P`\x01\x82a\xFF\xFF\x16\x90\x1B\x91P\x80`\x01\x01\x90Pa\x19\xD4V[\x83\x94PPPPP[\x92\x91PPV[a\x1A@a\x1EuV[_\x82_\x01Q\x14\x80\x15a\x1AUWP_\x82` \x01Q\x14[\x15a\x1AvW`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90Pa\x1A\xEBV[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x1A\xBA\x91\x90a+\x95V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1A\xE5\x91\x90a,SV[\x81RP\x90P[\x91\x90PV[_`\x01\x82`\xFF\x16\x84\x90\x1C\x16`\x01\x14\x90P\x92\x91PPV[___\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B8Wa\x1B7a+hV[[`\x03\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1BiWa\x1Bha+hV[[\x86\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x80a\x1B\x99Wa\x1B\x98a+hV[[\x88\x89\t\t\x08\x90P_a\x1B\xEC\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1D\x03V[\x90P\x81\x81\x93P\x93PPP\x91P\x91V[_a\x01\0\x82Q\x11\x15a\x1C9W`@Q\x7F\xFBJ\x9C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82Q\x03a\x1CIW_\x90Pa\x1C\xFEV[__\x83_\x81Q\x81\x10a\x1C^Wa\x1C]a*YV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P_`\x01\x90P[\x84Q\x81\x10\x15a\x1C\xF7W\x84\x81\x81Q\x81\x10a\x1C\x97Wa\x1C\x96a*YV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16`\x01\x90\x1B\x91P\x82\x82\x11a\x1C\xE7W`@Q\x7F\x80\xC8\x83H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x17\x92P\x80`\x01\x01\x90Pa\x1C{V[P\x81\x92PPP[\x91\x90PV[__a\x1D\ra\x1F\x1AV[a\x1D\x15a\x1F<V[` \x81_`\x06\x81\x10a\x1D*Wa\x1D)a*YV[[` \x02\x01\x81\x81RPP` \x81`\x01`\x06\x81\x10a\x1DIWa\x1DHa*YV[[` \x02\x01\x81\x81RPP` \x81`\x02`\x06\x81\x10a\x1DhWa\x1Dga*YV[[` \x02\x01\x81\x81RPP\x86\x81`\x03`\x06\x81\x10a\x1D\x86Wa\x1D\x85a*YV[[` \x02\x01\x81\x81RPP\x85\x81`\x04`\x06\x81\x10a\x1D\xA4Wa\x1D\xA3a*YV[[` \x02\x01\x81\x81RPP\x84\x81`\x05`\x06\x81\x10a\x1D\xC2Wa\x1D\xC1a*YV[[` \x02\x01\x81\x81RPP` \x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82_\x81\x03a\x1D\xE5W\xFE[P\x82a\x1E\x1DW`@Q\x7F\xD5\x1E\xDA\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_`\x01\x81\x10a\x1E0Wa\x1E/a*YV[[` \x02\x01Q\x93PPPP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80a\x1E\xE4a\x1F^V[\x81R` \x01a\x1E\xF1a\x1F^V[\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x1F\xA3\x81a\x1F\x91V[\x81\x14a\x1F\xADW__\xFD[PV[_\x815\x90Pa\x1F\xBE\x81a\x1F\x9AV[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a \x0E\x82a\x1F\xC8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a -Wa ,a\x1F\xD8V[[\x80`@RPPPV[_a ?a\x1F\x80V[\x90Pa K\x82\x82a \x05V[\x91\x90PV[__\xFD[_\x81\x90P\x91\x90PV[a f\x81a TV[\x81\x14a pW__\xFD[PV[_\x815\x90Pa \x81\x81a ]V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a \x9CWa \x9Ba\x1F\xC4V[[a \xA6`@a 6V[\x90P_a \xB5\x84\x82\x85\x01a sV[_\x83\x01RP` a \xC8\x84\x82\x85\x01a sV[` \x83\x01RP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a \xF2Wa \xF1a\x1F\xD8V[[` \x82\x02\x90P\x91\x90PV[__\xFD[_a!\x13a!\x0E\x84a \xD8V[a 6V[\x90P\x80` \x84\x02\x83\x01\x85\x81\x11\x15a!-Wa!,a \xFDV[[\x83[\x81\x81\x10\x15a!VW\x80a!B\x88\x82a sV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa!/V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a!tWa!sa \xD4V[[`\x02a!\x81\x84\x82\x85a!\x01V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a!\x9FWa!\x9Ea\x1F\xC4V[[a!\xA9`@a 6V[\x90P_a!\xB8\x84\x82\x85\x01a!`V[_\x83\x01RP`@a!\xCB\x84\x82\x85\x01a!`V[` \x83\x01RP\x92\x91PPV[____a\x01 \x85\x87\x03\x12\x15a!\xF0Wa!\xEFa\x1F\x89V[[_a!\xFD\x87\x82\x88\x01a\x1F\xB0V[\x94PP` a\"\x0E\x87\x82\x88\x01a \x87V[\x93PP``a\"\x1F\x87\x82\x88\x01a!\x8AV[\x92PP`\xE0a\"0\x87\x82\x88\x01a \x87V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[a\"P\x81a\"<V[\x82RPPV[_`@\x82\x01\x90Pa\"i_\x83\x01\x85a\"GV[a\"v` \x83\x01\x84a\"GV[\x93\x92PPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\"\xBFa\"\xBAa\"\xB5\x84a\"}V[a\"\x9CV[a\"}V[\x90P\x91\x90PV[_a\"\xD0\x82a\"\xA5V[\x90P\x91\x90PV[_a\"\xE1\x82a\"\xC6V[\x90P\x91\x90PV[a\"\xF1\x81a\"\xD7V[\x82RPPV[_` \x82\x01\x90Pa#\n_\x83\x01\x84a\"\xE8V[\x92\x91PPV[a#\x19\x81a TV[\x82RPPV[_` \x82\x01\x90Pa#2_\x83\x01\x84a#\x10V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a#P\x81a#8V[\x82RPPV[_` \x82\x01\x90Pa#i_\x83\x01\x84a#GV[\x92\x91PPV[_a#y\x82a\"\xC6V[\x90P\x91\x90PV[a#\x89\x81a#oV[\x82RPPV[_` \x82\x01\x90Pa#\xA2_\x83\x01\x84a#\x80V[\x92\x91PPV[_a#\xB2\x82a\"\xC6V[\x90P\x91\x90PV[a#\xC2\x81a#\xA8V[\x82RPPV[_` \x82\x01\x90Pa#\xDB_\x83\x01\x84a#\xB9V[\x92\x91PPV[__\xFD[__\x83`\x1F\x84\x01\x12a#\xFAWa#\xF9a \xD4V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x17Wa$\x16a#\xE1V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a$3Wa$2a \xFDV[[\x92P\x92\x90PV[a$C\x81a#8V[\x81\x14a$MW__\xFD[PV[_\x815\x90Pa$^\x81a$:V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a$~Wa$}a\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a$\xA1a$\x9C\x84a$dV[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a$\xC4Wa$\xC3a \xFDV[[\x83[\x81\x81\x10\x15a$\xEDW\x80a$\xD9\x88\x82a$PV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa$\xC6V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\x0BWa%\na \xD4V[[\x815a%\x1B\x84\x82` \x86\x01a$\x8FV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%>Wa%=a\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a%aa%\\\x84a%$V[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a%\x84Wa%\x83a \xFDV[[\x83[\x81\x81\x10\x15a%\xADW\x80a%\x99\x88\x82a \x87V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa%\x86V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a%\xCBWa%\xCAa \xD4V[[\x815a%\xDB\x84\x82` \x86\x01a%OV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xFEWa%\xFDa\x1F\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a&!a&\x1C\x84a%\xE4V[a 6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&DWa&Ca \xFDV[[\x83[\x81\x81\x10\x15a&\x8BW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&iWa&ha \xD4V[[\x80\x86\x01a&v\x89\x82a$\xF7V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa&FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a&\xA9Wa&\xA8a \xD4V[[\x815a&\xB9\x84\x82` \x86\x01a&\x0FV[\x91PP\x92\x91PPV[_a\x01\x80\x82\x84\x03\x12\x15a&\xD8Wa&\xD7a\x1F\xC4V[[a&\xE3a\x01\0a 6V[\x90P_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x02Wa'\x01a PV[[a'\x0E\x84\x82\x85\x01a$\xF7V[_\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'1Wa'0a PV[[a'=\x84\x82\x85\x01a%\xB7V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'aWa'`a PV[[a'm\x84\x82\x85\x01a%\xB7V[`@\x83\x01RP``a'\x81\x84\x82\x85\x01a!\x8AV[``\x83\x01RP`\xE0a'\x95\x84\x82\x85\x01a \x87V[`\x80\x83\x01RPa\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xBAWa'\xB9a PV[[a'\xC6\x84\x82\x85\x01a$\xF7V[`\xA0\x83\x01RPa\x01@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xEBWa'\xEAa PV[[a'\xF7\x84\x82\x85\x01a$\xF7V[`\xC0\x83\x01RPa\x01`\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1CWa(\x1Ba PV[[a((\x84\x82\x85\x01a&\x95V[`\xE0\x83\x01RP\x92\x91PPV[_____`\x80\x86\x88\x03\x12\x15a(MWa(La\x1F\x89V[[_a(Z\x88\x82\x89\x01a\x1F\xB0V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a({Wa(za\x1F\x8DV[[a(\x87\x88\x82\x89\x01a#\xE5V[\x94P\x94PP`@a(\x9A\x88\x82\x89\x01a$PV[\x92PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xBBWa(\xBAa\x1F\x8DV[[a(\xC7\x88\x82\x89\x01a&\xC2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)\x1D\x81a(\xFDV[\x82RPPV[_a).\x83\x83a)\x14V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a)P\x82a(\xD4V[a)Z\x81\x85a(\xDEV[\x93Pa)e\x83a(\xEEV[\x80_[\x83\x81\x10\x15a)\x95W\x81Qa)|\x88\x82a)#V[\x97Pa)\x87\x83a):V[\x92PP`\x01\x81\x01\x90Pa)hV[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra)\xBC\x82\x82a)FV[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra)\xD6\x82\x82a)FV[\x91PP\x80\x91PP\x92\x91PPV[a)\xEC\x81a\x1F\x91V[\x82RPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra*\n\x81\x85a)\xA2V[\x90Pa*\x19` \x83\x01\x84a)\xE3V[\x93\x92PPPV[_a**\x82a\"\xC6V[\x90P\x91\x90PV[a*:\x81a* V[\x82RPPV[_` \x82\x01\x90Pa*S_\x83\x01\x84a*1V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[a*\xA0a*\x9B\x82a\x1F\x91V[a*\x86V[\x82RPPV[_\x81\x90P\x91\x90PV[a*\xC0a*\xBB\x82a TV[a*\xA6V[\x82RPPV[_a*\xD1\x82\x8Ca*\x8FV[` \x82\x01\x91Pa*\xE1\x82\x8Ba*\xAFV[` \x82\x01\x91Pa*\xF1\x82\x8Aa*\xAFV[` \x82\x01\x91Pa+\x01\x82\x89a*\xAFV[` \x82\x01\x91Pa+\x11\x82\x88a*\xAFV[` \x82\x01\x91Pa+!\x82\x87a*\xAFV[` \x82\x01\x91Pa+1\x82\x86a*\xAFV[` \x82\x01\x91Pa+A\x82\x85a*\xAFV[` \x82\x01\x91Pa+Q\x82\x84a*\xAFV[` \x82\x01\x91P\x81\x90P\x9A\x99PPPPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a+\x9F\x82a TV[\x91Pa+\xAA\x83a TV[\x92P\x82a+\xBAWa+\xB9a+hV[[\x82\x82\x06\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a+\xDA\x81a+\xC5V[\x81\x14a+\xE4W__\xFD[PV[_\x81Q\x90Pa+\xF5\x81a+\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,\x10Wa,\x0Fa\x1F\x89V[[_a,\x1D\x84\x82\x85\x01a+\xE7V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a,]\x82a TV[\x91Pa,h\x83a TV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a,\x80Wa,\x7Fa,&V[[\x92\x91PPV[_a,\xA0a,\x9Ba,\x96\x84a#8V[a\"\x9CV[a TV[\x90P\x91\x90PV[a,\xB0\x81a,\x86V[\x82RPPV[_``\x82\x01\x90Pa,\xC9_\x83\x01\x86a)\xE3V[a,\xD6` \x83\x01\x85a#GV[a,\xE3`@\x83\x01\x84a,\xA7V[\x94\x93PPPPV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a-\x17\x81a,\xEBV[\x81\x14a-!W__\xFD[PV[_\x81Q\x90Pa-2\x81a-\x0EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a-MWa-La\x1F\x89V[[_a-Z\x84\x82\x85\x01a-$V[\x91PP\x92\x91PPV[a-l\x81a+\xC5V[\x82RPPV[_``\x82\x01\x90Pa-\x85_\x83\x01\x86a-cV[a-\x92` \x83\x01\x85a#GV[a-\x9F`@\x83\x01\x84a,\xA7V[\x94\x93PPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a-\xDB\x81a-\xA7V[\x81\x14a-\xE5W__\xFD[PV[_\x81Q\x90Pa-\xF6\x81a-\xD2V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\x11Wa.\x10a\x1F\x89V[[_a.\x1E\x84\x82\x85\x01a-\xE8V[\x91PP\x92\x91PPV[a.0\x81a(\xFDV[\x81\x14a.:W__\xFD[PV[_\x81Q\x90Pa.K\x81a.'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.fWa.ea\x1F\x89V[[_a.s\x84\x82\x85\x01a.=V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pa.\x8F_\x83\x01\x87a-cV[a.\x9C` \x83\x01\x86a#GV[a.\xA9`@\x83\x01\x85a)\xE3V[a.\xB6``\x83\x01\x84a,\xA7V[\x95\x94PPPPPV[_a.\xC9\x82a(\xFDV[\x91Pa.\xD4\x83a(\xFDV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xF8Wa.\xF7a,&V[[\x92\x91PPV[_\x81`\xE0\x1B\x90P\x91\x90PV[_a/\x14\x82a.\xFEV[\x90P\x91\x90PV[a/,a/'\x82a#8V[a/\nV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a/^\x81a\x1F\x91V[\x82RPPV[_a/o\x83\x83a/UV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a/\x91\x82a/2V[a/\x9B\x81\x85a/<V[\x93Pa/\xA6\x83a/FV[\x80_[\x83\x81\x10\x15a/\xD6W\x81Qa/\xBD\x88\x82a/dV[\x97Pa/\xC8\x83a/{V[\x92PP`\x01\x81\x01\x90Pa/\xA9V[P\x85\x93PPPP\x92\x91PPV[_a/\xEE\x82\x85a/\x1BV[`\x04\x82\x01\x91Pa/\xFE\x82\x84a/\x87V[\x91P\x81\x90P\x93\x92PPPV[_a0\x14\x82a#8V[\x91Pa0\x1F\x83a#8V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a0;Wa0:a,&V[[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a0m\x82a0AV[a0w\x81\x85a0KV[\x93Pa0\x87\x81\x85` \x86\x01a0UV[\x80\x84\x01\x91PP\x92\x91PPV[_a0\x9E\x82\x84a0cV[\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90Pa0\xB7\x81a\x1F\x9AV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a0\xD2Wa0\xD1a\x1F\x89V[[_a0\xDF\x84\x82\x85\x01a0\xA9V[\x91PP\x92\x91PPV[_a0\xF2\x82a TV[\x91Pa0\xFD\x83a TV[\x92P\x82\x82\x02a1\x0B\x81a TV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a1\"Wa1!a,&V[[P\x92\x91PPV[_a13\x82a TV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a1eWa1da,&V[[`\x01\x82\x01\x90P\x91\x90PV[_a1z\x82a TV[\x91Pa1\x85\x83a TV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a1\x9DWa1\x9Ca,&V[[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_a1\xBA\x82a1\xA3V[\x91Pa\xFF\xFF\x82\x03a1\xCEWa1\xCDa,&V[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 }\x80\xA9n\xC4N\xFA\\Q\x13\x9B7\xF0\xBA\x13\x82S_\x81\xE3\xE0\xA5\xC2!k(G\xC0\xA8\xC6\xC1\xF8dsolcC\0\x08\x1C\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
    ```solidity
    error BitmapValueTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
    ```solidity
    error BytesArrayLengthTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
    ```solidity
    error BytesArrayNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
    ```solidity
    error ECAddFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECAddFailed()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 143u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
    ```solidity
    error ECMulFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECMulFailed()";
            const SELECTOR: [u8; 4] = [70u8, 51u8, 190u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FutureBlockNumber()` and selector `0x252f8a0e`.
    ```solidity
    error FutureBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FutureBlockNumber {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FutureBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: FutureBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FutureBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FutureBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FutureBlockNumber()";
            const SELECTOR: [u8; 4] = [37u8, 47u8, 138u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
    ```solidity
    error InputArrayLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputArrayLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputEmptyQuorumNumbers()` and selector `0x1f0405a0`.
    ```solidity
    error InputEmptyQuorumNumbers();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputEmptyQuorumNumbers {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputEmptyQuorumNumbers> for UnderlyingRustTuple<'_> {
            fn from(value: InputEmptyQuorumNumbers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputEmptyQuorumNumbers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputEmptyQuorumNumbers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputEmptyQuorumNumbers()";
            const SELECTOR: [u8; 4] = [31u8, 4u8, 5u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputNonSignerLengthMismatch()` and selector `0x5f832f41`.
    ```solidity
    error InputNonSignerLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputNonSignerLengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputNonSignerLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputNonSignerLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputNonSignerLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputNonSignerLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputNonSignerLengthMismatch()";
            const SELECTOR: [u8; 4] = [95u8, 131u8, 47u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientQuorumThreshold()` and selector `0x6d8605db`.
    ```solidity
    error InsufficientQuorumThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientQuorumThreshold {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientQuorumThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientQuorumThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientQuorumThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientQuorumThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientQuorumThreshold()";
            const SELECTOR: [u8; 4] = [109u8, 134u8, 5u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSPairingKey()` and selector `0x67988d33`.
    ```solidity
    error InvalidBLSPairingKey();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSPairingKey {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBLSPairingKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSPairingKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSPairingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSPairingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSPairingKey()";
            const SELECTOR: [u8; 4] = [103u8, 152u8, 141u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSSignature()` and selector `0xab1b236b`.
    ```solidity
    error InvalidBLSSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBLSSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSSignature()";
            const SELECTOR: [u8; 4] = [171u8, 27u8, 35u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidHash()` and selector `0x0af806e0`.
    ```solidity
    error InvalidHash();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHash {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHash()";
            const SELECTOR: [u8; 4] = [10u8, 248u8, 6u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidQuorumApkHash()` and selector `0xe1310aed`.
    ```solidity
    error InvalidQuorumApkHash();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorumApkHash {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorumApkHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorumApkHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorumApkHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorumApkHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorumApkHash()";
            const SELECTOR: [u8; 4] = [225u8, 49u8, 10u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidReferenceBlocknumber()` and selector `0x4b874f45`.
    ```solidity
    error InvalidReferenceBlocknumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlocknumber {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlocknumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlocknumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlocknumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlocknumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlocknumber()";
            const SELECTOR: [u8; 4] = [75u8, 135u8, 79u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NonSignerPubkeysNotSorted()` and selector `0xff719414`.
    ```solidity
    error NonSignerPubkeysNotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerPubkeysNotSorted {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonSignerPubkeysNotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerPubkeysNotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerPubkeysNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonSignerPubkeysNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonSignerPubkeysNotSorted()";
            const SELECTOR: [u8; 4] = [255u8, 113u8, 148u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyRegistryCoordinatorOwner()` and selector `0xe0e1e762`.
    ```solidity
    error OnlyRegistryCoordinatorOwner();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinatorOwner {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyRegistryCoordinatorOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [224u8, 225u8, 231u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ScalarTooLarge()` and selector `0xff89d4fa`.
    ```solidity
    error ScalarTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarTooLarge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ScalarTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ScalarTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScalarTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ScalarTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ScalarTooLarge()";
            const SELECTOR: [u8; 4] = [255u8, 137u8, 212u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StaleBlockNumber()` and selector `0x305c3e93`.
    ```solidity
    error StaleBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleBlockNumber {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StaleBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: StaleBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StaleBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StaleBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StaleBlockNumber()";
            const SELECTOR: [u8; 4] = [48u8, 92u8, 62u8, 147u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _registryCoordinator);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _registryCoordinator: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._registryCoordinator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._registryCoordinator,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `BLOCK_STALE_MEASURE()` and selector `0x5e8b3f2d`.
    ```solidity
    function BLOCK_STALE_MEASURE() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BLOCK_STALE_MEASURECall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`BLOCK_STALE_MEASURE()`](BLOCK_STALE_MEASURECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BLOCK_STALE_MEASUREReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BLOCK_STALE_MEASURECall> for UnderlyingRustTuple<'_> {
                fn from(value: BLOCK_STALE_MEASURECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BLOCK_STALE_MEASURECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BLOCK_STALE_MEASUREReturn> for UnderlyingRustTuple<'_> {
                fn from(value: BLOCK_STALE_MEASUREReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BLOCK_STALE_MEASUREReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BLOCK_STALE_MEASURECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = BLOCK_STALE_MEASUREReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BLOCK_STALE_MEASURE()";
            const SELECTOR: [u8; 4] = [94u8, 139u8, 63u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `QUORUM_THRESHOLD()` and selector `0x5e510b60`.
    ```solidity
    function QUORUM_THRESHOLD() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUORUM_THRESHOLDCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`QUORUM_THRESHOLD()`](QUORUM_THRESHOLDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUORUM_THRESHOLDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<QUORUM_THRESHOLDCall> for UnderlyingRustTuple<'_> {
                fn from(value: QUORUM_THRESHOLDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUORUM_THRESHOLDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<QUORUM_THRESHOLDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: QUORUM_THRESHOLDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUORUM_THRESHOLDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for QUORUM_THRESHOLDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = QUORUM_THRESHOLDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QUORUM_THRESHOLD()";
            const SELECTOR: [u8; 4] = [94u8, 81u8, 11u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `THRESHOLD_DENOMINATOR()` and selector `0xef024458`.
    ```solidity
    function THRESHOLD_DENOMINATOR() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct THRESHOLD_DENOMINATORCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`THRESHOLD_DENOMINATOR()`](THRESHOLD_DENOMINATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct THRESHOLD_DENOMINATORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<THRESHOLD_DENOMINATORCall> for UnderlyingRustTuple<'_> {
                fn from(value: THRESHOLD_DENOMINATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for THRESHOLD_DENOMINATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<THRESHOLD_DENOMINATORReturn> for UnderlyingRustTuple<'_> {
                fn from(value: THRESHOLD_DENOMINATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for THRESHOLD_DENOMINATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for THRESHOLD_DENOMINATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = THRESHOLD_DENOMINATORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "THRESHOLD_DENOMINATOR()";
            const SELECTOR: [u8; 4] = [239u8, 2u8, 68u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`.
    ```solidity
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0:
            <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [110u8, 251u8, 70u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
    ```solidity
    function delegation() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `increment(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x85047a49`.
    ```solidity
    function increment(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incrementCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`increment(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](incrementCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incrementReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incrementCall> for UnderlyingRustTuple<'_> {
                fn from(value: incrementCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incrementCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incrementReturn> for UnderlyingRustTuple<'_> {
                fn from(value: incrementReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incrementReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for incrementCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = incrementReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increment(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [133u8, 4u8, 122u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `number()` and selector `0x8381f58a`.
    ```solidity
    function number() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numberCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`number()`](numberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numberCall> for UnderlyingRustTuple<'_> {
                fn from(value: numberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: numberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = numberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "number()";
            const SELECTOR: [u8; 4] = [131u8, 129u8, 245u8, 138u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`.
    ```solidity
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        #[allow(missing_docs)]
        pub pairingSuccessful: bool,
        #[allow(missing_docs)]
        pub siganatureIsValid: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationCall> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureAndApkVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [23u8, 31u8, 29u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`Counter`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum CounterCalls {
        #[allow(missing_docs)]
        BLOCK_STALE_MEASURE(BLOCK_STALE_MEASURECall),
        #[allow(missing_docs)]
        QUORUM_THRESHOLD(QUORUM_THRESHOLDCall),
        #[allow(missing_docs)]
        THRESHOLD_DENOMINATOR(THRESHOLD_DENOMINATORCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        checkSignatures(checkSignaturesCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        increment(incrementCall),
        #[allow(missing_docs)]
        number(numberCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
    }
    #[automatically_derived]
    impl CounterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 31u8, 29u8, 91u8],
            [93u8, 244u8, 89u8, 70u8],
            [94u8, 81u8, 11u8, 96u8],
            [94u8, 139u8, 63u8, 45u8],
            [104u8, 48u8, 72u8, 53u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [131u8, 129u8, 245u8, 138u8],
            [133u8, 4u8, 122u8, 73u8],
            [223u8, 92u8, 247u8, 35u8],
            [239u8, 2u8, 68u8, 88u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CounterCalls {
        const NAME: &'static str = "CounterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BLOCK_STALE_MEASURE(_) => {
                    <BLOCK_STALE_MEASURECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::QUORUM_THRESHOLD(_) => {
                    <QUORUM_THRESHOLDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::THRESHOLD_DENOMINATOR(_) => {
                    <THRESHOLD_DENOMINATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::increment(_) => <incrementCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::number(_) => <numberCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<CounterCalls>] = &[
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(CounterCalls::trySignatureAndApkVerification)
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn QUORUM_THRESHOLD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <QUORUM_THRESHOLDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::QUORUM_THRESHOLD)
                    }
                    QUORUM_THRESHOLD
                },
                {
                    fn BLOCK_STALE_MEASURE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <BLOCK_STALE_MEASURECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::BLOCK_STALE_MEASURE)
                    }
                    BLOCK_STALE_MEASURE
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn number(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <numberCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(CounterCalls::number)
                    }
                    number
                },
                {
                    fn increment(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <incrementCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(CounterCalls::increment)
                    }
                    increment
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(CounterCalls::delegation)
                    }
                    delegation
                },
                {
                    fn THRESHOLD_DENOMINATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterCalls> {
                        <THRESHOLD_DENOMINATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterCalls::THRESHOLD_DENOMINATOR)
                    }
                    THRESHOLD_DENOMINATOR
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BLOCK_STALE_MEASURE(inner) => {
                    <BLOCK_STALE_MEASURECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QUORUM_THRESHOLD(inner) => {
                    <QUORUM_THRESHOLDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::THRESHOLD_DENOMINATOR(inner) => {
                    <THRESHOLD_DENOMINATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::increment(inner) => {
                    <incrementCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::number(inner) => {
                    <numberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BLOCK_STALE_MEASURE(inner) => {
                    <BLOCK_STALE_MEASURECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::QUORUM_THRESHOLD(inner) => {
                    <QUORUM_THRESHOLDCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::THRESHOLD_DENOMINATOR(inner) => {
                    <THRESHOLD_DENOMINATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::increment(inner) => {
                    <incrementCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::number(inner) => {
                    <numberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Counter`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum CounterErrors {
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        FutureBlockNumber(FutureBlockNumber),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputEmptyQuorumNumbers(InputEmptyQuorumNumbers),
        #[allow(missing_docs)]
        InputNonSignerLengthMismatch(InputNonSignerLengthMismatch),
        #[allow(missing_docs)]
        InsufficientQuorumThreshold(InsufficientQuorumThreshold),
        #[allow(missing_docs)]
        InvalidBLSPairingKey(InvalidBLSPairingKey),
        #[allow(missing_docs)]
        InvalidBLSSignature(InvalidBLSSignature),
        #[allow(missing_docs)]
        InvalidHash(InvalidHash),
        #[allow(missing_docs)]
        InvalidQuorumApkHash(InvalidQuorumApkHash),
        #[allow(missing_docs)]
        InvalidReferenceBlocknumber(InvalidReferenceBlocknumber),
        #[allow(missing_docs)]
        NonSignerPubkeysNotSorted(NonSignerPubkeysNotSorted),
        #[allow(missing_docs)]
        OnlyRegistryCoordinatorOwner(OnlyRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
        #[allow(missing_docs)]
        StaleBlockNumber(StaleBlockNumber),
    }
    #[automatically_derived]
    impl CounterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 248u8, 6u8, 224u8],
            [31u8, 4u8, 5u8, 160u8],
            [37u8, 47u8, 138u8, 14u8],
            [48u8, 92u8, 62u8, 147u8],
            [67u8, 113u8, 74u8, 253u8],
            [70u8, 51u8, 190u8, 50u8],
            [75u8, 135u8, 79u8, 69u8],
            [95u8, 131u8, 47u8, 65u8],
            [103u8, 152u8, 141u8, 51u8],
            [109u8, 134u8, 5u8, 219u8],
            [128u8, 200u8, 131u8, 72u8],
            [171u8, 27u8, 35u8, 107u8],
            [202u8, 149u8, 115u8, 51u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [224u8, 225u8, 231u8, 98u8],
            [225u8, 49u8, 10u8, 237u8],
            [251u8, 74u8, 156u8, 142u8],
            [255u8, 113u8, 148u8, 20u8],
            [255u8, 137u8, 212u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CounterErrors {
        const NAME: &'static str = "CounterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::FutureBlockNumber(_) => {
                    <FutureBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputEmptyQuorumNumbers(_) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputNonSignerLengthMismatch(_) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientQuorumThreshold(_) => {
                    <InsufficientQuorumThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSPairingKey(_) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSSignature(_) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHash(_) => <InvalidHash as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidQuorumApkHash(_) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlocknumber(_) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonSignerPubkeysNotSorted(_) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR,
                Self::StaleBlockNumber(_) => {
                    <StaleBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<CounterErrors>] = &[
                {
                    fn InvalidHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InvalidHash as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(CounterErrors::InvalidHash)
                    }
                    InvalidHash
                },
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InputEmptyQuorumNumbers)
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn FutureBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <FutureBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::FutureBlockNumber)
                    }
                    FutureBlockNumber
                },
                {
                    fn StaleBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <StaleBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::StaleBlockNumber)
                    }
                    StaleBlockNumber
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(CounterErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InvalidReferenceBlocknumber)
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InputNonSignerLengthMismatch)
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InsufficientQuorumThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InsufficientQuorumThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InsufficientQuorumThreshold)
                    }
                    InsufficientQuorumThreshold
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(CounterErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(CounterErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::OnlyRegistryCoordinatorOwner)
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::NonSignerPubkeysNotSorted)
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<CounterErrors> {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(CounterErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FutureBlockNumber(inner) => {
                    <FutureBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientQuorumThreshold(inner) => {
                    <InsufficientQuorumThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidHash(inner) => {
                    <InvalidHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StaleBlockNumber(inner) => {
                    <StaleBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FutureBlockNumber(inner) => {
                    <FutureBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientQuorumThreshold(inner) => {
                    <InsufficientQuorumThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidHash(inner) => {
                    <InvalidHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StaleBlockNumber(inner) => {
                    <StaleBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Counter`](self) contract instance.

    See the [wrapper's documentation](`CounterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CounterInstance<T, P, N> {
        CounterInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<CounterInstance<T, P, N>>>
    {
        CounterInstance::<T, P, N>::deploy(provider, _registryCoordinator)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        CounterInstance::<T, P, N>::deploy_builder(provider, _registryCoordinator)
    }
    /**A [`Counter`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`Counter`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CounterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for CounterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CounterInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`Counter`](self) contract instance.

        See the [wrapper's documentation](`CounterInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _registryCoordinator: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<CounterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _registryCoordinator);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _registryCoordinator: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> CounterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CounterInstance<T, P, N> {
            CounterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`BLOCK_STALE_MEASURE`] function.
        pub fn BLOCK_STALE_MEASURE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BLOCK_STALE_MEASURECall, N> {
            self.call_builder(&BLOCK_STALE_MEASURECall {})
        }
        ///Creates a new call builder for the [`QUORUM_THRESHOLD`] function.
        pub fn QUORUM_THRESHOLD(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, QUORUM_THRESHOLDCall, N> {
            self.call_builder(&QUORUM_THRESHOLDCall {})
        }
        ///Creates a new call builder for the [`THRESHOLD_DENOMINATOR`] function.
        pub fn THRESHOLD_DENOMINATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, THRESHOLD_DENOMINATORCall, N> {
            self.call_builder(&THRESHOLD_DENOMINATORCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`increment`] function.
        pub fn increment(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, incrementCall, N> {
            self.call_builder(&incrementCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`number`] function.
        pub fn number(&self) -> alloy_contract::SolCallBuilder<T, &P, numberCall, N> {
            self.call_builder(&numberCall {})
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CounterInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
