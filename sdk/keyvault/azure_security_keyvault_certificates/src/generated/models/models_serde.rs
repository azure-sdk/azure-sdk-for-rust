// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use super::{
    CertificatePolicy, Contacts, CreateCertificateParameters, ImportCertificateParameters,
    MergeCertificateParameters, RestoreCertificateParameters, SetIssuerParameters,
    UpdateCertificateOperationParameter, UpdateCertificatePropertiesParameters,
    UpdateIssuerParameters,
};
use azure_core::{http::RequestContent, json::to_json, Result};

impl TryFrom<CertificatePolicy> for RequestContent<CertificatePolicy> {
    type Error = azure_core::Error;
    fn try_from(value: CertificatePolicy) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Contacts> for RequestContent<Contacts> {
    type Error = azure_core::Error;
    fn try_from(value: Contacts) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<CreateCertificateParameters> for RequestContent<CreateCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: CreateCertificateParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<ImportCertificateParameters> for RequestContent<ImportCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: ImportCertificateParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<MergeCertificateParameters> for RequestContent<MergeCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: MergeCertificateParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<RestoreCertificateParameters> for RequestContent<RestoreCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: RestoreCertificateParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<SetIssuerParameters> for RequestContent<SetIssuerParameters> {
    type Error = azure_core::Error;
    fn try_from(value: SetIssuerParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<UpdateCertificateOperationParameter>
    for RequestContent<UpdateCertificateOperationParameter>
{
    type Error = azure_core::Error;
    fn try_from(value: UpdateCertificateOperationParameter) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<UpdateCertificatePropertiesParameters>
    for RequestContent<UpdateCertificatePropertiesParameters>
{
    type Error = azure_core::Error;
    fn try_from(value: UpdateCertificatePropertiesParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<UpdateIssuerParameters> for RequestContent<UpdateIssuerParameters> {
    type Error = azure_core::Error;
    fn try_from(value: UpdateIssuerParameters) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

pub mod vec_encoded_bytes_std {
    #![allow(clippy::type_complexity)]
    use azure_core::base64;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::result::Result;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let to_deserialize = <Option<Vec<String>>>::deserialize(deserializer)?;
        match to_deserialize {
            Some(to_deserialize) => {
                let mut decoded0 = <Vec<Vec<u8>>>::new();
                for v in to_deserialize {
                    decoded0.push(base64::decode(v).map_err(serde::de::Error::custom)?);
                }
                Ok(decoded0)
            }
            None => Ok(<Vec<Vec<u8>>>::default()),
        }
    }

    pub fn serialize<S>(to_serialize: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded0 = to_serialize.iter().map(base64::encode).collect();
        <Vec<String>>::serialize(&encoded0, serializer)
    }
}
