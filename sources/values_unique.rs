

use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Unique;
	pub use super::UniqueData;
	pub use super::UniqueKind;
	pub use super::UniqueFingerprint;
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Unique ( StdRc<UniqueData> );


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct UniqueData {
	pub kind : UniqueKind,
	pub fingerprint : UniqueFingerprint,
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct UniqueFingerprint ( u128 );


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum UniqueKind {
	
	Null,
	Undefined,
	Void,
	PortEof,
	
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
	
	PathFingerprint,
	PathIdentity,
	
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
	pub fn fingerprint (&self) -> (UniqueFingerprint) {
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
		* StdRc::as_ref (&self.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn data_rc_clone (&self) -> (StdRc<UniqueData>) {
		StdRc::clone (&self.0)
	}
}




impl UniqueData {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_parameter (handle : Handle) -> (UniqueData) {
		UniqueData::for_raw_handle (UniqueKind::ParameterIdentity, handle.value (), UNIQUE_DATA_FUZZ_FOR_PARAMETER)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub const fn for_parameter_builtin (handle : u32) -> (UniqueData) {
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cast_lossless) ) ]
		UniqueData::for_raw_handle (UniqueKind::ParameterIdentity, handle as u64, UNIQUE_DATA_FUZZ_FOR_PARAMETER)
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	const fn for_raw_handle (kind : UniqueKind, handle : u64, fuzz : u128) -> (UniqueData) {
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cast_lossless) ) ]
		UniqueData::for_raw (kind, (((handle as u128) << 64) | (handle as u128)) ^ fuzz)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	const fn for_raw (kind : UniqueKind, fingerprint : u128) -> (UniqueData) {
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cast_lossless) ) ]
		UniqueData {
				kind : kind,
				fingerprint : UniqueFingerprint (
						(fingerprint & (! (0xff << 60)))
						| ((kind as u8 as u128) << 60)),
			}
	}
}




impl UniqueFingerprint {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (&self) -> (u128) {
		self.0
	}
}




const UNIQUE_DATA_FUZZ_FOR_PARAMETER : u128 = 0xeaed97d750aedb9b82fac1e375b185dc;

