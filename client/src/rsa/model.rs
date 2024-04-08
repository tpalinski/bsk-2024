use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};

use super::{encryption, utils::get_hash};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename="ds:Signature")]
pub struct Signature {
    #[serde(rename="ds:SignedInfo")]
    info: SignedInfo,
    #[serde(rename="ds:SignatureValue")]
    sig: String,
    #[serde(rename="ds:Object")]
    obj: Object
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            info: SignedInfo {
                cm: AlgoTag{ 
                    algorithm:"http://www.w3.org/2001/10/xml-exc-c14n#".to_owned()
                },
                sm: AlgoTag{
                    algorithm: "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256".to_owned()
                },
                rf: Reference {
                    dm: AlgoTag {
                        algorithm: "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256".to_owned()
                    },
                    dv: "hash".to_owned()
                }
            },
            sig: "signature".to_owned(),
            obj: Object {
                qp: QualifyingProps {
                    sp: SignedProperties {
                        ssp: SignedSignatureProperties {
                            cert: "certificate".to_owned(),
                            city: "Gdansk".to_owned(),
                            time: format!("{:?}", chrono::offset::Local::now())
                        }
                    }
                }
            }
        }
    }
}

pub struct SignatureProps<'a> {
    pub pkey: RsaPrivateKey,
    pub data: &'a[u8],
    pub author: String
}

impl Signature {
    fn set_author(mut self, author: String) -> Self {
        self.obj.qp.sp.ssp.cert = author;
        self
    }

    fn set_hash(mut self, data: &[u8]) -> Self {
        self.info.rf.dv = get_hash(data);
        self
    }

    fn set_signature(mut self, data: &[u8], key: RsaPrivateKey) -> Self {
        let sig = encryption::sign(data, key);
        self.sig = sig;
        self
    }

    pub fn generate(props: SignatureProps) -> Self {
        let sig = Signature::default();
        sig
            .set_author(props.author)
            .set_hash(&props.data)
            .set_signature(&props.data, props.pkey)
    }

    pub fn to_xml(&self) -> String {
        quick_xml::se::to_string(self).unwrap()
    }
}

// Standard XMLSig fields

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SignedInfo {
    #[serde(rename="ds:CanonicalizationMethod")]
    cm: AlgoTag,

    #[serde(rename="ds:SignatureMethod")]
    sm: AlgoTag,

    #[serde(rename="ds:Reference")]
    rf: Reference

}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AlgoTag{
    #[serde(rename="@Algorithm")]
    algorithm: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Reference{
    #[serde(rename="ds:DigestMethod")]
    dm: AlgoTag,

    #[serde(rename="ds:DigestValue")]
    dv: String,
}

// Object related fields

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Object {
    #[serde(rename="QualifyingProperties")]
    qp: QualifyingProps
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct QualifyingProps {
    #[serde(rename="SignedProperties")]
    sp: SignedProperties
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SignedProperties {
    #[serde(rename="SignedSignatureProperties")]
    ssp: SignedSignatureProperties
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SignedSignatureProperties {
    #[serde(rename="SigningTime")]
    time: String,
    #[serde(rename="SigningCertificate")]
    cert: String,
    #[serde(rename="SignatureProductionPlace")]
    city: String

}
