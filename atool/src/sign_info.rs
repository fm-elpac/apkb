//! apk 签名信息

use apksig::{
    Algorithms, RawData, SignatureSchemeV2, SignatureSchemeV3, SigningBlock, ValueSigningBlock,
    common::{AdditionalAttribute, Certificate, Digest, PubKey, Signature},
    signing_block::{scheme_v2, scheme_v3},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

fn encode_bytes(value: &[u8]) -> String {
    base16ct::lower::encode_string(value)
}

/// apk 签名信息 (JSON)
#[derive(Serialize, Deserialize)]
pub struct SignInfo {
    pub content: Vec<SignInfoBlock>,
}

impl From<SigningBlock> for SignInfo {
    fn from(value: SigningBlock) -> Self {
        Self {
            content: value.content.iter().map(|i| i.into()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct SignInfoBlock {
    pub raw: Option<String>,
    pub v2: Option<SignInfoBlockV2>,
    pub v3: Option<SignInfoBlockV3>,
}

impl From<&ValueSigningBlock> for SignInfoBlock {
    fn from(value: &ValueSigningBlock) -> Self {
        match value {
            ValueSigningBlock::BaseSigningBlock(raw) => Self {
                raw: Some(s(raw.into())),
                v2: None,
                v3: None,
            },
            ValueSigningBlock::SignatureSchemeV2Block(v2) => Self {
                raw: None,
                v2: Some(v2.into()),
                v3: None,
            },
            ValueSigningBlock::SignatureSchemeV3Block(v3) => Self {
                raw: None,
                v2: None,
                v3: Some(v3.into()),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoBlockV2 {
    pub id: u32,
    pub signers: Vec<SignInfoSignerV2>,
}

impl From<&SignatureSchemeV2> for SignInfoBlockV2 {
    fn from(value: &SignatureSchemeV2) -> Self {
        Self {
            id: value.id,
            signers: value
                .signers
                .signers_data
                .iter()
                .map(|i| i.into())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoBlockV3 {
    pub id: u32,
    pub signers: Vec<SignInfoSignerV3>,
}

impl From<&SignatureSchemeV3> for SignInfoBlockV3 {
    fn from(value: &SignatureSchemeV3) -> Self {
        Self {
            id: value.id,
            signers: value
                .signers
                .signers_data
                .iter()
                .map(|i| i.into())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoSignerV2 {
    pub pub_key: String,
    pub signatures: Vec<SignInfoSign>,
    pub signed_data: SignInfoSignedDataV2,
}

impl From<&scheme_v2::Signer> for SignInfoSignerV2 {
    fn from(value: &scheme_v2::Signer) -> Self {
        Self {
            pub_key: s((&value.pub_key).into()),
            signatures: value
                .signatures
                .signatures_data
                .iter()
                .map(|i| i.into())
                .collect(),
            signed_data: (&value.signed_data).into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoSignerV3 {
    pub pub_key: String,
    pub min_sdk: u32,
    pub max_sdk: u32,
    pub signatures: Vec<SignInfoSign>,
    pub signed_data: SignInfoSignedDataV3,
}

impl From<&scheme_v3::Signer> for SignInfoSignerV3 {
    fn from(value: &scheme_v3::Signer) -> Self {
        Self {
            pub_key: s((&value.pub_key).into()),
            min_sdk: value.min_sdk,
            max_sdk: value.max_sdk,
            signatures: value
                .signatures
                .signatures_data
                .iter()
                .map(|i| i.into())
                .collect(),
            signed_data: (&value.signed_data).into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoSignedDataV2 {
    pub digests: Vec<SignInfoDigest>,
    pub certificates: Vec<String>,
    pub additional_attributes: Vec<SignInfoAddAttr>,
}

impl From<&scheme_v2::SignedData> for SignInfoSignedDataV2 {
    fn from(value: &scheme_v2::SignedData) -> Self {
        Self {
            digests: value
                .digests
                .digests_data
                .iter()
                .map(|i| i.into())
                .collect(),
            certificates: value
                .certificates
                .certificates_data
                .iter()
                .map(|i| s(i.into()))
                .collect(),
            additional_attributes: value
                .additional_attributes
                .additional_attributes_data
                .iter()
                .map(|i| i.into())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoSignedDataV3 {
    pub digests: Vec<SignInfoDigest>,
    pub min_sdk: u32,
    pub max_sdk: u32,
    pub certificates: Vec<String>,
    pub additional_attributes: Vec<SignInfoAddAttr>,
}

impl From<&scheme_v3::SignedData> for SignInfoSignedDataV3 {
    fn from(value: &scheme_v3::SignedData) -> Self {
        Self {
            digests: value
                .digests
                .digests_data
                .iter()
                .map(|i| i.into())
                .collect(),
            min_sdk: value.min_sdk,
            max_sdk: value.max_sdk,
            certificates: value
                .certificates
                .certificates_data
                .iter()
                .map(|i| s(i.into()))
                .collect(),
            additional_attributes: value
                .additional_attributes
                .additional_attributes_data
                .iter()
                .map(|i| i.into())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoSign {
    pub algorithm: String,
    pub signature: String,
}

impl From<&Signature> for SignInfoSign {
    fn from(value: &Signature) -> Self {
        Self {
            algorithm: s((&value.signature_algorithm_id).into()),
            signature: encode_bytes(&value.signature),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoDigest {
    pub algorithm: String,
    pub digest: String,
}

impl From<&Digest> for SignInfoDigest {
    fn from(value: &Digest) -> Self {
        Self {
            algorithm: s((&value.signature_algorithm_id).into()),
            digest: encode_bytes(&value.digest),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInfoAddAttr {
    pub id: u32,
    pub data: String,
}

impl From<&AdditionalAttribute> for SignInfoAddAttr {
    fn from(value: &AdditionalAttribute) -> Self {
        Self {
            id: value.id,
            data: encode_bytes(&value.data),
        }
    }
}

/// 方便类型转换 (String)
pub struct StringBox(String);

impl From<String> for StringBox {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for StringBox {
    fn into(self) -> String {
        self.0
    }
}

pub fn s(b: StringBox) -> String {
    b.into()
}

impl From<&RawData> for StringBox {
    fn from(value: &RawData) -> Self {
        encode_bytes(&value.data).into()
    }
}

impl From<&PubKey> for StringBox {
    fn from(value: &PubKey) -> Self {
        encode_bytes(&value.data).into()
    }
}

impl From<&Certificate> for StringBox {
    fn from(value: &Certificate) -> Self {
        encode_bytes(&value.certificate).into()
    }
}

impl From<&Algorithms> for StringBox {
    fn from(value: &Algorithms) -> Self {
        match value {
            Algorithms::RSASSA_PSS_256 => "RSASSA_PSS_256".into(),
            Algorithms::RSASSA_PSS_512 => "RSASSA_PSS_512".into(),
            Algorithms::RSASSA_PKCS1_v1_5_256 => "RSASSA_PKCS1_v1_5_256".into(),
            Algorithms::RSASSA_PKCS1_v1_5_512 => "RSASSA_PKCS1_v1_5_512".into(),
            Algorithms::ECDSA_SHA2_256 => "ECDSA_SHA2_256".into(),
            Algorithms::ECDSA_SHA2_512 => "ECDSA_SHA2_512".into(),
            Algorithms::DSA_SHA2_256 => "DSA_SHA2_256".into(),
            Algorithms::Unknown(x) => format!("{}", x),
        }
        .into()
    }
}
