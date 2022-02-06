

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::FileSystemPrimitive0;
	pub use super::FileSystemPrimitive1;
	pub use super::FileSystemPrimitive2;
	pub use super::FileSystemPrimitive3;
	pub use super::FileSystemPrimitive4;
	pub use super::FileSystemPrimitive5;
	pub use super::FileSystemPrimitiveN;
	pub use super::FileSystemPrimitiveV;
	
	pub use super::filesystem_primitive_0_evaluate;
	pub use super::filesystem_primitive_1_evaluate;
	pub use super::filesystem_primitive_2_evaluate;
	pub use super::filesystem_primitive_3_evaluate;
	pub use super::filesystem_primitive_4_evaluate;
	pub use super::filesystem_primitive_5_evaluate;
	pub use super::filesystem_primitive_n_evaluate;
	
	pub use super::filesystem_primitive_v_alternative_0;
	pub use super::filesystem_primitive_v_alternative_1;
	pub use super::filesystem_primitive_v_alternative_2;
	pub use super::filesystem_primitive_v_alternative_3;
	pub use super::filesystem_primitive_v_alternative_4;
	pub use super::filesystem_primitive_v_alternative_5;
	pub use super::filesystem_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::filesystem_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (FileSystemPrimitive0, (procedure, 0), {
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateFile,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateDirectory,
	
});


def_primitives_enum! (FileSystemPrimitive1, (procedure, 1), {
	
	AnyExists,
	FileExists,
	FileDelete,
	DirectoryExists,
	DirectoryDelete,
	SymLinkExists,
	MountPointIs,
	
	DirectoryListAsList,
	DirectoryListAsArray,
	
	MetadataResolve,
	
	MetadataKindGet,
	MetadataKindIsFile,
	MetadataKindIsDirectory,
	MetadataKindIsSymLink,
	MetadataKindIsFifo,
	MetadataKindIsSocket,
	MetadataKindIsBlockDevice,
	MetadataKindIsCharacterDevice,
	
	MetadataFileGetSize,
	MetadataFileIsEmpty,
	MetadataFileIsEmptyNot,
	
	MetadataIsReadonly,
	MetadataIsReadable,
	MetadataIsWriteable,
	MetadataFileIsExecutable,
	MetadataDirectoryIsTraversable,
	
	MetadataUnixGetMode,
	MetadataUnixGetType,
	MetadataUnixGetPermissions,
	MetadataUnixGetUserIdentifier,
	MetadataUnixGetGroupIdentifier,
	MetadataUnixGetDataAccessedAt,
	MetadataUnixGetDataModifiedAt,
	MetadataUnixGetInodeChangedAt,
	MetadataUnixGetInodeDevice,
	MetadataUnixGetInodeNumber,
	MetadataUnixGetInodeLinks,
	
	SymLinkResolve,
	PathCanonicalize,
	
	PathCoerce,
	PathParent,
	PathJoin,
	PathSplit,
	
	PathName,
	PathNameWithoutExtension,
	PathNameOnlyExtension,
	PathNameJoin,
	PathNameSplit,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	PathToString,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringToPath,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	PathToBytes,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesToPath,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateFile,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateDirectory,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryRelease,
	
});


def_primitives_enum! (FileSystemPrimitive2, (procedure, 2), {
	
	AnyExists,
	FileExists,
	DirectoryExists,
	MountPointIs,
	
	DirectoryListAsList,
	DirectoryListAsArray,
	
	PathJoin,
	PathSplit,
	PathHasPrefix,
	PathHasSuffix,
	
	PathNameJoin,
	PathNameSplit,
	PathNameIs,
	PathNameHasPrefix,
	PathNameHasSuffix,
	
	MetadataResolve,
	MetadataIsSelf,
	
	MetadataKindGet,
	MetadataKindHas,
	
	MetadataFileGetSize,
	MetadataFileIsEmpty,
	MetadataFileIsEmptyNot,
	
	MetadataIsReadonly,
	MetadataIsReadable,
	MetadataIsWriteable,
	MetadataFileIsExecutable,
	MetadataDirectoryIsTraversable,
	
	MetadataUnixGetMode,
	MetadataUnixGetType,
	MetadataUnixGetPermissions,
	MetadataUnixGetUserIdentifier,
	MetadataUnixGetGroupIdentifier,
	MetadataUnixGetDataAccessedAt,
	MetadataUnixGetDataModifiedAt,
	MetadataUnixGetInodeChangedAt,
	MetadataUnixGetInodeDevice,
	MetadataUnixGetInodeNumber,
	MetadataUnixGetInodeLinks,
	
	SymLinkResolve,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateFile,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateDirectory,
	
});


