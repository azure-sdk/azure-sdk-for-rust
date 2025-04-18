// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{
    ArrowField, BlobItemInternal, BlobPrefix, BlobTag, Block, ClearRange, ContainerItem, CorsRule,
    FilterBlobItem, PageRange,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize)]
#[serde(rename = "BlobItems")]
pub(crate) struct Blob_itemsBlobItemInternal {
    #[serde(default)]
    BlobItemInternal: Vec<BlobItemInternal>,
}

impl Blob_itemsBlobItemInternal {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<BlobItemInternal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Blob_itemsBlobItemInternal::deserialize(deserializer)?.BlobItemInternal)
    }

    pub fn wrap<S>(to_serialize: &Vec<BlobItemInternal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Blob_itemsBlobItemInternal {
            BlobItemInternal: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "BlobPrefixes")]
pub(crate) struct Blob_prefixesBlobPrefix {
    #[serde(default)]
    BlobPrefix: Vec<BlobPrefix>,
}

impl Blob_prefixesBlobPrefix {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<BlobPrefix>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Blob_prefixesBlobPrefix::deserialize(deserializer)?.BlobPrefix)
    }

    pub fn wrap<S>(to_serialize: &Vec<BlobPrefix>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Blob_prefixesBlobPrefix {
            BlobPrefix: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "TagSet")]
pub(crate) struct Blob_tag_setBlobTag {
    #[serde(default)]
    BlobTag: Vec<BlobTag>,
}

impl Blob_tag_setBlobTag {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<BlobTag>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Blob_tag_setBlobTag::deserialize(deserializer)?.BlobTag)
    }

    pub fn wrap<S>(to_serialize: &Vec<BlobTag>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Blob_tag_setBlobTag {
            BlobTag: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "Blobs")]
pub(crate) struct BlobsFilterBlobItem {
    #[serde(default)]
    FilterBlobItem: Vec<FilterBlobItem>,
}

impl BlobsFilterBlobItem {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<FilterBlobItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BlobsFilterBlobItem::deserialize(deserializer)?.FilterBlobItem)
    }

    pub fn wrap<S>(to_serialize: &Vec<FilterBlobItem>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        BlobsFilterBlobItem {
            FilterBlobItem: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "ClearRange")]
pub(crate) struct Clear_rangeClearRange {
    #[serde(default)]
    ClearRange: Vec<ClearRange>,
}

impl Clear_rangeClearRange {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<ClearRange>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Clear_rangeClearRange::deserialize(deserializer)?.ClearRange)
    }

    pub fn wrap<S>(to_serialize: &Vec<ClearRange>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Clear_rangeClearRange {
            ClearRange: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "CommittedBlocks")]
pub(crate) struct Committed_blocksBlock {
    #[serde(default)]
    Block: Vec<Block>,
}

impl Committed_blocksBlock {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<Block>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Committed_blocksBlock::deserialize(deserializer)?.Block)
    }

    pub fn wrap<S>(to_serialize: &Vec<Block>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Committed_blocksBlock {
            Block: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "Containers")]
pub(crate) struct Container_itemsContainerItem {
    #[serde(default)]
    ContainerItem: Vec<ContainerItem>,
}

impl Container_itemsContainerItem {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<ContainerItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Container_itemsContainerItem::deserialize(deserializer)?.ContainerItem)
    }

    pub fn wrap<S>(to_serialize: &Vec<ContainerItem>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Container_itemsContainerItem {
            ContainerItem: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "Cors")]
pub(crate) struct CorsCorsRule {
    #[serde(default)]
    CorsRule: Vec<CorsRule>,
}

impl CorsCorsRule {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<CorsRule>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CorsCorsRule::deserialize(deserializer)?.CorsRule)
    }

    pub fn wrap<S>(to_serialize: &Vec<CorsRule>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        CorsCorsRule {
            CorsRule: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "PageRange")]
pub(crate) struct Page_rangePageRange {
    #[serde(default)]
    PageRange: Vec<PageRange>,
}

impl Page_rangePageRange {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<PageRange>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Page_rangePageRange::deserialize(deserializer)?.PageRange)
    }

    pub fn wrap<S>(to_serialize: &Vec<PageRange>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Page_rangePageRange {
            PageRange: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "Schema")]
pub(crate) struct SchemaArrowField {
    #[serde(default)]
    ArrowField: Vec<ArrowField>,
}

impl SchemaArrowField {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<ArrowField>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(SchemaArrowField::deserialize(deserializer)?.ArrowField)
    }

    pub fn wrap<S>(to_serialize: &Vec<ArrowField>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SchemaArrowField {
            ArrowField: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "UncommittedBlocks")]
pub(crate) struct Uncommitted_blocksBlock {
    #[serde(default)]
    Block: Vec<Block>,
}

impl Uncommitted_blocksBlock {
    pub fn unwrap<'de, D>(deserializer: D) -> Result<Vec<Block>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Uncommitted_blocksBlock::deserialize(deserializer)?.Block)
    }

    pub fn wrap<S>(to_serialize: &Vec<Block>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Uncommitted_blocksBlock {
            Block: to_serialize.to_owned(),
        }
        .serialize(serializer)
    }
}
