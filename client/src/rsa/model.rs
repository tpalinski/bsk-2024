use std::error::Error;

use rsa::RsaPrivateKey;
use strong_xml::{XmlRead, XmlWrite};

use super::{encryption, utils::get_hash};

#[derive(XmlWrite, XmlRead, Clone, Debug)]
#[xml(tag="ds:Signature")]
pub struct Signature {
    #[xml(child="ds:SignedInfo")]
    info: SignedInfo,
    #[xml(child="ds:SignatureValue")]
    sig: Sigval,
    #[xml(child="ds:Object")]
    obj: Object
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            info: SignedInfo {
                cm: CanonMethod { 
                    algorithm:"http://www.w3.org/2001/10/xml-exc-c14n#".to_owned()
                },
                sm: SigMethod{
                    algorithm: "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256".to_owned()
                },
                rf: Reference {
                    dm: AlgoTag {
                        algorithm: "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256".to_owned()
                    },
                    dv: Digest {
                        text: "hash".to_owned()
                    }
                }
            },
            sig: Sigval {text: "signature".to_owned()},
            obj: Object {
                qp: QualifyingProps {
                    sp: SignedProperties {
                        ssp: SignedSignatureProperties {
                            cert: SigningCert{ text: "certificate".to_owned()},
                            city: SigningPlace{text: "Gdansk".to_owned()},
                            time: SigningTime {text: format!("{:?}", chrono::offset::Local::now())}
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
        self.obj.qp.sp.ssp.cert.text = author;
        self
    }

    fn set_hash(mut self, data: &[u8]) -> Self {
        self.info.rf.dv.text = get_hash(data);
        self
    }

    fn set_signature(mut self, data: &[u8], key: RsaPrivateKey) -> Self {
        let sig = encryption::sign(data, key);
        self.sig.text = sig;
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
        self.to_string().unwrap()
    }

    pub fn from_xml(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let xml_string = String::from_utf8(data)?;
        match Signature::from_str(&xml_string) {
            Ok(xades) => Ok(xades),
            Err(e) => Err(format!("{e}").into())
        }
    }
}

// Standard XMLSig fields

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag = "ds:SignedInfo")]
struct SignedInfo {
    #[xml(child="ds:CanonicalizationMethod")]
    cm: CanonMethod,

    #[xml(child="ds:SignatureMethod")]
    sm: SigMethod,

    #[xml(child="ds:Reference")]
    rf: Reference

}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag = "ds:CanonicalizationMethod")]
struct CanonMethod{
    #[xml(attr="Algorithm")]
    algorithm: String
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag = "ds:SignatureMethod")]
struct SigMethod{
    #[xml(attr="Algorithm")]
    algorithm: String
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag = "ds:DigestMethod")]
struct AlgoTag{
    #[xml(attr="Algorithm")]
    algorithm: String
}


#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="ds:Reference")]
struct Reference{
    #[xml(child="ds:DigestMethod")]
    dm: AlgoTag,

    #[xml(child="ds:DigestValue")]
    dv: Digest,
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="ds:DigestValue")]
struct Digest{
    #[xml(text)]
    text: String
}


#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="ds:SignatureValue")]
struct Sigval{
    #[xml(text)]
    text: String
}


// Object related fields

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="ds:Object")]
struct Object {
    #[xml(child="QualifyingProperties")]
    qp: QualifyingProps
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="QualifyingProperties")]
struct QualifyingProps {
    #[xml(child="SignedProperties")]
    sp: SignedProperties
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="SignedProperties")]
struct SignedProperties {
    #[xml(child="SignedSignatureProperties")]
    ssp: SignedSignatureProperties
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="SignedSignatureProperties")]
struct SignedSignatureProperties {
    #[xml(child="SigningTime")]
    time: SigningTime,
    #[xml(child="SigningCertificate")]
    cert: SigningCert,
    #[xml(child="SignatureProductionPlace")]
    city: SigningPlace
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="SigningTime")]
struct SigningTime {
    #[xml(text)]
    text: String
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="SigningCertificate")]
struct SigningCert {
    #[xml(text)]
    text: String
}

#[derive(XmlRead, XmlWrite, Clone, Debug)]
#[xml(tag="SignatureProductionPlace")]
struct SigningPlace {
    #[xml(text)]
    text: String
}