def_primitives_enum! (FileSystemPrimitive3, (procedure, 3), {
	
	DirectoryListAsList,
	DirectoryListAsArray,
	DirectoryListFold,
	DirectoryListFoldRecursive,
	
	PathJoin,
	PathNameJoin,
	
	MetadataIsSelf,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateFile,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateDirectory,
	
});


def_primitives_enum! (FileSystemPrimitive4, (procedure, 4), {
	
	DirectoryListAsList,
	DirectoryListAsArray,
	DirectoryListFold,
	DirectoryListFoldRecursive,
	
	PathJoin,
	PathNameJoin,
	
});


def_primitives_enum! (FileSystemPrimitive5, (procedure, 5), {
	
	DirectoryListAsList,
	DirectoryListAsArray,
	DirectoryListFold,
	DirectoryListFoldRecursive,
	
	PathJoin,
	PathNameJoin,
	
});


def_primitives_enum! (FileSystemPrimitiveN, (procedure, n), {
	
	DirectoryListAsList,
	DirectoryListAsArray,
	DirectoryListFold,
	DirectoryListFoldRecursive,
	
	PathJoin,
	PathNameJoin,
	
});


def_primitives_enum! (FileSystemPrimitiveV, (procedure, v), {
	
	AnyExists,
	FileExists,
	DirectoryExists,
	MountPointIs,
	
	DirectoryListAsList,
	DirectoryListAsArray,
	DirectoryListFold,
	DirectoryListFoldRecursive,
	
	PathJoin,
	PathSplit,
	PathNameJoin,
	PathNameSplit,
	
	MetadataResolve,
	MetadataIsSelf,
	
	MetadataKindGet,
	
	MetadataFileGetSize,
	MetadataFileIsEmpty,
	MetadataFileIsEmptyNot,
	
	MetadataIsReadonly,
	MetadataIsReadable,
	MetadataIsWriteable,
	MetadataFileIsExecutable,
	MetadataDirectoryIsTraversable,
	
	MetadataUnixGetMode,
	MetadataUnixGetType,
	MetadataUnixGetPermissions,
	MetadataUnixGetUserIdentifier,
	MetadataUnixGetGroupIdentifier,
	MetadataUnixGetDataAccessedAt,
	MetadataUnixGetDataModifiedAt,
	MetadataUnixGetInodeChangedAt,
	MetadataUnixGetInodeDevice,
	MetadataUnixGetInodeNumber,
	MetadataUnixGetInodeLinks,
	
	SymLinkResolve,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateFile,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	TemporaryCreateDirectory,
	
});




impl_procedure_primitive_enum_matrix! (
		(FileSystemPrimitive0, filesystem_primitive_0_evaluate, filesystem_primitive_0_attributes, filesystem_primitive_v_alternative_0),
		(FileSystemPrimitive1, filesystem_primitive_1_evaluate, filesystem_primitive_1_attributes, filesystem_primitive_v_alternative_1),
		(FileSystemPrimitive2, filesystem_primitive_2_evaluate, filesystem_primitive_2_attributes, filesystem_primitive_v_alternative_2),
		(FileSystemPrimitive3, filesystem_primitive_3_evaluate, filesystem_primitive_3_attributes, filesystem_primitive_v_alternative_3),
		(FileSystemPrimitive4, filesystem_primitive_4_evaluate, filesystem_primitive_4_attributes, filesystem_primitive_v_alternative_4),
		(FileSystemPrimitive5, filesystem_primitive_5_evaluate, filesystem_primitive_5_attributes, filesystem_primitive_v_alternative_5),
		(FileSystemPrimitiveN, filesystem_primitive_n_evaluate, filesystem_primitive_n_attributes, filesystem_primitive_v_alternative_n),
		(FileSystemPrimitiveV, filesystem_primitive_v_evaluate, filesystem_primitive_v_attributes),
	);




pub fn filesystem_primitive_0_evaluate (primitive : FileSystemPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive0::TemporaryCreateFile =>
			return filesystem_temporary_create_file (None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive0::TemporaryCreateDirectory =>
			return filesystem_temporary_create_directory (None, None, None) .into_0 () .into_0 (),
		
	}
}




