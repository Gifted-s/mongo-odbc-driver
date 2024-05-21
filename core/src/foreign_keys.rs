use crate::{col_metadata::MongoColMetadata, stmt::EmptyStatement, BsonTypeInfo};
use definitions::Nullability;
use once_cell::sync::OnceCell;

static FK_METADATA: OnceCell<Vec<MongoColMetadata>> = OnceCell::new();

pub struct MongoForeignKeys {}

impl MongoForeignKeys {
    pub fn empty(max_string_length: Option<u16>) -> EmptyStatement {
        EmptyStatement {
            resultset_metadata: FK_METADATA.get_or_init(|| {
                vec![
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "PKTABLE_CAT".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "PKTABLE_SCHEM".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "PKTABLE_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NO_NULLS,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "PKCOLUMN_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NO_NULLS,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "FKTABLE_CAT".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "FKTABLE_SCHEM".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "FKTABLE_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NO_NULLS,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "FKCOLUMN_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NO_NULLS,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "KEY_SEQ".to_string(),
                        BsonTypeInfo::INT,
                        max_string_length,
                        Nullability::SQL_NO_NULLS,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "UPDATE_RULE".to_string(),
                        BsonTypeInfo::INT,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "DELETE_RULE".to_string(),
                        BsonTypeInfo::INT,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "FK_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "PK_NAME".to_string(),
                        BsonTypeInfo::STRING,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                    MongoColMetadata::new_metadata_from_bson_type_info_default(
                        "",
                        "".to_string(),
                        "DEFERRABILITY".to_string(),
                        BsonTypeInfo::INT,
                        max_string_length,
                        Nullability::SQL_NULLABLE,
                    ),
                ]
            }),
        }
    }
}
