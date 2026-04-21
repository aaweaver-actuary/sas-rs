use std::collections::HashMap;

use super::{ColumnKind, RowValueKind, SasColumn};

/// Schema description for one decoded row-batch column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowBatchColumn {
    /// Source column index in the SAS metadata.
    pub source_index: usize,
    /// Source column name.
    pub name: String,
    /// Physical SAS column kind.
    pub kind: ColumnKind,
    /// Logical row value kind after semantic inference.
    pub value_kind: RowValueKind,
    /// Whether the primary output column can contain nulls.
    pub nullable: bool,
    /// Output-field metadata used during transform projection.
    pub metadata: HashMap<String, String>,
    /// Companion missing-tag output column name when one exists.
    pub missing_tag_column_name: Option<String>,
    /// Metadata for the companion missing-tag output column.
    pub missing_tag_metadata: Option<HashMap<String, String>>,
}

impl RowBatchColumn {
    /// Build a row-batch column descriptor from a source SAS column.
    pub fn from_source(source_index: usize, column: &SasColumn) -> Self {
        let value_kind = RowValueKind::from_source_column(column);
        let missing_tag_column_name = if column.kind == ColumnKind::Numeric {
            Some(format!("{}__sas_missing_tag", column.name))
        } else {
            None
        };

        Self {
            source_index,
            name: column.name.clone(),
            kind: column.kind,
            value_kind,
            nullable: value_kind.is_nullable(),
            metadata: primary_field_metadata(column, missing_tag_column_name.as_deref()),
            missing_tag_metadata: missing_tag_column_name
                .as_ref()
                .map(|_| missing_tag_field_metadata(column)),
            missing_tag_column_name,
        }
    }
}

fn primary_field_metadata(
    column: &SasColumn,
    missing_tag_column_name: Option<&str>,
) -> HashMap<String, String> {
    let mut metadata = HashMap::from([
        (
            "sas.kind".to_string(),
            match column.kind {
                ColumnKind::Numeric => "numeric",
                ColumnKind::String => "string",
            }
            .to_string(),
        ),
        (
            "sas.semantic_type".to_string(),
            column.semantic_type.label().to_string(),
        ),
    ]);
    if let Some(label) = &column.metadata.label {
        metadata.insert("sas.label".to_string(), label.clone());
    }
    if let Some(format_name) = &column.metadata.format_name {
        metadata.insert("sas.format_name".to_string(), format_name.clone());
    }
    if let Some(informat_name) = &column.metadata.informat_name {
        metadata.insert("sas.informat_name".to_string(), informat_name.clone());
    }
    if let Some(missing_tag_column_name) = missing_tag_column_name {
        metadata.insert(
            "sas.missing_tag_column".to_string(),
            missing_tag_column_name.to_string(),
        );
    }
    metadata
}

fn missing_tag_field_metadata(column: &SasColumn) -> HashMap<String, String> {
    HashMap::from([
        ("sas.kind".to_string(), "missing_tag".to_string()),
        ("sas.parent_column".to_string(), column.name.clone()),
        ("sas.tag_domain".to_string(), ". _ A-Z".to_string()),
    ])
}