pub fn filesystem_primitive_1_evaluate (primitive : FileSystemPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive1::AnyExists =>
			return filesystem_any_exists (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::FileExists =>
			return filesystem_file_exists (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::FileDelete =>
			return filesystem_file_delete (input_1) .into_0 (),
		
		FileSystemPrimitive1::DirectoryExists =>
			return filesystem_directory_exists (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::DirectoryDelete =>
			return filesystem_directory_delete (input_1) .into_0 (),
		
		FileSystemPrimitive1::SymLinkExists =>
			return filesystem_symlink_exists (input_1) .into_0 (),
		
		FileSystemPrimitive1::MountPointIs =>
			return filesystem_mountpoint_is (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::DirectoryListAsList =>
			return filesystem_directory_list (input_1, false, false, false, true, true, false, None),
		
		FileSystemPrimitive1::DirectoryListAsArray =>
			return filesystem_directory_list (input_1, false, false, false, true, true, true, None),
		
		FileSystemPrimitive1::MetadataResolve =>
			return filesystem_metadata_resolve (input_1, true),
		
		FileSystemPrimitive1::MetadataKindGet =>
			return filesystem_metadata_get_kind_symbol (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsFile =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::File, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsDirectory =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::Directory, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsSymLink =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::SymLink, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsFifo =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::Fifo, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsSocket =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::Socket, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsBlockDevice =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::BlockDevice, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataKindIsCharacterDevice =>
			return filesystem_metadata_has_kind (input_1, FileSystemMetadataKind::CharacterDevice, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataFileGetSize =>
			return filesystem_metadata_file_get_size (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataFileIsEmpty =>
			return filesystem_metadata_file_is_empty (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataFileIsEmptyNot =>
			return filesystem_metadata_file_is_not_empty (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataIsReadonly =>
			return filesystem_metadata_is_readonly (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataIsReadable =>
			return filesystem_metadata_is_readable (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataIsWriteable =>
			return filesystem_metadata_is_writeable (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataFileIsExecutable =>
			return filesystem_metadata_file_is_executable (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataDirectoryIsTraversable =>
			return filesystem_metadata_directory_is_traversable (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetMode =>
			return filesystem_metadata_unix_get_mode (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetType =>
			return filesystem_metadata_unix_get_type (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetPermissions =>
			return filesystem_metadata_unix_get_permissions (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetUserIdentifier =>
			return filesystem_metadata_unix_get_user_identifier (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetGroupIdentifier =>
			return filesystem_metadata_unix_get_group_identifier (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetDataAccessedAt =>
			return filesystem_metadata_unix_get_data_accessed_at (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetDataModifiedAt =>
			return filesystem_metadata_unix_get_data_modified_at (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetInodeChangedAt =>
			return filesystem_metadata_unix_get_inode_changed_at (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetInodeDevice =>
			return filesystem_metadata_unix_get_inode_device (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetInodeNumber =>
			return filesystem_metadata_unix_get_inode_number (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::MetadataUnixGetInodeLinks =>
			return filesystem_metadata_unix_get_inode_links (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::SymLinkResolve =>
			return filesystem_symlink_resolve (input_1, false, false),
		
		FileSystemPrimitive1::PathCanonicalize =>
			return filesystem_path_canonicalize (input_1),
		
		FileSystemPrimitive1::PathCoerce =>
			return filesystem_path_coerce (input_1, true) .into_0 (),
		
		FileSystemPrimitive1::PathParent =>
			return filesystem_path_parent (input_1),
		
		FileSystemPrimitive1::PathJoin =>
			return filesystem_path_join (&[input_1], true) .into_0 (),
		
		FileSystemPrimitive1::PathSplit =>
			return filesystem_path_split (input_1, false, None) .into_0 (),
		
		FileSystemPrimitive1::PathName =>
			return filesystem_path_name (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameWithoutExtension =>
			return filesystem_path_name_without_extension (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameOnlyExtension =>
			return filesystem_path_name_only_extension (input_1) .into_0 (),
		
		FileSystemPrimitive1::PathNameJoin =>
			return filesystem_path_name_join (&[input_1]) .into_0 (),
		
		FileSystemPrimitive1::PathNameSplit =>
			return filesystem_path_name_split (input_1, false, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FileSystemPrimitive1::PathToString =>
			return filesystem_path_to_string (input_1, false),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		FileSystemPrimitive1::StringToPath =>
			return filesystem_string_to_path (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FileSystemPrimitive1::PathToBytes =>
			return filesystem_path_to_bytes (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		FileSystemPrimitive1::BytesToPath =>
			return filesystem_bytes_to_path (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive1::TemporaryCreateFile =>
			return filesystem_temporary_create_file (Some (input_1), None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive1::TemporaryCreateDirectory =>
			return filesystem_temporary_create_directory (Some (input_1), None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive1::TemporaryRelease =>
			return filesystem_temporary_release (input_1) .into_0 (),
		
	}
}




pub fn filesystem_primitive_2_evaluate (primitive : FileSystemPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive2::AnyExists =>
			return filesystem_any_exists (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::FileExists =>
			return filesystem_file_exists (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::DirectoryExists =>
			return filesystem_directory_exists (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MountPointIs =>
			return filesystem_mountpoint_is (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::DirectoryListAsList =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), false, false, true, true, false, None),
		
		FileSystemPrimitive2::DirectoryListAsArray =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), false, false, true, true, true, None),
		
		FileSystemPrimitive2::PathJoin =>
			return filesystem_path_join (&[input_1, input_2], true) .into_0 (),
		
		FileSystemPrimitive2::PathSplit =>
			return filesystem_path_split (input_1, r#try! (boolean_coerce (input_2)), None) .into_0 (),
		
		FileSystemPrimitive2::PathHasPrefix =>
			return filesystem_path_has_prefix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathHasSuffix =>
			return filesystem_path_has_suffix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2]) .into_0 (),
		
		FileSystemPrimitive2::PathNameSplit =>
			return filesystem_path_name_split (input_1, r#try! (boolean_coerce (input_2)), None) .into_0 (),
		
		FileSystemPrimitive2::MetadataResolve =>
			return filesystem_metadata_resolve (input_1, r#try! (boolean_coerce (input_2))),
		
		FileSystemPrimitive2::MetadataIsSelf =>
			return filesystem_metadata_is_self (input_1, input_2, true) .into_0 (),
		
		FileSystemPrimitive2::MetadataKindGet =>
			return filesystem_metadata_get_kind_symbol (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataKindHas =>
			return filesystem_metadata_has_kind_symbol (input_1, input_2, false) .into_0 (),
		
		FileSystemPrimitive2::MetadataFileGetSize =>
			return filesystem_metadata_file_get_size (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataFileIsEmpty =>
			return filesystem_metadata_file_is_empty (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataFileIsEmptyNot =>
			return filesystem_metadata_file_is_not_empty (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataIsReadonly =>
			return filesystem_metadata_is_readonly (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataIsReadable =>
			return filesystem_metadata_is_readable (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataIsWriteable =>
			return filesystem_metadata_is_writeable (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataFileIsExecutable =>
			return filesystem_metadata_file_is_executable (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataDirectoryIsTraversable =>
			return filesystem_metadata_directory_is_traversable (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetMode =>
			return filesystem_metadata_unix_get_mode (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetType =>
			return filesystem_metadata_unix_get_type (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetPermissions =>
			return filesystem_metadata_unix_get_permissions (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetUserIdentifier =>
			return filesystem_metadata_unix_get_user_identifier (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetGroupIdentifier =>
			return filesystem_metadata_unix_get_group_identifier (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetDataAccessedAt =>
			return filesystem_metadata_unix_get_data_accessed_at (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetDataModifiedAt =>
			return filesystem_metadata_unix_get_data_modified_at (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetInodeChangedAt =>
			return filesystem_metadata_unix_get_inode_changed_at (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetInodeDevice =>
			return filesystem_metadata_unix_get_inode_device (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetInodeNumber =>
			return filesystem_metadata_unix_get_inode_number (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::MetadataUnixGetInodeLinks =>
			return filesystem_metadata_unix_get_inode_links (input_1, r#try! (boolean_coerce (input_2))) .into_0 (),
		
		FileSystemPrimitive2::SymLinkResolve =>
			return filesystem_symlink_resolve (input_1, r#try! (boolean_coerce (input_2)), false),
		
		FileSystemPrimitive2::PathNameIs =>
			return filesystem_path_name_is (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameHasPrefix =>
			return filesystem_path_name_has_prefix (input_1, input_2) .into_0 (),
		
		FileSystemPrimitive2::PathNameHasSuffix =>
			return filesystem_path_name_has_suffix (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive2::TemporaryCreateFile =>
			return filesystem_temporary_create_file (Some (input_1), Some (input_2), None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive2::TemporaryCreateDirectory =>
			return filesystem_temporary_create_directory (Some (input_1), Some (input_2), None) .into_0 () .into_0 (),
		
	}
}




pub fn filesystem_primitive_3_evaluate (primitive : FileSystemPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive3::DirectoryListAsList =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), false, true, true, false, None),
		
		FileSystemPrimitive3::DirectoryListAsArray =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), false, true, true, true, None),
		
		FileSystemPrimitive3::DirectoryListFold =>
			return filesystem_directory_fold (input_1, input_2, input_3, false, false, false, true, evaluator),
		
		FileSystemPrimitive3::DirectoryListFoldRecursive =>
			return filesystem_directory_fold_recursive (input_1, input_2, &FALSE_VALUE, input_3, false, false, false, true, evaluator),
		
		FileSystemPrimitive3::MetadataIsSelf =>
			return filesystem_metadata_is_self (input_1, input_2, r#try! (boolean_coerce (input_3))) .into_0 (),
		
		FileSystemPrimitive3::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3], true) .into_0 (),
		
		FileSystemPrimitive3::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3]) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive3::TemporaryCreateFile =>
			return filesystem_temporary_create_file (Some (input_1), Some (input_2), Some (input_3)) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitive3::TemporaryCreateDirectory =>
			return filesystem_temporary_create_directory (Some (input_1), Some (input_2), Some (input_3)) .into_0 () .into_0 (),
		
	}
}




pub fn filesystem_primitive_4_evaluate (primitive : FileSystemPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive4::DirectoryListAsList =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), r#try! (boolean_coerce (input_4)), true, true, false, None),
		
		FileSystemPrimitive4::DirectoryListAsArray =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), r#try! (boolean_coerce (input_4)), true, true, true, None),
		
		FileSystemPrimitive4::DirectoryListFold =>
			return filesystem_directory_fold (input_1, input_2, input_3, r#try! (boolean_coerce (input_4)), false, false, true, evaluator),
		
		FileSystemPrimitive4::DirectoryListFoldRecursive =>
			return filesystem_directory_fold_recursive (input_1, input_2, input_3, input_4, false, false, false, true, evaluator),
		
		FileSystemPrimitive4::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3, input_4], true) .into_0 (),
		
		FileSystemPrimitive4::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3, input_4]) .into_0 (),
		
	}
}




pub fn filesystem_primitive_5_evaluate (primitive : FileSystemPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitive5::DirectoryListAsList =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), r#try! (boolean_coerce (input_4)), r#try! (boolean_coerce (input_5)), true, false, None),
		
		FileSystemPrimitive5::DirectoryListAsArray =>
			return filesystem_directory_list (input_1, r#try! (boolean_coerce (input_2)), r#try! (boolean_coerce (input_3)), r#try! (boolean_coerce (input_4)), r#try! (boolean_coerce (input_5)), true, true, None),
		
		FileSystemPrimitive5::DirectoryListFold =>
			return filesystem_directory_fold (input_1, input_2, input_3, r#try! (boolean_coerce (input_4)), r#try! (boolean_coerce (input_5)), false, true, evaluator),
		
		FileSystemPrimitive5::DirectoryListFoldRecursive =>
			return filesystem_directory_fold_recursive (input_1, input_2, input_3, input_4, r#try! (boolean_coerce (input_5)), false, false, true, evaluator),
		
		FileSystemPrimitive5::PathJoin =>
			return filesystem_path_join (&[input_1, input_2, input_3, input_4, input_5], true) .into_0 (),
		
		FileSystemPrimitive5::PathNameJoin =>
			return filesystem_path_name_join (&[input_1, input_2, input_3, input_4, input_5]) .into_0 (),
		
	}
}




pub fn filesystem_primitive_n_evaluate (primitive : FileSystemPrimitiveN, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		FileSystemPrimitiveN::DirectoryListAsList =>
			match inputs.len () {
				6 =>
					return filesystem_directory_list (inputs[0].as_ref (), r#try! (boolean_coerce (inputs[1].as_ref ())), r#try! (boolean_coerce (inputs[2].as_ref ())), r#try! (boolean_coerce (inputs[3].as_ref ())), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), false, None),
				_ =>
					fail! (0xecd0caf3),
			},
		
		FileSystemPrimitiveN::DirectoryListAsArray =>
			match inputs.len () {
				6 =>
					return filesystem_directory_list (inputs[0].as_ref (), r#try! (boolean_coerce (inputs[1].as_ref ())), r#try! (boolean_coerce (inputs[2].as_ref ())), r#try! (boolean_coerce (inputs[3].as_ref ())), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), true, None),
				_ =>
					fail! (0x33a0d15b),
			},
		
		FileSystemPrimitiveN::DirectoryListFold =>
			match inputs.len () {
				6 =>
					return filesystem_directory_fold (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), r#try! (boolean_coerce (inputs[3].as_ref ())), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), true, evaluator),
				7 =>
					return filesystem_directory_fold (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), r#try! (boolean_coerce (inputs[3].as_ref ())), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), r#try! (boolean_coerce (inputs[6].as_ref ())), evaluator),
				_ =>
					fail! (0x1e69d076),
			},
		
		FileSystemPrimitiveN::DirectoryListFoldRecursive =>
			match inputs.len () {
				6 =>
					return filesystem_directory_fold_recursive (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), false, true, evaluator),
				7 =>
					return filesystem_directory_fold_recursive (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), r#try! (boolean_coerce (inputs[6].as_ref ())), true, evaluator),
				8 =>
					return filesystem_directory_fold_recursive (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), r#try! (boolean_coerce (inputs[4].as_ref ())), r#try! (boolean_coerce (inputs[5].as_ref ())), r#try! (boolean_coerce (inputs[6].as_ref ())), r#try! (boolean_coerce (inputs[7].as_ref ())), evaluator),
				_ =>
					fail! (0x8cc36965),
			},
		
		FileSystemPrimitiveN::PathJoin =>
			return filesystem_path_join (inputs, true) .into_0 (),
		
		FileSystemPrimitiveN::PathNameJoin =>
			return filesystem_path_name_join (inputs) .into_0 (),
		
	}
}




pub fn filesystem_primitive_v_alternative_0 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive0>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			None,
		FileSystemPrimitiveV::FileExists =>
			None,
		FileSystemPrimitiveV::DirectoryExists =>
			None,
		FileSystemPrimitiveV::MountPointIs =>
			None,
		FileSystemPrimitiveV::DirectoryListAsList =>
			None,
		FileSystemPrimitiveV::DirectoryListAsArray =>
			None,
		FileSystemPrimitiveV::DirectoryListFold =>
			None,
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			None,
		FileSystemPrimitiveV::PathJoin =>
			None,
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			None,
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::MetadataResolve =>
			None,
		FileSystemPrimitiveV::MetadataIsSelf =>
			None,
		FileSystemPrimitiveV::MetadataKindGet =>
			None,
		FileSystemPrimitiveV::MetadataFileGetSize =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			None,
		FileSystemPrimitiveV::MetadataIsReadonly =>
			None,
		FileSystemPrimitiveV::MetadataIsReadable =>
			None,
		FileSystemPrimitiveV::MetadataIsWriteable =>
			None,
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			None,
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetType =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			None,
		FileSystemPrimitiveV::SymLinkResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			Some (FileSystemPrimitive0::TemporaryCreateFile),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			Some (FileSystemPrimitive0::TemporaryCreateDirectory),
	}
}




pub fn filesystem_primitive_v_alternative_1 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive1>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			Some (FileSystemPrimitive1::AnyExists),
		FileSystemPrimitiveV::FileExists =>
			Some (FileSystemPrimitive1::FileExists),
		FileSystemPrimitiveV::DirectoryExists =>
			Some (FileSystemPrimitive1::DirectoryExists),
		FileSystemPrimitiveV::MountPointIs =>
			Some (FileSystemPrimitive1::MountPointIs),
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitive1::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitive1::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			None,
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			None,
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive1::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			Some (FileSystemPrimitive1::PathSplit),
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive1::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			Some (FileSystemPrimitive1::PathNameSplit),
		FileSystemPrimitiveV::MetadataResolve =>
			Some (FileSystemPrimitive1::MetadataResolve),
		FileSystemPrimitiveV::MetadataIsSelf =>
			None,
		FileSystemPrimitiveV::MetadataKindGet =>
			Some (FileSystemPrimitive1::MetadataKindGet),
		FileSystemPrimitiveV::MetadataFileGetSize =>
			Some (FileSystemPrimitive1::MetadataFileGetSize),
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			Some (FileSystemPrimitive1::MetadataFileIsEmpty),
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			Some (FileSystemPrimitive1::MetadataFileIsEmptyNot),
		FileSystemPrimitiveV::MetadataIsReadonly =>
			Some (FileSystemPrimitive1::MetadataIsReadonly),
		FileSystemPrimitiveV::MetadataIsReadable =>
			Some (FileSystemPrimitive1::MetadataIsReadable),
		FileSystemPrimitiveV::MetadataIsWriteable =>
			Some (FileSystemPrimitive1::MetadataIsWriteable),
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			Some (FileSystemPrimitive1::MetadataFileIsExecutable),
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			Some (FileSystemPrimitive1::MetadataDirectoryIsTraversable),
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			Some (FileSystemPrimitive1::MetadataUnixGetMode),
		FileSystemPrimitiveV::MetadataUnixGetType =>
			Some (FileSystemPrimitive1::MetadataUnixGetType),
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			Some (FileSystemPrimitive1::MetadataUnixGetPermissions),
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			Some (FileSystemPrimitive1::MetadataUnixGetUserIdentifier),
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			Some (FileSystemPrimitive1::MetadataUnixGetGroupIdentifier),
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			Some (FileSystemPrimitive1::MetadataUnixGetDataAccessedAt),
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			Some (FileSystemPrimitive1::MetadataUnixGetDataModifiedAt),
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			Some (FileSystemPrimitive1::MetadataUnixGetInodeChangedAt),
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			Some (FileSystemPrimitive1::MetadataUnixGetInodeDevice),
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			Some (FileSystemPrimitive1::MetadataUnixGetInodeNumber),
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			Some (FileSystemPrimitive1::MetadataUnixGetInodeLinks),
		FileSystemPrimitiveV::SymLinkResolve =>
			Some (FileSystemPrimitive1::SymLinkResolve),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			Some (FileSystemPrimitive1::TemporaryCreateFile),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			Some (FileSystemPrimitive1::TemporaryCreateDirectory),
	}
}




pub fn filesystem_primitive_v_alternative_2 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive2>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			Some (FileSystemPrimitive2::AnyExists),
		FileSystemPrimitiveV::FileExists =>
			Some (FileSystemPrimitive2::FileExists),
		FileSystemPrimitiveV::DirectoryExists =>
			Some (FileSystemPrimitive2::DirectoryExists),
		FileSystemPrimitiveV::MountPointIs =>
			Some (FileSystemPrimitive2::MountPointIs),
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitive2::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitive2::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			None,
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			None,
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive2::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			Some (FileSystemPrimitive2::PathSplit),
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive2::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			Some (FileSystemPrimitive2::PathNameSplit),
		FileSystemPrimitiveV::MetadataResolve =>
			Some (FileSystemPrimitive2::MetadataResolve),
		FileSystemPrimitiveV::MetadataIsSelf =>
			Some (FileSystemPrimitive2::MetadataIsSelf),
		FileSystemPrimitiveV::MetadataKindGet =>
			Some (FileSystemPrimitive2::MetadataKindGet),
		FileSystemPrimitiveV::MetadataFileGetSize =>
			Some (FileSystemPrimitive2::MetadataFileGetSize),
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			Some (FileSystemPrimitive2::MetadataFileIsEmpty),
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			Some (FileSystemPrimitive2::MetadataFileIsEmptyNot),
		FileSystemPrimitiveV::MetadataIsReadonly =>
			Some (FileSystemPrimitive2::MetadataIsReadonly),
		FileSystemPrimitiveV::MetadataIsReadable =>
			Some (FileSystemPrimitive2::MetadataIsReadable),
		FileSystemPrimitiveV::MetadataIsWriteable =>
			Some (FileSystemPrimitive2::MetadataIsWriteable),
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			Some (FileSystemPrimitive2::MetadataFileIsExecutable),
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			Some (FileSystemPrimitive2::MetadataDirectoryIsTraversable),
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			Some (FileSystemPrimitive2::MetadataUnixGetMode),
		FileSystemPrimitiveV::MetadataUnixGetType =>
			Some (FileSystemPrimitive2::MetadataUnixGetType),
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			Some (FileSystemPrimitive2::MetadataUnixGetPermissions),
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			Some (FileSystemPrimitive2::MetadataUnixGetUserIdentifier),
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			Some (FileSystemPrimitive2::MetadataUnixGetGroupIdentifier),
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			Some (FileSystemPrimitive2::MetadataUnixGetDataAccessedAt),
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			Some (FileSystemPrimitive2::MetadataUnixGetDataModifiedAt),
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			Some (FileSystemPrimitive2::MetadataUnixGetInodeChangedAt),
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			Some (FileSystemPrimitive2::MetadataUnixGetInodeDevice),
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			Some (FileSystemPrimitive2::MetadataUnixGetInodeNumber),
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			Some (FileSystemPrimitive2::MetadataUnixGetInodeLinks),
		FileSystemPrimitiveV::SymLinkResolve =>
			Some (FileSystemPrimitive2::SymLinkResolve),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			Some (FileSystemPrimitive2::TemporaryCreateFile),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			Some (FileSystemPrimitive2::TemporaryCreateDirectory),
	}
}




pub fn filesystem_primitive_v_alternative_3 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive3>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			None,
		FileSystemPrimitiveV::FileExists =>
			None,
		FileSystemPrimitiveV::DirectoryExists =>
			None,
		FileSystemPrimitiveV::MountPointIs =>
			None,
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitive3::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitive3::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			Some (FileSystemPrimitive3::DirectoryListFold),
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			Some (FileSystemPrimitive3::DirectoryListFoldRecursive),
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive3::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive3::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::MetadataResolve =>
			None,
		FileSystemPrimitiveV::MetadataIsSelf =>
			Some (FileSystemPrimitive3::MetadataIsSelf),
		FileSystemPrimitiveV::MetadataKindGet =>
			None,
		FileSystemPrimitiveV::MetadataFileGetSize =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			None,
		FileSystemPrimitiveV::MetadataIsReadonly =>
			None,
		FileSystemPrimitiveV::MetadataIsReadable =>
			None,
		FileSystemPrimitiveV::MetadataIsWriteable =>
			None,
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			None,
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetType =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			None,
		FileSystemPrimitiveV::SymLinkResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			Some (FileSystemPrimitive3::TemporaryCreateFile),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			Some (FileSystemPrimitive3::TemporaryCreateDirectory),
	}
}




pub fn filesystem_primitive_v_alternative_4 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive4>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			None,
		FileSystemPrimitiveV::FileExists =>
			None,
		FileSystemPrimitiveV::DirectoryExists =>
			None,
		FileSystemPrimitiveV::MountPointIs =>
			None,
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitive4::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitive4::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			Some (FileSystemPrimitive4::DirectoryListFold),
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			Some (FileSystemPrimitive4::DirectoryListFoldRecursive),
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive4::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive4::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::MetadataResolve =>
			None,
		FileSystemPrimitiveV::MetadataIsSelf =>
			None,
		FileSystemPrimitiveV::MetadataKindGet =>
			None,
		FileSystemPrimitiveV::MetadataFileGetSize =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			None,
		FileSystemPrimitiveV::MetadataIsReadonly =>
			None,
		FileSystemPrimitiveV::MetadataIsReadable =>
			None,
		FileSystemPrimitiveV::MetadataIsWriteable =>
			None,
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			None,
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetType =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			None,
		FileSystemPrimitiveV::SymLinkResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			None,
	}
}




pub fn filesystem_primitive_v_alternative_5 (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitive5>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			None,
		FileSystemPrimitiveV::FileExists =>
			None,
		FileSystemPrimitiveV::DirectoryExists =>
			None,
		FileSystemPrimitiveV::MountPointIs =>
			None,
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitive5::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitive5::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			Some (FileSystemPrimitive5::DirectoryListFold),
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			Some (FileSystemPrimitive5::DirectoryListFoldRecursive),
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitive5::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitive5::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::MetadataResolve =>
			None,
		FileSystemPrimitiveV::MetadataIsSelf =>
			None,
		FileSystemPrimitiveV::MetadataKindGet =>
			None,
		FileSystemPrimitiveV::MetadataFileGetSize =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			None,
		FileSystemPrimitiveV::MetadataIsReadonly =>
			None,
		FileSystemPrimitiveV::MetadataIsReadable =>
			None,
		FileSystemPrimitiveV::MetadataIsWriteable =>
			None,
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			None,
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetType =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			None,
		FileSystemPrimitiveV::SymLinkResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			None,
	}
}




pub fn filesystem_primitive_v_alternative_n (primitive : FileSystemPrimitiveV) -> (Option<FileSystemPrimitiveN>) {
	match primitive {
		FileSystemPrimitiveV::AnyExists =>
			None,
		FileSystemPrimitiveV::FileExists =>
			None,
		FileSystemPrimitiveV::DirectoryExists =>
			None,
		FileSystemPrimitiveV::MountPointIs =>
			None,
		FileSystemPrimitiveV::DirectoryListAsList =>
			Some (FileSystemPrimitiveN::DirectoryListAsList),
		FileSystemPrimitiveV::DirectoryListAsArray =>
			Some (FileSystemPrimitiveN::DirectoryListAsArray),
		FileSystemPrimitiveV::DirectoryListFold =>
			Some (FileSystemPrimitiveN::DirectoryListFold),
		FileSystemPrimitiveV::DirectoryListFoldRecursive =>
			Some (FileSystemPrimitiveN::DirectoryListFoldRecursive),
		FileSystemPrimitiveV::PathJoin =>
			Some (FileSystemPrimitiveN::PathJoin),
		FileSystemPrimitiveV::PathSplit =>
			None,
		FileSystemPrimitiveV::PathNameJoin =>
			Some (FileSystemPrimitiveN::PathNameJoin),
		FileSystemPrimitiveV::PathNameSplit =>
			None,
		FileSystemPrimitiveV::MetadataResolve =>
			None,
		FileSystemPrimitiveV::MetadataIsSelf =>
			None,
		FileSystemPrimitiveV::MetadataKindGet =>
			None,
		FileSystemPrimitiveV::MetadataFileGetSize =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmpty =>
			None,
		FileSystemPrimitiveV::MetadataFileIsEmptyNot =>
			None,
		FileSystemPrimitiveV::MetadataIsReadonly =>
			None,
		FileSystemPrimitiveV::MetadataIsReadable =>
			None,
		FileSystemPrimitiveV::MetadataIsWriteable =>
			None,
		FileSystemPrimitiveV::MetadataFileIsExecutable =>
			None,
		FileSystemPrimitiveV::MetadataDirectoryIsTraversable =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetMode =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetType =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetPermissions =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetUserIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeDevice =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeNumber =>
			None,
		FileSystemPrimitiveV::MetadataUnixGetInodeLinks =>
			None,
		FileSystemPrimitiveV::SymLinkResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateFile =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
		FileSystemPrimitiveV::TemporaryCreateDirectory =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_0_attributes (_primitive : FileSystemPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_1_attributes (_primitive : FileSystemPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_2_attributes (_primitive : FileSystemPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_3_attributes (_primitive : FileSystemPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_4_attributes (_primitive : FileSystemPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_5_attributes (_primitive : FileSystemPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn filesystem_primitive_n_attributes (_primitive : FileSystemPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

