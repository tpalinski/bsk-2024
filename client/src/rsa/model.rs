use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename="ds:Signature")]
pub struct Signature {
    #[serde(rename="ds:SignedInfo")]
    info: SignedInfo,
    #[serde(rename="ds:Object")]
    obj: Object
}

impl Signature {
    pub fn new() -> Self {
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
            obj: Object {
                qp: QualifyingProps {
                    sp: SignedProperties {
                        ssp: SignedSignatureProperties {
                            cert: "certificate".to_owned(),
                            city: "city".to_owned(),
                            time: "now".to_owned()
                        }
                    }
                }
            }
        }
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
