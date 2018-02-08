

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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
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
	pub fn new (data : UniqueData) -> (Unique) {
		Unique (StdRc::new (data))
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




impl UniqueData {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_parameter (handle : Handle) -> (UniqueData) {
		UniqueData::from_raw_handle (UniqueKind::ParameterIdentity, handle.value (), 0xeaed97d750aedb9b82fac1e375b185dc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_raw_handle (kind : UniqueKind, handle : u64, fuzz : u128) -> (UniqueData) {
		let fingerprint = (((handle as u128) << 0) | (handle as u128) << 64) ^ fuzz;
		UniqueData::from_raw (kind, fingerprint)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from_raw (kind : UniqueKind, fingerprint : u128) -> (UniqueData) {
		let kind_fingerprint_mask = ! (0xff << 60);
		let kind_fingerprint_value = (unsafe { mem::transmute::<_, u8> (kind) } as u128) << 60;
		let fingerprint = (fingerprint & kind_fingerprint_mask) | kind_fingerprint_value;
		UniqueData {kind, fingerprint}
	}
}

