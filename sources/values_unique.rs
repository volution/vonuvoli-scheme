

use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Unique;
	pub use super::UniqueData;
	pub use super::UniqueKind;
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Unique ( StdRc<UniqueData> );


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct UniqueData {
	pub kind : UniqueKind,
	pub fingerprint : u128,
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum UniqueKind {
	
	Singleton (ValueSingleton),
	
	Boolean,
	NumberInteger,
	NumberReal,
	Character,
	
	SymbolFingerprint,
	SymbolIdentity,
	
	KeywordFingerprint,
	KeywordIdentity,
	
	StringRegexFingerprint,
	StringRegexIdentity,
	
	StringFingerprint,
	StringImmutableIdentity,
	StringMutableIdentity,
	
	BytesFingerprint,
	BytesImmutableIdentity,
	BytesMutableIdentity,
	
	PairValuesFingerprintFingerprint,
	PairValuesIdentityFingerprint,
	PairImmutableIdentity,
	PairMutableIdentity,
	
	ArrayValuesFingerprintFingerprint,
	ArrayValuesIdentityFingerprint,
	ArrayImmutableIdentity,
	ArrayMutableIdentity,
	
	ValuesValuesFingerprintFingerprint,
	ValuesValuesIdentityFingerprint,
	ValuesIdentity,
	
	RecordKindIdentity,
	RecordValuesFingerprintFingerprint,
	RecordValuesIdentityFingerprint,
	RecordImmutableIdentity,
	RecordMutableIdentity,
	
	ErrorIdentity,
	
	ProcedurePrimitiveIdentity,
	ProcedureExtendedIdentity,
	ProcedureNativeIdentity,
	ProcedureLambdaIdentity,
	
	SyntaxPrimitiveIdentity,
	SyntaxExtendedIdentity,
	SyntaxNativeIdentity,
	SyntaxLambdaIdentity,
	
	PortIdentity,
	ProcessIdentity,
	
	ContextIdentity,
	BindingIdentity,
	ParametersIdentity,
	ParameterIdentity,
	PromiseIdentity,
	
}




impl Unique {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_parameter (handle : Handle) -> (Unique) {
		Unique::from_raw_handle (UniqueKind::ParameterIdentity, 0xeaed97d750aedb9b82fac1e375b185dc, handle)
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (UniqueKind) {
		let self_0 = self.data_ref ();
		self_0.kind
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fingerprint (&self) -> (u128) {
		let self_0 = self.data_ref ();
		self_0.fingerprint
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from_raw_handle (kind : UniqueKind, fingerprint_base : u128, handle : Handle) -> (Unique) {
		let mut fingerprint = fingerprint_base;
		fingerprint ^= (handle.value () as u128) << 0;
		fingerprint ^= (handle.value () as u128) << 32;
		fingerprint ^= (handle.value () as u128) << 64;
		fingerprint ^= (handle.value () as u128) << 96;
		Unique::from_raw (kind, fingerprint)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from_raw (kind : UniqueKind, fingerprint : u128) -> (Unique) {
		Unique::from_data (UniqueData {kind, fingerprint})
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_data (data : UniqueData) -> (Unique) {
		Unique::from_rc (StdRc::new (data))
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<UniqueData>) -> (Unique) {
		Unique (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<UniqueData>) -> (Unique) {
		Unique::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Unique) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || StdRc::eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn data_ref (&self) -> (&UniqueData) {
		StdRc::as_ref (&self.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn data_clone (&self) -> (UniqueData) {
		StdRc::as_ref (&self.0) .clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn data_rc_clone (&self) -> (StdRc<UniqueData>) {
		StdRc::clone (&self.0)
	}
}

