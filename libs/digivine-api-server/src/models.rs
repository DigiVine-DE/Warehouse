#![allow(unused_qualifications)]

#[cfg(any(feature = "client", feature = "server"))]
use crate::header;
use crate::models;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AccessRight {
    /// The path to give access to. `%` can be used as wildcard character with
    #[serde(rename = "path")]
    pub path: String,

    /// Whether the access is read-only.
    #[serde(rename = "read_only")]
    pub read_only: bool,
}

impl AccessRight {
    pub fn new(path: String, read_only: bool) -> AccessRight {
        AccessRight {
            path: path,
            read_only: read_only,
        }
    }
}

/// Converts the AccessRight value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AccessRight {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("path".to_string());
        params.push(self.path.to_string());

        params.push("read_only".to_string());
        params.push(self.read_only.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AccessRight value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AccessRight {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub path: Vec<String>,
            pub read_only: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AccessRight".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "path" => intermediate_rep.path.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "read_only" => intermediate_rep.read_only.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AccessRight".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AccessRight {
            path: intermediate_rep
                .path
                .into_iter()
                .next()
                .ok_or("path missing in AccessRight".to_string())?,
            read_only: intermediate_rep
                .read_only
                .into_iter()
                .next()
                .ok_or("read_only missing in AccessRight".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AccessRight> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AccessRight>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AccessRight>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AccessRight - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AccessRight> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AccessRight as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AccessRight - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing a cultivar.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Cultivar {
    /// The ID of the cultivar. If it is used to insert a new cultivar, this value will be ignored.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// The name of the cultivar.
    #[serde(rename = "cultivar_name")]
    pub cultivar_name: String,

    /// The prime name of the cultivar.
    #[serde(rename = "prime_name")]
    pub prime_name: String,

    /// The variety number of the cultivar.
    #[serde(rename = "variety_number")]
    pub variety_number: i32,

    /// The species of the cultivar.
    #[serde(rename = "species")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub species: Option<String>,

    /// The color of the berry skin.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// The country or region of origin of the variety.
    #[serde(rename = "country_or_region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}

impl Cultivar {
    pub fn new(cultivar_name: String, prime_name: String, variety_number: i32) -> Cultivar {
        Cultivar {
            id: None,
            cultivar_name: cultivar_name,
            prime_name: prime_name,
            variety_number: variety_number,
            species: None,
            color: None,
            country_or_region: None,
        }
    }
}

/// Converts the Cultivar value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Cultivar {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.push("cultivar_name".to_string());
        params.push(self.cultivar_name.to_string());

        params.push("prime_name".to_string());
        params.push(self.prime_name.to_string());

        params.push("variety_number".to_string());
        params.push(self.variety_number.to_string());

        if let Some(ref species) = self.species {
            params.push("species".to_string());
            params.push(species.to_string());
        }

        if let Some(ref color) = self.color {
            params.push("color".to_string());
            params.push(color.to_string());
        }

        if let Some(ref country_or_region) = self.country_or_region {
            params.push("country_or_region".to_string());
            params.push(country_or_region.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Cultivar value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Cultivar {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub cultivar_name: Vec<String>,
            pub prime_name: Vec<String>,
            pub variety_number: Vec<i32>,
            pub species: Vec<String>,
            pub color: Vec<String>,
            pub country_or_region: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Cultivar".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "cultivar_name" => intermediate_rep.cultivar_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "prime_name" => intermediate_rep.prime_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "variety_number" => intermediate_rep.variety_number.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "species" => intermediate_rep.species.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "color" => intermediate_rep.color.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "country_or_region" => intermediate_rep.country_or_region.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Cultivar".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Cultivar {
            id: intermediate_rep.id.into_iter().next(),
            cultivar_name: intermediate_rep
                .cultivar_name
                .into_iter()
                .next()
                .ok_or("cultivar_name missing in Cultivar".to_string())?,
            prime_name: intermediate_rep
                .prime_name
                .into_iter()
                .next()
                .ok_or("prime_name missing in Cultivar".to_string())?,
            variety_number: intermediate_rep
                .variety_number
                .into_iter()
                .next()
                .ok_or("variety_number missing in Cultivar".to_string())?,
            species: intermediate_rep.species.into_iter().next(),
            color: intermediate_rep.color.into_iter().next(),
            country_or_region: intermediate_rep.country_or_region.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Cultivar> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Cultivar>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Cultivar>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Cultivar - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Cultivar> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Cultivar as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Cultivar - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CultivarPatch {
    /// The name of the cultivar.
    #[serde(rename = "cultivar_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cultivar_name: Option<String>,

    /// The prime name of the cultivar.
    #[serde(rename = "prime_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_name: Option<String>,

    /// The variety number of the cultivar.
    #[serde(rename = "variety_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety_number: Option<i32>,

    /// The species of the cultivar.
    #[serde(rename = "species")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub species: Option<String>,

    /// The color of the berry skin.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// The country or region of origin of the variety.
    #[serde(rename = "country_or_region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}

impl CultivarPatch {
    pub fn new() -> CultivarPatch {
        CultivarPatch {
            cultivar_name: None,
            prime_name: None,
            variety_number: None,
            species: None,
            color: None,
            country_or_region: None,
        }
    }
}

/// Converts the CultivarPatch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CultivarPatch {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cultivar_name) = self.cultivar_name {
            params.push("cultivar_name".to_string());
            params.push(cultivar_name.to_string());
        }

        if let Some(ref prime_name) = self.prime_name {
            params.push("prime_name".to_string());
            params.push(prime_name.to_string());
        }

        if let Some(ref variety_number) = self.variety_number {
            params.push("variety_number".to_string());
            params.push(variety_number.to_string());
        }

        if let Some(ref species) = self.species {
            params.push("species".to_string());
            params.push(species.to_string());
        }

        if let Some(ref color) = self.color {
            params.push("color".to_string());
            params.push(color.to_string());
        }

        if let Some(ref country_or_region) = self.country_or_region {
            params.push("country_or_region".to_string());
            params.push(country_or_region.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CultivarPatch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CultivarPatch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cultivar_name: Vec<String>,
            pub prime_name: Vec<String>,
            pub variety_number: Vec<i32>,
            pub species: Vec<String>,
            pub color: Vec<String>,
            pub country_or_region: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CultivarPatch".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "cultivar_name" => intermediate_rep.cultivar_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "prime_name" => intermediate_rep.prime_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "variety_number" => intermediate_rep.variety_number.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "species" => intermediate_rep.species.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "color" => intermediate_rep.color.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "country_or_region" => intermediate_rep.country_or_region.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CultivarPatch".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CultivarPatch {
            cultivar_name: intermediate_rep.cultivar_name.into_iter().next(),
            prime_name: intermediate_rep.prime_name.into_iter().next(),
            variety_number: intermediate_rep.variety_number.into_iter().next(),
            species: intermediate_rep.species.into_iter().next(),
            color: intermediate_rep.color.into_iter().next(),
            country_or_region: intermediate_rep.country_or_region.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CultivarPatch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CultivarPatch>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CultivarPatch>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CultivarPatch - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CultivarPatch> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CultivarPatch as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CultivarPatch - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error {
    /// A uniquely identifiable code for the error message
    #[serde(rename = "code")]
    pub code: String,

    /// The error message
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    pub fn new(code: String, message: String) -> Error {
        Error {
            code: code,
            message: message,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("code".to_string());
        params.push(self.code.to_string());

        params.push("message".to_string());
        params.push(self.message.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub code: Vec<String>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Error".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Error".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or("code missing in Error".to_string())?,
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or("message missing in Error".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Error>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Error - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Error as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Error - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Field {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<models::MultiPolygon>,

    /// An array with data on planting
    #[serde(rename = "planter_runs")]
    pub planter_runs: Vec<models::PlanterRun>,

    #[serde(rename = "properties")]
    pub properties: models::FieldProperties,
}

impl Field {
    pub fn new(
        type_: String,
        planter_runs: Vec<models::PlanterRun>,
        properties: models::FieldProperties,
    ) -> Field {
        Field {
            type_: type_,
            geometry: None,
            planter_runs: planter_runs,
            properties: properties,
        }
    }
}

/// Converts the Field value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Field {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping planter_runs in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Field value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::MultiPolygon>,
            pub planter_runs: Vec<Vec<models::PlanterRun>>,
            pub properties: Vec<models::FieldProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Field".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::MultiPolygon as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "planter_runs" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Field"
                                .to_string(),
                        )
                    }
                    "properties" => intermediate_rep.properties.push(
                        <models::FieldProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Field".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Field {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Field".to_string())?,
            geometry: intermediate_rep.geometry.into_iter().next(),
            planter_runs: intermediate_rep
                .planter_runs
                .into_iter()
                .next()
                .ok_or("planter_runs missing in Field".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in Field".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Field> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Field>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Field>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Field - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Field> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Field as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Field - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FieldPatch {
    /// Field location
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The numeric ID of the station that makes Forecasts for a group of Fields.
    #[serde(rename = "station_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_id: Option<i64>,

    /// The numeric ID of the historical weather station associated with the fields.
    #[serde(rename = "historical_station_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_station_id: Option<i32>,

    /// The Number of the Field (not the ID)
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Size of the Field given in square meters
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    /// Name of the fruit on this Field
    #[serde(rename = "fruit_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fruit_name: Option<String>,
}

impl FieldPatch {
    pub fn new() -> FieldPatch {
        FieldPatch {
            name: None,
            station_id: None,
            historical_station_id: None,
            number: None,
            size: None,
            fruit_name: None,
        }
    }
}

/// Converts the FieldPatch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FieldPatch {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        if let Some(ref station_id) = self.station_id {
            params.push("station_id".to_string());
            params.push(station_id.to_string());
        }

        if let Some(ref historical_station_id) = self.historical_station_id {
            params.push("historical_station_id".to_string());
            params.push(historical_station_id.to_string());
        }

        if let Some(ref number) = self.number {
            params.push("number".to_string());
            params.push(number.to_string());
        }

        if let Some(ref size) = self.size {
            params.push("size".to_string());
            params.push(size.to_string());
        }

        if let Some(ref fruit_name) = self.fruit_name {
            params.push("fruit_name".to_string());
            params.push(fruit_name.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FieldPatch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FieldPatch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
            pub station_id: Vec<i64>,
            pub historical_station_id: Vec<i32>,
            pub number: Vec<String>,
            pub size: Vec<f64>,
            pub fruit_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing FieldPatch".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "station_id" => intermediate_rep.station_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "historical_station_id" => intermediate_rep.historical_station_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "number" => intermediate_rep.number.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "size" => intermediate_rep.size.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_name" => intermediate_rep.fruit_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing FieldPatch".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FieldPatch {
            name: intermediate_rep.name.into_iter().next(),
            station_id: intermediate_rep.station_id.into_iter().next(),
            historical_station_id: intermediate_rep.historical_station_id.into_iter().next(),
            number: intermediate_rep.number.into_iter().next(),
            size: intermediate_rep.size.into_iter().next(),
            fruit_name: intermediate_rep.fruit_name.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FieldPatch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FieldPatch>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<FieldPatch>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for FieldPatch - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FieldPatch> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <FieldPatch as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into FieldPatch - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FieldProperties {
    /// The numeric ID of the field
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Field location
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The numeric ID of the station that makes Forecasts for a group of Fields.
    #[serde(rename = "station_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_id: Option<i64>,

    /// The numeric ID of the historical weather station associated with the fields.
    #[serde(rename = "historical_station_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_station_id: Option<i32>,

    /// The Number of the Field (not the ID)
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Size of the Field given in square meters
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    /// Name of the fruit on this Field
    #[serde(rename = "fruit_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fruit_name: Option<String>,
}

impl FieldProperties {
    pub fn new() -> FieldProperties {
        FieldProperties {
            id: None,
            name: None,
            station_id: None,
            historical_station_id: None,
            number: None,
            size: None,
            fruit_name: None,
        }
    }
}

/// Converts the FieldProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FieldProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        if let Some(ref station_id) = self.station_id {
            params.push("station_id".to_string());
            params.push(station_id.to_string());
        }

        if let Some(ref historical_station_id) = self.historical_station_id {
            params.push("historical_station_id".to_string());
            params.push(historical_station_id.to_string());
        }

        if let Some(ref number) = self.number {
            params.push("number".to_string());
            params.push(number.to_string());
        }

        if let Some(ref size) = self.size {
            params.push("size".to_string());
            params.push(size.to_string());
        }

        if let Some(ref fruit_name) = self.fruit_name {
            params.push("fruit_name".to_string());
            params.push(fruit_name.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FieldProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FieldProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub name: Vec<String>,
            pub station_id: Vec<i64>,
            pub historical_station_id: Vec<i32>,
            pub number: Vec<String>,
            pub size: Vec<f64>,
            pub fruit_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing FieldProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "station_id" => intermediate_rep.station_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "historical_station_id" => intermediate_rep.historical_station_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "number" => intermediate_rep.number.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "size" => intermediate_rep.size.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_name" => intermediate_rep.fruit_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing FieldProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FieldProperties {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            station_id: intermediate_rep.station_id.into_iter().next(),
            historical_station_id: intermediate_rep.historical_station_id.into_iter().next(),
            number: intermediate_rep.number.into_iter().next(),
            size: intermediate_rep.size.into_iter().next(),
            fruit_name: intermediate_rep.fruit_name.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FieldProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FieldProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<FieldProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for FieldProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<FieldProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <FieldProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into FieldProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Forecast {
    /// The numeric ID of the Forecast
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Notice. This information will soon no longer be used and sent along!
    #[serde(rename = "group_id")]
    pub group_id: i64,

    /// The numeric ID of the station that makes Forecasts for a group of Fields.
    #[serde(rename = "station_id")]
    pub station_id: i64,

    /// Name of the measuring station
    #[serde(rename = "station")]
    pub station: String,

    /// An array containing the IDs of the Fields for which the Forecast is.
    #[serde(rename = "field_ids")]
    pub field_ids: Vec<i64>,

    /// The numeric ID of the model
    #[serde(rename = "model_id")]
    pub model_id: i64,

    /// Name of the model
    #[serde(rename = "model")]
    pub model: String,

    /// Time of retrieval of the data from an external source, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "query_timestamp")]
    pub query_timestamp: chrono::DateTime<chrono::Utc>,

    /// Start time of the measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "time_span_start")]
    pub time_span_start: chrono::DateTime<chrono::Utc>,

    /// End time of the measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "time_span_end")]
    pub time_span_end: chrono::DateTime<chrono::Utc>,

    /// Rate of the measurement
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "measurement_rate")]
    pub measurement_rate: String,

    /// An array of measurements
    #[serde(rename = "measurements")]
    pub measurements: Vec<models::ForecastMeasurement>,
}

impl Forecast {
    pub fn new(
        group_id: i64,
        station_id: i64,
        station: String,
        field_ids: Vec<i64>,
        model_id: i64,
        model: String,
        query_timestamp: chrono::DateTime<chrono::Utc>,
        time_span_start: chrono::DateTime<chrono::Utc>,
        time_span_end: chrono::DateTime<chrono::Utc>,
        measurement_rate: String,
        measurements: Vec<models::ForecastMeasurement>,
    ) -> Forecast {
        Forecast {
            id: None,
            group_id: group_id,
            station_id: station_id,
            station: station,
            field_ids: field_ids,
            model_id: model_id,
            model: model,
            query_timestamp: query_timestamp,
            time_span_start: time_span_start,
            time_span_end: time_span_end,
            measurement_rate: measurement_rate,
            measurements: measurements,
        }
    }
}

/// Converts the Forecast value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Forecast {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.push("group_id".to_string());
        params.push(self.group_id.to_string());

        params.push("station_id".to_string());
        params.push(self.station_id.to_string());

        params.push("station".to_string());
        params.push(self.station.to_string());

        params.push("field_ids".to_string());
        params.push(
            self.field_ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .to_string(),
        );

        params.push("model_id".to_string());
        params.push(self.model_id.to_string());

        params.push("model".to_string());
        params.push(self.model.to_string());

        // Skipping query_timestamp in query parameter serialization

        // Skipping time_span_start in query parameter serialization

        // Skipping time_span_end in query parameter serialization

        params.push("measurement_rate".to_string());
        params.push(self.measurement_rate.to_string());

        // Skipping measurements in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Forecast value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Forecast {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub group_id: Vec<i64>,
            pub station_id: Vec<i64>,
            pub station: Vec<String>,
            pub field_ids: Vec<Vec<i64>>,
            pub model_id: Vec<i64>,
            pub model: Vec<String>,
            pub query_timestamp: Vec<chrono::DateTime<chrono::Utc>>,
            pub time_span_start: Vec<chrono::DateTime<chrono::Utc>>,
            pub time_span_end: Vec<chrono::DateTime<chrono::Utc>>,
            pub measurement_rate: Vec<String>,
            pub measurements: Vec<Vec<models::ForecastMeasurement>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Forecast".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "group_id" => intermediate_rep.group_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "station_id" => intermediate_rep.station_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "station" => intermediate_rep.station.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "field_ids" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Forecast"
                                .to_string(),
                        )
                    }
                    "model_id" => intermediate_rep.model_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "model" => intermediate_rep.model.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "query_timestamp" => intermediate_rep.query_timestamp.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "time_span_start" => intermediate_rep.time_span_start.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "time_span_end" => intermediate_rep.time_span_end.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "measurement_rate" => intermediate_rep.measurement_rate.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "measurements" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Forecast"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Forecast".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Forecast {
            id: intermediate_rep.id.into_iter().next(),
            group_id: intermediate_rep
                .group_id
                .into_iter()
                .next()
                .ok_or("group_id missing in Forecast".to_string())?,
            station_id: intermediate_rep
                .station_id
                .into_iter()
                .next()
                .ok_or("station_id missing in Forecast".to_string())?,
            station: intermediate_rep
                .station
                .into_iter()
                .next()
                .ok_or("station missing in Forecast".to_string())?,
            field_ids: intermediate_rep
                .field_ids
                .into_iter()
                .next()
                .ok_or("field_ids missing in Forecast".to_string())?,
            model_id: intermediate_rep
                .model_id
                .into_iter()
                .next()
                .ok_or("model_id missing in Forecast".to_string())?,
            model: intermediate_rep
                .model
                .into_iter()
                .next()
                .ok_or("model missing in Forecast".to_string())?,
            query_timestamp: intermediate_rep
                .query_timestamp
                .into_iter()
                .next()
                .ok_or("query_timestamp missing in Forecast".to_string())?,
            time_span_start: intermediate_rep
                .time_span_start
                .into_iter()
                .next()
                .ok_or("time_span_start missing in Forecast".to_string())?,
            time_span_end: intermediate_rep
                .time_span_end
                .into_iter()
                .next()
                .ok_or("time_span_end missing in Forecast".to_string())?,
            measurement_rate: intermediate_rep
                .measurement_rate
                .into_iter()
                .next()
                .ok_or("measurement_rate missing in Forecast".to_string())?,
            measurements: intermediate_rep
                .measurements
                .into_iter()
                .next()
                .ok_or("measurements missing in Forecast".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Forecast> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Forecast>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Forecast>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Forecast - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Forecast> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Forecast as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Forecast - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ForecastMeasurement {
    /// The numeric ID of the Measurement
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Time of measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "time")]
    pub time: chrono::DateTime<chrono::Utc>,

    /// An object containing the values of the measurement.
    #[serde(rename = "values")]
    pub values: serde_json::Value,
}

impl ForecastMeasurement {
    pub fn new(
        time: chrono::DateTime<chrono::Utc>,
        values: serde_json::Value,
    ) -> ForecastMeasurement {
        ForecastMeasurement {
            id: None,
            time: time,
            values: values,
        }
    }
}

/// Converts the ForecastMeasurement value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ForecastMeasurement {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        // Skipping time in query parameter serialization

        // Skipping values in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ForecastMeasurement value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ForecastMeasurement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub time: Vec<chrono::DateTime<chrono::Utc>>,
            pub values: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ForecastMeasurement".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "time" => intermediate_rep.time.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "values" => intermediate_rep.values.push(
                        <serde_json::Value as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ForecastMeasurement".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ForecastMeasurement {
            id: intermediate_rep.id.into_iter().next(),
            time: intermediate_rep
                .time
                .into_iter()
                .next()
                .ok_or("time missing in ForecastMeasurement".to_string())?,
            values: intermediate_rep
                .values
                .into_iter()
                .next()
                .ok_or("values missing in ForecastMeasurement".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ForecastMeasurement> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ForecastMeasurement>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ForecastMeasurement>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ForecastMeasurement - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ForecastMeasurement>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ForecastMeasurement as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ForecastMeasurement - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Harvest {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// An array containing the measurements taken during the harvest task.
    #[serde(rename = "features")]
    pub features: Vec<models::HarvestMeasurement>,

    #[serde(rename = "properties")]
    pub properties: models::HarvestProperties,
}

impl Harvest {
    pub fn new(
        type_: String,
        features: Vec<models::HarvestMeasurement>,
        properties: models::HarvestProperties,
    ) -> Harvest {
        Harvest {
            type_: type_,
            features: features,
            properties: properties,
        }
    }
}

/// Converts the Harvest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Harvest {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping features in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Harvest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Harvest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub features: Vec<Vec<models::HarvestMeasurement>>,
            pub properties: Vec<models::HarvestProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Harvest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "features" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Harvest"
                                .to_string(),
                        )
                    }
                    "properties" => intermediate_rep.properties.push(
                        <models::HarvestProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Harvest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Harvest {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Harvest".to_string())?,
            features: intermediate_rep
                .features
                .into_iter()
                .next()
                .ok_or("features missing in Harvest".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in Harvest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Harvest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Harvest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Harvest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Harvest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Harvest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Harvest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Harvest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestMeasurement {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::Point,

    #[serde(rename = "properties")]
    pub properties: models::HarvestMeasurementProperties,
}

impl HarvestMeasurement {
    pub fn new(
        type_: String,
        geometry: models::Point,
        properties: models::HarvestMeasurementProperties,
    ) -> HarvestMeasurement {
        HarvestMeasurement {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the HarvestMeasurement value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestMeasurement {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestMeasurement value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestMeasurement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::Point>,
            pub properties: Vec<models::HarvestMeasurementProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestMeasurement".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Point as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::HarvestMeasurementProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestMeasurement".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestMeasurement {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in HarvestMeasurement".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in HarvestMeasurement".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in HarvestMeasurement".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestMeasurement> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestMeasurement>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestMeasurement>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestMeasurement - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<HarvestMeasurement>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HarvestMeasurement as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HarvestMeasurement - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestMeasurementProperties {
    /// Acid content of the harvested fruits in g/L
    #[serde(rename = "acid")]
    pub acid: i64,

    /// Sugar content of the harvested fruits in g/L
    #[serde(rename = "sugar")]
    pub sugar: i64,

    #[serde(rename = "spectral")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectral: Option<String>,
}

impl HarvestMeasurementProperties {
    pub fn new(acid: i64, sugar: i64) -> HarvestMeasurementProperties {
        HarvestMeasurementProperties {
            acid: acid,
            sugar: sugar,
            spectral: None,
        }
    }
}

/// Converts the HarvestMeasurementProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestMeasurementProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("acid".to_string());
        params.push(self.acid.to_string());

        params.push("sugar".to_string());
        params.push(self.sugar.to_string());

        if let Some(ref spectral) = self.spectral {
            params.push("spectral".to_string());
            params.push(spectral.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestMeasurementProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestMeasurementProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub acid: Vec<i64>,
            pub sugar: Vec<i64>,
            pub spectral: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestMeasurementProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "acid" => intermediate_rep.acid.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "sugar" => intermediate_rep.sugar.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "spectral" => intermediate_rep.spectral.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestMeasurementProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestMeasurementProperties {
            acid: intermediate_rep
                .acid
                .into_iter()
                .next()
                .ok_or("acid missing in HarvestMeasurementProperties".to_string())?,
            sugar: intermediate_rep
                .sugar
                .into_iter()
                .next()
                .ok_or("sugar missing in HarvestMeasurementProperties".to_string())?,
            spectral: intermediate_rep.spectral.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestMeasurementProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestMeasurementProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestMeasurementProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestMeasurementProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<HarvestMeasurementProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <HarvestMeasurementProperties as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into HarvestMeasurementProperties - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestOrTask {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// An array containing the measurements taken during the harvest task.
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<models::HarvestMeasurement>>,

    #[serde(rename = "geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<models::Polygon>,

    #[serde(rename = "properties")]
    pub properties: models::HarvestProperties,
}

impl HarvestOrTask {
    pub fn new(type_: String, properties: models::HarvestProperties) -> HarvestOrTask {
        HarvestOrTask {
            type_: type_,
            features: None,
            geometry: None,
            properties: properties,
        }
    }
}

/// Converts the HarvestOrTask value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestOrTask {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping features in query parameter serialization

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestOrTask value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestOrTask {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub features: Vec<Vec<models::HarvestMeasurement>>,
            pub geometry: Vec<models::Polygon>,
            pub properties: Vec<models::HarvestProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestOrTask".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "features" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in HarvestOrTask"
                                .to_string(),
                        )
                    }
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Polygon as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::HarvestProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestOrTask".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestOrTask {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in HarvestOrTask".to_string())?,
            features: intermediate_rep.features.into_iter().next(),
            geometry: intermediate_rep.geometry.into_iter().next(),
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in HarvestOrTask".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestOrTask> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestOrTask>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestOrTask>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestOrTask - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<HarvestOrTask> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HarvestOrTask as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HarvestOrTask - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestProperties {
    /// The numeric ID of the Harvest Task
    #[serde(rename = "task_id")]
    pub task_id: i64,

    /// Name of the task
    #[serde(rename = "task_name")]
    pub task_name: String,

    /// Name of the customer
    #[serde(rename = "customer_name")]
    pub customer_name: String,

    /// Name of the farm
    #[serde(rename = "farm_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_name: Option<String>,

    /// Name of the fruit group
    #[serde(rename = "fruit_group_name")]
    pub fruit_group_name: String,

    /// Name of the specific fruit
    #[serde(rename = "fruit_name")]
    pub fruit_name: String,

    /// Name of the plains
    #[serde(rename = "plain_name")]
    pub plain_name: String,

    /// Size of the plains in square meter
    #[serde(rename = "plain_size")]
    pub plain_size: i64,

    /// The mass of the harvest yield in kilogram
    #[serde(rename = "yield_total_mass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_total_mass: Option<i64>,

    /// The total area of the harvest task in square meter
    #[serde(rename = "total_area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_area: Option<i64>,

    /// The effective distance travelled by the harvester in meter
    #[serde(rename = "effective_total_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_distance: Option<i64>,

    /// The effective total time of the harvesting run in minutes
    #[serde(rename = "effective_total_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_time: Option<i64>,

    /// Average tartaric acid content of the harvested fruits in g/L
    #[serde(rename = "average_tartaric_acid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_tartaric_acid: Option<i64>,

    /// Average malic acid content of the harvested fruits in g/L
    #[serde(rename = "average_malic_acid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_malic_acid: Option<i64>,

    /// Average glucose content of the harvested fruits in g/L
    #[serde(rename = "average_glucose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_glucose: Option<i64>,

    /// Average fructose content of the harvested fruits in g/L
    #[serde(rename = "average_fructose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_fructose: Option<i64>,
}

impl HarvestProperties {
    pub fn new(
        task_id: i64,
        task_name: String,
        customer_name: String,
        fruit_group_name: String,
        fruit_name: String,
        plain_name: String,
        plain_size: i64,
    ) -> HarvestProperties {
        HarvestProperties {
            task_id: task_id,
            task_name: task_name,
            customer_name: customer_name,
            farm_name: None,
            fruit_group_name: fruit_group_name,
            fruit_name: fruit_name,
            plain_name: plain_name,
            plain_size: plain_size,
            yield_total_mass: None,
            total_area: None,
            effective_total_distance: None,
            effective_total_time: None,
            average_tartaric_acid: None,
            average_malic_acid: None,
            average_glucose: None,
            average_fructose: None,
        }
    }
}

/// Converts the HarvestProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("task_id".to_string());
        params.push(self.task_id.to_string());

        params.push("task_name".to_string());
        params.push(self.task_name.to_string());

        params.push("customer_name".to_string());
        params.push(self.customer_name.to_string());

        if let Some(ref farm_name) = self.farm_name {
            params.push("farm_name".to_string());
            params.push(farm_name.to_string());
        }

        params.push("fruit_group_name".to_string());
        params.push(self.fruit_group_name.to_string());

        params.push("fruit_name".to_string());
        params.push(self.fruit_name.to_string());

        params.push("plain_name".to_string());
        params.push(self.plain_name.to_string());

        params.push("plain_size".to_string());
        params.push(self.plain_size.to_string());

        if let Some(ref yield_total_mass) = self.yield_total_mass {
            params.push("yield_total_mass".to_string());
            params.push(yield_total_mass.to_string());
        }

        if let Some(ref total_area) = self.total_area {
            params.push("total_area".to_string());
            params.push(total_area.to_string());
        }

        if let Some(ref effective_total_distance) = self.effective_total_distance {
            params.push("effective_total_distance".to_string());
            params.push(effective_total_distance.to_string());
        }

        if let Some(ref effective_total_time) = self.effective_total_time {
            params.push("effective_total_time".to_string());
            params.push(effective_total_time.to_string());
        }

        if let Some(ref average_tartaric_acid) = self.average_tartaric_acid {
            params.push("average_tartaric_acid".to_string());
            params.push(average_tartaric_acid.to_string());
        }

        if let Some(ref average_malic_acid) = self.average_malic_acid {
            params.push("average_malic_acid".to_string());
            params.push(average_malic_acid.to_string());
        }

        if let Some(ref average_glucose) = self.average_glucose {
            params.push("average_glucose".to_string());
            params.push(average_glucose.to_string());
        }

        if let Some(ref average_fructose) = self.average_fructose {
            params.push("average_fructose".to_string());
            params.push(average_fructose.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub task_id: Vec<i64>,
            pub task_name: Vec<String>,
            pub customer_name: Vec<String>,
            pub farm_name: Vec<String>,
            pub fruit_group_name: Vec<String>,
            pub fruit_name: Vec<String>,
            pub plain_name: Vec<String>,
            pub plain_size: Vec<i64>,
            pub yield_total_mass: Vec<i64>,
            pub total_area: Vec<i64>,
            pub effective_total_distance: Vec<i64>,
            pub effective_total_time: Vec<i64>,
            pub average_tartaric_acid: Vec<i64>,
            pub average_malic_acid: Vec<i64>,
            pub average_glucose: Vec<i64>,
            pub average_fructose: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "task_id" => intermediate_rep.task_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "task_name" => intermediate_rep.task_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "customer_name" => intermediate_rep.customer_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "farm_name" => intermediate_rep.farm_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_group_name" => intermediate_rep.fruit_group_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_name" => intermediate_rep.fruit_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_name" => intermediate_rep.plain_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_size" => intermediate_rep.plain_size.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "yield_total_mass" => intermediate_rep.yield_total_mass.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "total_area" => intermediate_rep.total_area.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_distance" => intermediate_rep.effective_total_distance.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_time" => intermediate_rep.effective_total_time.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "average_tartaric_acid" => intermediate_rep.average_tartaric_acid.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "average_malic_acid" => intermediate_rep.average_malic_acid.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "average_glucose" => intermediate_rep.average_glucose.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "average_fructose" => intermediate_rep.average_fructose.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestProperties {
            task_id: intermediate_rep
                .task_id
                .into_iter()
                .next()
                .ok_or("task_id missing in HarvestProperties".to_string())?,
            task_name: intermediate_rep
                .task_name
                .into_iter()
                .next()
                .ok_or("task_name missing in HarvestProperties".to_string())?,
            customer_name: intermediate_rep
                .customer_name
                .into_iter()
                .next()
                .ok_or("customer_name missing in HarvestProperties".to_string())?,
            farm_name: intermediate_rep.farm_name.into_iter().next(),
            fruit_group_name: intermediate_rep
                .fruit_group_name
                .into_iter()
                .next()
                .ok_or("fruit_group_name missing in HarvestProperties".to_string())?,
            fruit_name: intermediate_rep
                .fruit_name
                .into_iter()
                .next()
                .ok_or("fruit_name missing in HarvestProperties".to_string())?,
            plain_name: intermediate_rep
                .plain_name
                .into_iter()
                .next()
                .ok_or("plain_name missing in HarvestProperties".to_string())?,
            plain_size: intermediate_rep
                .plain_size
                .into_iter()
                .next()
                .ok_or("plain_size missing in HarvestProperties".to_string())?,
            yield_total_mass: intermediate_rep.yield_total_mass.into_iter().next(),
            total_area: intermediate_rep.total_area.into_iter().next(),
            effective_total_distance: intermediate_rep.effective_total_distance.into_iter().next(),
            effective_total_time: intermediate_rep.effective_total_time.into_iter().next(),
            average_tartaric_acid: intermediate_rep.average_tartaric_acid.into_iter().next(),
            average_malic_acid: intermediate_rep.average_malic_acid.into_iter().next(),
            average_glucose: intermediate_rep.average_glucose.into_iter().next(),
            average_fructose: intermediate_rep.average_fructose.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<HarvestProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HarvestProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HarvestProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestTask {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::Polygon,

    #[serde(rename = "properties")]
    pub properties: models::HarvestTaskProperties,
}

impl HarvestTask {
    pub fn new(
        type_: String,
        geometry: models::Polygon,
        properties: models::HarvestTaskProperties,
    ) -> HarvestTask {
        HarvestTask {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the HarvestTask value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestTask {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestTask value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestTask {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::Polygon>,
            pub properties: Vec<models::HarvestTaskProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestTask".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Polygon as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::HarvestTaskProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestTask".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestTask {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in HarvestTask".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in HarvestTask".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in HarvestTask".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestTask> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestTask>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestTask>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestTask - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<HarvestTask> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HarvestTask as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HarvestTask - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HarvestTaskProperties {
    /// The numeric ID of the Harvest Task
    #[serde(rename = "task_id")]
    pub task_id: i64,

    /// Name of the task
    #[serde(rename = "task_name")]
    pub task_name: String,

    /// Name of the customer
    #[serde(rename = "customer_name")]
    pub customer_name: String,

    /// Name of the farm
    #[serde(rename = "farm_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_name: Option<String>,

    /// Name of the fruit group
    #[serde(rename = "fruit_group_name")]
    pub fruit_group_name: String,

    /// Name of the specific fruit
    #[serde(rename = "fruit_name")]
    pub fruit_name: String,

    /// Name of the plains
    #[serde(rename = "plain_name")]
    pub plain_name: String,

    /// Size of the plains in square meter
    #[serde(rename = "plain_size")]
    pub plain_size: i64,

    /// The mass of the harvest yield in kilogram
    #[serde(rename = "yield_total_mass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_total_mass: Option<i64>,

    /// The total area of the harvest task in square meter
    #[serde(rename = "total_area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_area: Option<i64>,

    /// The effective distance travelled by the harvester in meter
    #[serde(rename = "effective_total_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_distance: Option<i64>,

    /// The effective total time of the harvesting run in minutes
    #[serde(rename = "effective_total_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_time: Option<i64>,
}

impl HarvestTaskProperties {
    pub fn new(
        task_id: i64,
        task_name: String,
        customer_name: String,
        fruit_group_name: String,
        fruit_name: String,
        plain_name: String,
        plain_size: i64,
    ) -> HarvestTaskProperties {
        HarvestTaskProperties {
            task_id: task_id,
            task_name: task_name,
            customer_name: customer_name,
            farm_name: None,
            fruit_group_name: fruit_group_name,
            fruit_name: fruit_name,
            plain_name: plain_name,
            plain_size: plain_size,
            yield_total_mass: None,
            total_area: None,
            effective_total_distance: None,
            effective_total_time: None,
        }
    }
}

/// Converts the HarvestTaskProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HarvestTaskProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("task_id".to_string());
        params.push(self.task_id.to_string());

        params.push("task_name".to_string());
        params.push(self.task_name.to_string());

        params.push("customer_name".to_string());
        params.push(self.customer_name.to_string());

        if let Some(ref farm_name) = self.farm_name {
            params.push("farm_name".to_string());
            params.push(farm_name.to_string());
        }

        params.push("fruit_group_name".to_string());
        params.push(self.fruit_group_name.to_string());

        params.push("fruit_name".to_string());
        params.push(self.fruit_name.to_string());

        params.push("plain_name".to_string());
        params.push(self.plain_name.to_string());

        params.push("plain_size".to_string());
        params.push(self.plain_size.to_string());

        if let Some(ref yield_total_mass) = self.yield_total_mass {
            params.push("yield_total_mass".to_string());
            params.push(yield_total_mass.to_string());
        }

        if let Some(ref total_area) = self.total_area {
            params.push("total_area".to_string());
            params.push(total_area.to_string());
        }

        if let Some(ref effective_total_distance) = self.effective_total_distance {
            params.push("effective_total_distance".to_string());
            params.push(effective_total_distance.to_string());
        }

        if let Some(ref effective_total_time) = self.effective_total_time {
            params.push("effective_total_time".to_string());
            params.push(effective_total_time.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HarvestTaskProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HarvestTaskProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub task_id: Vec<i64>,
            pub task_name: Vec<String>,
            pub customer_name: Vec<String>,
            pub farm_name: Vec<String>,
            pub fruit_group_name: Vec<String>,
            pub fruit_name: Vec<String>,
            pub plain_name: Vec<String>,
            pub plain_size: Vec<i64>,
            pub yield_total_mass: Vec<i64>,
            pub total_area: Vec<i64>,
            pub effective_total_distance: Vec<i64>,
            pub effective_total_time: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HarvestTaskProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "task_id" => intermediate_rep.task_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "task_name" => intermediate_rep.task_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "customer_name" => intermediate_rep.customer_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "farm_name" => intermediate_rep.farm_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_group_name" => intermediate_rep.fruit_group_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_name" => intermediate_rep.fruit_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_name" => intermediate_rep.plain_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_size" => intermediate_rep.plain_size.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "yield_total_mass" => intermediate_rep.yield_total_mass.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "total_area" => intermediate_rep.total_area.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_distance" => intermediate_rep.effective_total_distance.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_time" => intermediate_rep.effective_total_time.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HarvestTaskProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HarvestTaskProperties {
            task_id: intermediate_rep
                .task_id
                .into_iter()
                .next()
                .ok_or("task_id missing in HarvestTaskProperties".to_string())?,
            task_name: intermediate_rep
                .task_name
                .into_iter()
                .next()
                .ok_or("task_name missing in HarvestTaskProperties".to_string())?,
            customer_name: intermediate_rep
                .customer_name
                .into_iter()
                .next()
                .ok_or("customer_name missing in HarvestTaskProperties".to_string())?,
            farm_name: intermediate_rep.farm_name.into_iter().next(),
            fruit_group_name: intermediate_rep
                .fruit_group_name
                .into_iter()
                .next()
                .ok_or("fruit_group_name missing in HarvestTaskProperties".to_string())?,
            fruit_name: intermediate_rep
                .fruit_name
                .into_iter()
                .next()
                .ok_or("fruit_name missing in HarvestTaskProperties".to_string())?,
            plain_name: intermediate_rep
                .plain_name
                .into_iter()
                .next()
                .ok_or("plain_name missing in HarvestTaskProperties".to_string())?,
            plain_size: intermediate_rep
                .plain_size
                .into_iter()
                .next()
                .ok_or("plain_size missing in HarvestTaskProperties".to_string())?,
            yield_total_mass: intermediate_rep.yield_total_mass.into_iter().next(),
            total_area: intermediate_rep.total_area.into_iter().next(),
            effective_total_distance: intermediate_rep.effective_total_distance.into_iter().next(),
            effective_total_time: intermediate_rep.effective_total_time.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HarvestTaskProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HarvestTaskProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HarvestTaskProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HarvestTaskProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<HarvestTaskProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HarvestTaskProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HarvestTaskProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse201 {
    /// Numeric ID of the created Forecast.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse201 {
    pub fn new() -> InlineResponse201 {
        InlineResponse201 { id: None }
    }
}

/// Converts the InlineResponse201 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse201 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse201 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse201 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse201".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse201".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse201 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse201> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse201>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse201>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse201 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse201>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse201 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse201 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2011 {
    /// Numeric ID of the created Field.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2011 {
    pub fn new() -> InlineResponse2011 {
        InlineResponse2011 { id: None }
    }
}

/// Converts the InlineResponse2011 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2011 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2011 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2011 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2011".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2011".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2011 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2011> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2011>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2011>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2011 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2011>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2011 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2011 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2012 {
    /// Numeric ID of the created Planter Run.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2012 {
    pub fn new() -> InlineResponse2012 {
        InlineResponse2012 { id: None }
    }
}

/// Converts the InlineResponse2012 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2012 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2012 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2012 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2012".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2012".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2012 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2012> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2012>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2012>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2012 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2012>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2012 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2012 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2013 {
    /// Numeric ID of the created Plant.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2013 {
    pub fn new() -> InlineResponse2013 {
        InlineResponse2013 { id: None }
    }
}

/// Converts the InlineResponse2013 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2013 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2013 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2013 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2013".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2013".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2013 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2013> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2013>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2013>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2013 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2013>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2013 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2013 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2014 {
    /// Numeric ID of the created Quality Measurement.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2014 {
    pub fn new() -> InlineResponse2014 {
        InlineResponse2014 { id: None }
    }
}

/// Converts the InlineResponse2014 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2014 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2014 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2014 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2014".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2014".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2014 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2014> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2014>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2014>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2014 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2014>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2014 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2014 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2015 {
    /// Numeric ID of the created Harvest Task.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2015 {
    pub fn new() -> InlineResponse2015 {
        InlineResponse2015 { id: None }
    }
}

/// Converts the InlineResponse2015 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2015 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2015 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2015 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2015".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2015".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2015 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2015> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2015>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2015>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2015 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2015>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2015 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2015 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2016 {
    /// Numeric ID of the created Spray Task.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl InlineResponse2016 {
    pub fn new() -> InlineResponse2016 {
        InlineResponse2016 { id: None }
    }
}

/// Converts the InlineResponse2016 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2016 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2016 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2016 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2016".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2016".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2016 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2016> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2016>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2016>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2016 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2016>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2016 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2016 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2017 {
    /// Numeric ID of the created Cultivar.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl InlineResponse2017 {
    pub fn new() -> InlineResponse2017 {
        InlineResponse2017 { id: None }
    }
}

/// Converts the InlineResponse2017 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2017 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2017 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2017 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2017".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2017".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2017 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2017> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2017>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2017>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2017 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2017>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2017 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2017 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2018 {
    /// Numeric ID of the created Weather Station.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl InlineResponse2018 {
    pub fn new() -> InlineResponse2018 {
        InlineResponse2018 { id: None }
    }
}

/// Converts the InlineResponse2018 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2018 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2018 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2018 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2018".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2018".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2018 {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2018> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2018>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2018>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2018 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2018>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2018 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2018 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2019 {
    /// The ID of the weather station.
    #[serde(rename = "station_id")]
    pub station_id: i32,

    /// The reference date of the weather data, following the format specified by RFC 3339.
    #[serde(rename = "measurement_date")]
    pub measurement_date: String,
}

impl InlineResponse2019 {
    pub fn new(station_id: i32, measurement_date: String) -> InlineResponse2019 {
        InlineResponse2019 {
            station_id: station_id,
            measurement_date: measurement_date,
        }
    }
}

/// Converts the InlineResponse2019 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2019 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("station_id".to_string());
        params.push(self.station_id.to_string());

        params.push("measurement_date".to_string());
        params.push(self.measurement_date.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2019 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2019 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub station_id: Vec<i32>,
            pub measurement_date: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse2019".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "station_id" => intermediate_rep.station_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "measurement_date" => intermediate_rep.measurement_date.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse2019".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2019 {
            station_id: intermediate_rep
                .station_id
                .into_iter()
                .next()
                .ok_or("station_id missing in InlineResponse2019".to_string())?,
            measurement_date: intermediate_rep
                .measurement_date
                .into_iter()
                .next()
                .ok_or("measurement_date missing in InlineResponse2019".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2019> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2019>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse2019>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse2019 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse2019>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse2019 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse2019 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse400 {
    #[serde(rename = "error")]
    pub error: String,
}

impl InlineResponse400 {
    pub fn new(error: String) -> InlineResponse400 {
        InlineResponse400 { error: error }
    }
}

/// Converts the InlineResponse400 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse400 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("error".to_string());
        params.push(self.error.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse400 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse400 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InlineResponse400".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "error" => intermediate_rep.error.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InlineResponse400".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse400 {
            error: intermediate_rep
                .error
                .into_iter()
                .next()
                .ok_or("error missing in InlineResponse400".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse400> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse400>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InlineResponse400>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InlineResponse400 - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<InlineResponse400>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InlineResponse400 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InlineResponse400 - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultiPointWithHeight {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. The altitude is given in meter. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most southwesterly point: Altitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude   - The most northeasterly point: Altitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// An array of Points with its coordinates and altitudes, given in the decimal degrees unit. The altitude/height is given in meter. In the following it is described, which value of the array represents which coordinate:   - Longitude   - Latitude   - Altitude
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<Vec<f64>>,
}

impl MultiPointWithHeight {
    pub fn new(type_: String, coordinates: Vec<Vec<f64>>) -> MultiPointWithHeight {
        MultiPointWithHeight {
            type_: type_,
            bbox: None,
            coordinates: coordinates,
        }
    }
}

/// Converts the MultiPointWithHeight value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultiPointWithHeight {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        // Skipping coordinates in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultiPointWithHeight value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultiPointWithHeight {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub bbox: Vec<Vec<f64>>,
            pub coordinates: Vec<Vec<Vec<f64>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MultiPointWithHeight".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "bbox" => return std::result::Result::Err("Parsing a container in this style is not supported in MultiPointWithHeight".to_string()),
                    "coordinates" => return std::result::Result::Err("Parsing a container in this style is not supported in MultiPointWithHeight".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MultiPointWithHeight".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultiPointWithHeight {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in MultiPointWithHeight".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            coordinates: intermediate_rep
                .coordinates
                .into_iter()
                .next()
                .ok_or("coordinates missing in MultiPointWithHeight".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultiPointWithHeight> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultiPointWithHeight>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MultiPointWithHeight>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MultiPointWithHeight - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<MultiPointWithHeight>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MultiPointWithHeight as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MultiPointWithHeight - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultiPolygon {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// A Polygon is a surface specified by its vertices. The vertices are coordinates, given in the decimal degrees unit. Note the following:   - The surface vertices are given counterclockwise.   - Holes in the surface are given clockwise.   - The last and the first coordinates must be equal. In the following it is described, which value of the innermost array represents which coordinate:   - Longitude   - Latitude The outermost array includes the various Polygons. A example, given in the [RFC7946 - The GeoJSON format](https://tools.ietf.org/html/rfc7946#appendix-A.6)
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

impl MultiPolygon {
    pub fn new(type_: String, coordinates: Vec<Vec<Vec<Vec<f64>>>>) -> MultiPolygon {
        MultiPolygon {
            type_: type_,
            bbox: None,
            coordinates: coordinates,
        }
    }
}

/// Converts the MultiPolygon value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultiPolygon {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        // Skipping coordinates in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultiPolygon value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultiPolygon {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub bbox: Vec<Vec<f64>>,
            pub coordinates: Vec<Vec<Vec<Vec<Vec<f64>>>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MultiPolygon".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "bbox" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in MultiPolygon"
                                .to_string(),
                        )
                    }
                    "coordinates" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in MultiPolygon"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MultiPolygon".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultiPolygon {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in MultiPolygon".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            coordinates: intermediate_rep
                .coordinates
                .into_iter()
                .next()
                .ok_or("coordinates missing in MultiPolygon".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultiPolygon> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultiPolygon>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MultiPolygon>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MultiPolygon - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultiPolygon> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MultiPolygon as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MultiPolygon - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Plant {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::Point,

    #[serde(rename = "properties")]
    pub properties: models::PlantProperties,
}

impl Plant {
    pub fn new(
        type_: String,
        geometry: models::Point,
        properties: models::PlantProperties,
    ) -> Plant {
        Plant {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the Plant value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Plant {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Plant value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Plant {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::Point>,
            pub properties: Vec<models::PlantProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Plant".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Point as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::PlantProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Plant".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Plant {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Plant".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in Plant".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in Plant".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Plant> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Plant>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Plant>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Plant - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Plant> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Plant as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Plant - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlantPatch {
    /// Internal key to facilitate error tracing
    #[serde(rename = "import_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_key: Option<String>,

    /// Timestamp when this plant was planted, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    /// Height above sea level, measured in millimeters
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    #[serde(rename = "deviation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deviation: Option<f64>,

    /// Planting speed measured in km/h
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    /// Distance between two Plants in a row, measured in millimeters
    #[serde(rename = "plant_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_distance: Option<f64>,

    /// Distance between two rows, measured in millimeters
    #[serde(rename = "row_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_distance: Option<f64>,

    /// Number of the row of the Plant
    #[serde(rename = "row_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_number: Option<i32>,
}

impl PlantPatch {
    pub fn new() -> PlantPatch {
        PlantPatch {
            import_key: None,
            timestamp: None,
            height: None,
            deviation: None,
            speed: None,
            plant_distance: None,
            row_distance: None,
            row_number: None,
        }
    }
}

/// Converts the PlantPatch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlantPatch {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref import_key) = self.import_key {
            params.push("import_key".to_string());
            params.push(import_key.to_string());
        }

        // Skipping timestamp in query parameter serialization

        if let Some(ref height) = self.height {
            params.push("height".to_string());
            params.push(height.to_string());
        }

        if let Some(ref deviation) = self.deviation {
            params.push("deviation".to_string());
            params.push(deviation.to_string());
        }

        if let Some(ref speed) = self.speed {
            params.push("speed".to_string());
            params.push(speed.to_string());
        }

        if let Some(ref plant_distance) = self.plant_distance {
            params.push("plant_distance".to_string());
            params.push(plant_distance.to_string());
        }

        if let Some(ref row_distance) = self.row_distance {
            params.push("row_distance".to_string());
            params.push(row_distance.to_string());
        }

        if let Some(ref row_number) = self.row_number {
            params.push("row_number".to_string());
            params.push(row_number.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlantPatch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlantPatch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub import_key: Vec<String>,
            pub timestamp: Vec<chrono::DateTime<chrono::Utc>>,
            pub height: Vec<f64>,
            pub deviation: Vec<f64>,
            pub speed: Vec<f64>,
            pub plant_distance: Vec<f64>,
            pub row_distance: Vec<f64>,
            pub row_number: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlantPatch".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "import_key" => intermediate_rep.import_key.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "timestamp" => intermediate_rep.timestamp.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "height" => intermediate_rep.height.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "deviation" => intermediate_rep.deviation.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "speed" => intermediate_rep.speed.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "plant_distance" => intermediate_rep.plant_distance.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "row_distance" => intermediate_rep.row_distance.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "row_number" => intermediate_rep.row_number.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlantPatch".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlantPatch {
            import_key: intermediate_rep.import_key.into_iter().next(),
            timestamp: intermediate_rep.timestamp.into_iter().next(),
            height: intermediate_rep.height.into_iter().next(),
            deviation: intermediate_rep.deviation.into_iter().next(),
            speed: intermediate_rep.speed.into_iter().next(),
            plant_distance: intermediate_rep.plant_distance.into_iter().next(),
            row_distance: intermediate_rep.row_distance.into_iter().next(),
            row_number: intermediate_rep.row_number.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlantPatch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlantPatch>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlantPatch>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PlantPatch - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PlantPatch> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlantPatch as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PlantPatch - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlantProperties {
    /// The numeric ID of the Plant
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Internal key to facilitate error tracing
    #[serde(rename = "import_key")]
    pub import_key: String,

    /// Timestamp when this plant was planted, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Height above sea level, measured in millimeters
    #[serde(rename = "height")]
    pub height: f64,

    #[serde(rename = "deviation")]
    pub deviation: f64,

    /// Planting speed measured in km/h
    #[serde(rename = "speed")]
    pub speed: f64,

    /// Distance between two Plants in a row, measured in millimeters
    #[serde(rename = "plant_distance")]
    pub plant_distance: f64,

    /// Distance between two rows, measured in millimeters
    #[serde(rename = "row_distance")]
    pub row_distance: f64,

    /// Number of the row of the Plant
    #[serde(rename = "row_number")]
    pub row_number: i32,
}

impl PlantProperties {
    pub fn new(
        import_key: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        height: f64,
        deviation: f64,
        speed: f64,
        plant_distance: f64,
        row_distance: f64,
        row_number: i32,
    ) -> PlantProperties {
        PlantProperties {
            id: None,
            import_key: import_key,
            timestamp: timestamp,
            height: height,
            deviation: deviation,
            speed: speed,
            plant_distance: plant_distance,
            row_distance: row_distance,
            row_number: row_number,
        }
    }
}

/// Converts the PlantProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlantProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.push("import_key".to_string());
        params.push(self.import_key.to_string());

        // Skipping timestamp in query parameter serialization

        params.push("height".to_string());
        params.push(self.height.to_string());

        params.push("deviation".to_string());
        params.push(self.deviation.to_string());

        params.push("speed".to_string());
        params.push(self.speed.to_string());

        params.push("plant_distance".to_string());
        params.push(self.plant_distance.to_string());

        params.push("row_distance".to_string());
        params.push(self.row_distance.to_string());

        params.push("row_number".to_string());
        params.push(self.row_number.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlantProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlantProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub import_key: Vec<String>,
            pub timestamp: Vec<chrono::DateTime<chrono::Utc>>,
            pub height: Vec<f64>,
            pub deviation: Vec<f64>,
            pub speed: Vec<f64>,
            pub plant_distance: Vec<f64>,
            pub row_distance: Vec<f64>,
            pub row_number: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlantProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "import_key" => intermediate_rep.import_key.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "timestamp" => intermediate_rep.timestamp.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "height" => intermediate_rep.height.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "deviation" => intermediate_rep.deviation.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "speed" => intermediate_rep.speed.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "plant_distance" => intermediate_rep.plant_distance.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "row_distance" => intermediate_rep.row_distance.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "row_number" => intermediate_rep.row_number.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlantProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlantProperties {
            id: intermediate_rep.id.into_iter().next(),
            import_key: intermediate_rep
                .import_key
                .into_iter()
                .next()
                .ok_or("import_key missing in PlantProperties".to_string())?,
            timestamp: intermediate_rep
                .timestamp
                .into_iter()
                .next()
                .ok_or("timestamp missing in PlantProperties".to_string())?,
            height: intermediate_rep
                .height
                .into_iter()
                .next()
                .ok_or("height missing in PlantProperties".to_string())?,
            deviation: intermediate_rep
                .deviation
                .into_iter()
                .next()
                .ok_or("deviation missing in PlantProperties".to_string())?,
            speed: intermediate_rep
                .speed
                .into_iter()
                .next()
                .ok_or("speed missing in PlantProperties".to_string())?,
            plant_distance: intermediate_rep
                .plant_distance
                .into_iter()
                .next()
                .ok_or("plant_distance missing in PlantProperties".to_string())?,
            row_distance: intermediate_rep
                .row_distance
                .into_iter()
                .next()
                .ok_or("row_distance missing in PlantProperties".to_string())?,
            row_number: intermediate_rep
                .row_number
                .into_iter()
                .next()
                .ok_or("row_number missing in PlantProperties".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlantProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlantProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlantProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PlantProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<PlantProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlantProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PlantProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlanterRun {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// An array containing the planted Plants of this Planter Run.
    #[serde(rename = "features")]
    pub features: Vec<models::Plant>,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    #[serde(rename = "properties")]
    pub properties: models::PlanterRunProperties,
}

impl PlanterRun {
    pub fn new(
        type_: String,
        features: Vec<models::Plant>,
        properties: models::PlanterRunProperties,
    ) -> PlanterRun {
        PlanterRun {
            type_: type_,
            features: features,
            bbox: None,
            properties: properties,
        }
    }
}

/// Converts the PlanterRun value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlanterRun {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping features in query parameter serialization

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlanterRun value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlanterRun {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub features: Vec<Vec<models::Plant>>,
            pub bbox: Vec<Vec<f64>>,
            pub properties: Vec<models::PlanterRunProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlanterRun".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "features" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in PlanterRun"
                                .to_string(),
                        )
                    }
                    "bbox" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in PlanterRun"
                                .to_string(),
                        )
                    }
                    "properties" => intermediate_rep.properties.push(
                        <models::PlanterRunProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlanterRun".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlanterRun {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in PlanterRun".to_string())?,
            features: intermediate_rep
                .features
                .into_iter()
                .next()
                .ok_or("features missing in PlanterRun".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in PlanterRun".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlanterRun> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlanterRun>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlanterRun>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PlanterRun - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PlanterRun> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlanterRun as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PlanterRun - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlanterRunProperties {
    /// The numeric ID of the Planter Run
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The numeric ID of the User who planted
    #[serde(rename = "user_id")]
    pub user_id: i64,

    /// The numeric ID of the Terminal of the Machine that planted
    #[serde(rename = "terminal_id")]
    pub terminal_id: i64,

    /// The numeric ID of the Field on which the Planter Run is performed.
    #[serde(rename = "field_id")]
    pub field_id: i64,
}

impl PlanterRunProperties {
    pub fn new(user_id: i64, terminal_id: i64, field_id: i64) -> PlanterRunProperties {
        PlanterRunProperties {
            id: None,
            user_id: user_id,
            terminal_id: terminal_id,
            field_id: field_id,
        }
    }
}

/// Converts the PlanterRunProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlanterRunProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.push("user_id".to_string());
        params.push(self.user_id.to_string());

        params.push("terminal_id".to_string());
        params.push(self.terminal_id.to_string());

        params.push("field_id".to_string());
        params.push(self.field_id.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlanterRunProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlanterRunProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub user_id: Vec<i64>,
            pub terminal_id: Vec<i64>,
            pub field_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlanterRunProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "user_id" => intermediate_rep.user_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "terminal_id" => intermediate_rep.terminal_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "field_id" => intermediate_rep.field_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlanterRunProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlanterRunProperties {
            id: intermediate_rep.id.into_iter().next(),
            user_id: intermediate_rep
                .user_id
                .into_iter()
                .next()
                .ok_or("user_id missing in PlanterRunProperties".to_string())?,
            terminal_id: intermediate_rep
                .terminal_id
                .into_iter()
                .next()
                .ok_or("terminal_id missing in PlanterRunProperties".to_string())?,
            field_id: intermediate_rep
                .field_id
                .into_iter()
                .next()
                .ok_or("field_id missing in PlanterRunProperties".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlanterRunProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlanterRunProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlanterRunProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PlanterRunProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<PlanterRunProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlanterRunProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PlanterRunProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Point {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// A Point is described by its coordinate, given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - Longitude   - Latitude  A example, given in the [RFC7946 - The GeoJSON format](https://tools.ietf.org/html/rfc7946#appendix-A.1)
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<f64>,
}

impl Point {
    pub fn new(type_: String, coordinates: Vec<f64>) -> Point {
        Point {
            type_: type_,
            bbox: None,
            coordinates: coordinates,
        }
    }
}

/// Converts the Point value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Point {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        params.push("coordinates".to_string());
        params.push(
            self.coordinates
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .to_string(),
        );

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Point value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub bbox: Vec<Vec<f64>>,
            pub coordinates: Vec<Vec<f64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Point".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "bbox" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Point"
                                .to_string(),
                        )
                    }
                    "coordinates" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Point"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Point".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Point {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Point".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            coordinates: intermediate_rep
                .coordinates
                .into_iter()
                .next()
                .ok_or("coordinates missing in Point".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Point> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Point>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Point>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Point - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Point> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Point as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Point - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PointWithHeight {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. The altitude is given in meter. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most southwesterly point: Altitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude   - The most northeasterly point: Altitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// A Point with its height is described by its coordinate, given in the decimal degrees unit, and its altitude, given in meter. In the following it is described, which value of the array represents which coordinate:   - Longitude   - Latitude   - Altitude
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<f64>,
}

impl PointWithHeight {
    pub fn new(type_: String, coordinates: Vec<f64>) -> PointWithHeight {
        PointWithHeight {
            type_: type_,
            bbox: None,
            coordinates: coordinates,
        }
    }
}

/// Converts the PointWithHeight value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PointWithHeight {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        params.push("coordinates".to_string());
        params.push(
            self.coordinates
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .to_string(),
        );

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PointWithHeight value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PointWithHeight {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub bbox: Vec<Vec<f64>>,
            pub coordinates: Vec<Vec<f64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PointWithHeight".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "bbox" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in PointWithHeight"
                                .to_string(),
                        )
                    }
                    "coordinates" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in PointWithHeight"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PointWithHeight".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PointWithHeight {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in PointWithHeight".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            coordinates: intermediate_rep
                .coordinates
                .into_iter()
                .next()
                .ok_or("coordinates missing in PointWithHeight".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PointWithHeight> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PointWithHeight>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PointWithHeight>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PointWithHeight - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<PointWithHeight>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PointWithHeight as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PointWithHeight - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Polygon {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude
    #[serde(rename = "bbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// A Polygon is a surface specified by its vertices. The vertices are coordinates, given in the decimal degrees unit. Note the following:   - The surface vertices are given counterclockwise.   - Holes in the surface are given clockwise.   - The last and the first coordinates must be equal. In the following it is described, which value of the innermost array represents which coordinate:   - Longitude   - Latitude  A example, given in the [RFC7946 - The GeoJSON format](https://tools.ietf.org/html/rfc7946#appendix-A.3)
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

impl Polygon {
    pub fn new(type_: String, coordinates: Vec<Vec<Vec<f64>>>) -> Polygon {
        Polygon {
            type_: type_,
            bbox: None,
            coordinates: coordinates,
        }
    }
}

/// Converts the Polygon value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Polygon {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        if let Some(ref bbox) = self.bbox {
            params.push("bbox".to_string());
            params.push(
                bbox.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string(),
            );
        }

        // Skipping coordinates in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Polygon value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Polygon {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub bbox: Vec<Vec<f64>>,
            pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Polygon".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "bbox" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Polygon"
                                .to_string(),
                        )
                    }
                    "coordinates" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Polygon"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Polygon".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Polygon {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Polygon".to_string())?,
            bbox: intermediate_rep.bbox.into_iter().next(),
            coordinates: intermediate_rep
                .coordinates
                .into_iter()
                .next()
                .ok_or("coordinates missing in Polygon".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Polygon> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Polygon>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Polygon>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Polygon - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Polygon> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Polygon as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Polygon - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QualityLevel(i32);

impl std::convert::From<i32> for QualityLevel {
    fn from(x: i32) -> Self {
        QualityLevel(x)
    }
}

impl std::convert::From<QualityLevel> for i32 {
    fn from(x: QualityLevel) -> Self {
        x.0
    }
}

impl std::ops::Deref for QualityLevel {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for QualityLevel {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QualityMeasurement {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::Point,

    #[serde(rename = "properties")]
    pub properties: models::QualityMeasurementProperties,
}

impl QualityMeasurement {
    pub fn new(
        type_: String,
        geometry: models::Point,
        properties: models::QualityMeasurementProperties,
    ) -> QualityMeasurement {
        QualityMeasurement {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the QualityMeasurement value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for QualityMeasurement {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a QualityMeasurement value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for QualityMeasurement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::Point>,
            pub properties: Vec<models::QualityMeasurementProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing QualityMeasurement".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Point as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::QualityMeasurementProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing QualityMeasurement".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(QualityMeasurement {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in QualityMeasurement".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in QualityMeasurement".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in QualityMeasurement".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<QualityMeasurement> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<QualityMeasurement>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<QualityMeasurement>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for QualityMeasurement - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<QualityMeasurement>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <QualityMeasurement as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into QualityMeasurement - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct QualityMeasurementProperties {
    /// The numeric ID of the Quality Measurement
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The numeric ID of the Plant
    #[serde(rename = "plant_id")]
    pub plant_id: i64,

    /// Timestamp when the Measurement was taken, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time).
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Device id, needed for e.g. assigning the device to a customer/user
    #[serde(rename = "device")]
    pub device: String,

    /// A Universally Unique IDentifier (UUID) of the Measurement
    #[serde(rename = "measurement_uuid")]
    pub measurement_uuid: uuid::Uuid,

    /// The tartaric acid concentration, measured in g/L
    #[serde(rename = "tartaric_acid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tartaric_acid: Option<f64>,

    /// The malic acid concentration, measured in g/L
    #[serde(rename = "malic_acid")]
    pub malic_acid: f64,

    /// The fructose concentration, measured in g/L
    #[serde(rename = "fructose")]
    pub fructose: f64,

    /// The glucose concentration, measured in g/L
    #[serde(rename = "glucose")]
    pub glucose: f64,
}

impl QualityMeasurementProperties {
    pub fn new(
        plant_id: i64,
        timestamp: chrono::DateTime<chrono::Utc>,
        device: String,
        measurement_uuid: uuid::Uuid,
        malic_acid: f64,
        fructose: f64,
        glucose: f64,
    ) -> QualityMeasurementProperties {
        QualityMeasurementProperties {
            id: None,
            plant_id: plant_id,
            timestamp: timestamp,
            device: device,
            measurement_uuid: measurement_uuid,
            tartaric_acid: None,
            malic_acid: malic_acid,
            fructose: fructose,
            glucose: glucose,
        }
    }
}

/// Converts the QualityMeasurementProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for QualityMeasurementProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.push("plant_id".to_string());
        params.push(self.plant_id.to_string());

        // Skipping timestamp in query parameter serialization

        params.push("device".to_string());
        params.push(self.device.to_string());

        // Skipping measurement_uuid in query parameter serialization

        if let Some(ref tartaric_acid) = self.tartaric_acid {
            params.push("tartaric_acid".to_string());
            params.push(tartaric_acid.to_string());
        }

        params.push("malic_acid".to_string());
        params.push(self.malic_acid.to_string());

        params.push("fructose".to_string());
        params.push(self.fructose.to_string());

        params.push("glucose".to_string());
        params.push(self.glucose.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a QualityMeasurementProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for QualityMeasurementProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub plant_id: Vec<i64>,
            pub timestamp: Vec<chrono::DateTime<chrono::Utc>>,
            pub device: Vec<String>,
            pub measurement_uuid: Vec<uuid::Uuid>,
            pub tartaric_acid: Vec<f64>,
            pub malic_acid: Vec<f64>,
            pub fructose: Vec<f64>,
            pub glucose: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing QualityMeasurementProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "plant_id" => intermediate_rep.plant_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "timestamp" => intermediate_rep.timestamp.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "device" => intermediate_rep.device.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "measurement_uuid" => intermediate_rep.measurement_uuid.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tartaric_acid" => intermediate_rep.tartaric_acid.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "malic_acid" => intermediate_rep.malic_acid.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "fructose" => intermediate_rep.fructose.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "glucose" => intermediate_rep.glucose.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing QualityMeasurementProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(QualityMeasurementProperties {
            id: intermediate_rep.id.into_iter().next(),
            plant_id: intermediate_rep
                .plant_id
                .into_iter()
                .next()
                .ok_or("plant_id missing in QualityMeasurementProperties".to_string())?,
            timestamp: intermediate_rep
                .timestamp
                .into_iter()
                .next()
                .ok_or("timestamp missing in QualityMeasurementProperties".to_string())?,
            device: intermediate_rep
                .device
                .into_iter()
                .next()
                .ok_or("device missing in QualityMeasurementProperties".to_string())?,
            measurement_uuid: intermediate_rep
                .measurement_uuid
                .into_iter()
                .next()
                .ok_or("measurement_uuid missing in QualityMeasurementProperties".to_string())?,
            tartaric_acid: intermediate_rep.tartaric_acid.into_iter().next(),
            malic_acid: intermediate_rep
                .malic_acid
                .into_iter()
                .next()
                .ok_or("malic_acid missing in QualityMeasurementProperties".to_string())?,
            fructose: intermediate_rep
                .fructose
                .into_iter()
                .next()
                .ok_or("fructose missing in QualityMeasurementProperties".to_string())?,
            glucose: intermediate_rep
                .glucose
                .into_iter()
                .next()
                .ok_or("glucose missing in QualityMeasurementProperties".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<QualityMeasurementProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<QualityMeasurementProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<QualityMeasurementProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for QualityMeasurementProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<QualityMeasurementProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <QualityMeasurementProperties as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into QualityMeasurementProperties - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SprayTask {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::Polygon,

    #[serde(rename = "properties")]
    pub properties: models::SprayTaskProperties,
}

impl SprayTask {
    pub fn new(
        type_: String,
        geometry: models::Polygon,
        properties: models::SprayTaskProperties,
    ) -> SprayTask {
        SprayTask {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the SprayTask value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SprayTask {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SprayTask value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SprayTask {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<models::Polygon>,
            pub properties: Vec<models::SprayTaskProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SprayTask".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::Polygon as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::SprayTaskProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SprayTask".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SprayTask {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in SprayTask".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in SprayTask".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in SprayTask".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SprayTask> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SprayTask>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SprayTask>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SprayTask - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SprayTask> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SprayTask as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SprayTask - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SprayTaskProperties {
    /// The numeric ID of the Harvest Task
    #[serde(rename = "task_id")]
    pub task_id: i64,

    /// Name of the task
    #[serde(rename = "task_name")]
    pub task_name: String,

    /// Name of the customer
    #[serde(rename = "customer_name")]
    pub customer_name: String,

    /// Name of the farm
    #[serde(rename = "farm_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm_name: Option<String>,

    /// Name of the fruit group
    #[serde(rename = "fruit_group_name")]
    pub fruit_group_name: String,

    /// Name of the specific fruit
    #[serde(rename = "fruit_name")]
    pub fruit_name: String,

    /// Name of the plains
    #[serde(rename = "plain_name")]
    pub plain_name: String,

    /// Size of the plains in square meter
    #[serde(rename = "plain_size")]
    pub plain_size: i64,

    /// The total volume of protection sprayed
    #[serde(rename = "application_total_volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_total_volume: Option<i64>,

    /// The total area of the spray task in square meter
    #[serde(rename = "total_area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_area: Option<i64>,

    /// The effective distance travelled by the spray machine in meter
    #[serde(rename = "effective_total_distance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_distance: Option<i64>,

    /// The effective total time of the spray run in minutes
    #[serde(rename = "effective_total_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_total_time: Option<i64>,

    #[serde(rename = "plant_protection")]
    pub plant_protection: Vec<models::SprayTaskPropertiesPlantProtection>,
}

impl SprayTaskProperties {
    pub fn new(
        task_id: i64,
        task_name: String,
        customer_name: String,
        fruit_group_name: String,
        fruit_name: String,
        plain_name: String,
        plain_size: i64,
        plant_protection: Vec<models::SprayTaskPropertiesPlantProtection>,
    ) -> SprayTaskProperties {
        SprayTaskProperties {
            task_id: task_id,
            task_name: task_name,
            customer_name: customer_name,
            farm_name: None,
            fruit_group_name: fruit_group_name,
            fruit_name: fruit_name,
            plain_name: plain_name,
            plain_size: plain_size,
            application_total_volume: None,
            total_area: None,
            effective_total_distance: None,
            effective_total_time: None,
            plant_protection: plant_protection,
        }
    }
}

/// Converts the SprayTaskProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SprayTaskProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("task_id".to_string());
        params.push(self.task_id.to_string());

        params.push("task_name".to_string());
        params.push(self.task_name.to_string());

        params.push("customer_name".to_string());
        params.push(self.customer_name.to_string());

        if let Some(ref farm_name) = self.farm_name {
            params.push("farm_name".to_string());
            params.push(farm_name.to_string());
        }

        params.push("fruit_group_name".to_string());
        params.push(self.fruit_group_name.to_string());

        params.push("fruit_name".to_string());
        params.push(self.fruit_name.to_string());

        params.push("plain_name".to_string());
        params.push(self.plain_name.to_string());

        params.push("plain_size".to_string());
        params.push(self.plain_size.to_string());

        if let Some(ref application_total_volume) = self.application_total_volume {
            params.push("application_total_volume".to_string());
            params.push(application_total_volume.to_string());
        }

        if let Some(ref total_area) = self.total_area {
            params.push("total_area".to_string());
            params.push(total_area.to_string());
        }

        if let Some(ref effective_total_distance) = self.effective_total_distance {
            params.push("effective_total_distance".to_string());
            params.push(effective_total_distance.to_string());
        }

        if let Some(ref effective_total_time) = self.effective_total_time {
            params.push("effective_total_time".to_string());
            params.push(effective_total_time.to_string());
        }

        // Skipping plant_protection in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SprayTaskProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SprayTaskProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub task_id: Vec<i64>,
            pub task_name: Vec<String>,
            pub customer_name: Vec<String>,
            pub farm_name: Vec<String>,
            pub fruit_group_name: Vec<String>,
            pub fruit_name: Vec<String>,
            pub plain_name: Vec<String>,
            pub plain_size: Vec<i64>,
            pub application_total_volume: Vec<i64>,
            pub total_area: Vec<i64>,
            pub effective_total_distance: Vec<i64>,
            pub effective_total_time: Vec<i64>,
            pub plant_protection: Vec<Vec<models::SprayTaskPropertiesPlantProtection>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SprayTaskProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "task_id" => intermediate_rep.task_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "task_name" => intermediate_rep.task_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "customer_name" => intermediate_rep.customer_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "farm_name" => intermediate_rep.farm_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_group_name" => intermediate_rep.fruit_group_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fruit_name" => intermediate_rep.fruit_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_name" => intermediate_rep.plain_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "plain_size" => intermediate_rep.plain_size.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "application_total_volume" => intermediate_rep.application_total_volume.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "total_area" => intermediate_rep.total_area.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_distance" => intermediate_rep.effective_total_distance.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "effective_total_time" => intermediate_rep.effective_total_time.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "plant_protection" => return std::result::Result::Err(
                        "Parsing a container in this style is not supported in SprayTaskProperties"
                            .to_string(),
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SprayTaskProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SprayTaskProperties {
            task_id: intermediate_rep
                .task_id
                .into_iter()
                .next()
                .ok_or("task_id missing in SprayTaskProperties".to_string())?,
            task_name: intermediate_rep
                .task_name
                .into_iter()
                .next()
                .ok_or("task_name missing in SprayTaskProperties".to_string())?,
            customer_name: intermediate_rep
                .customer_name
                .into_iter()
                .next()
                .ok_or("customer_name missing in SprayTaskProperties".to_string())?,
            farm_name: intermediate_rep.farm_name.into_iter().next(),
            fruit_group_name: intermediate_rep
                .fruit_group_name
                .into_iter()
                .next()
                .ok_or("fruit_group_name missing in SprayTaskProperties".to_string())?,
            fruit_name: intermediate_rep
                .fruit_name
                .into_iter()
                .next()
                .ok_or("fruit_name missing in SprayTaskProperties".to_string())?,
            plain_name: intermediate_rep
                .plain_name
                .into_iter()
                .next()
                .ok_or("plain_name missing in SprayTaskProperties".to_string())?,
            plain_size: intermediate_rep
                .plain_size
                .into_iter()
                .next()
                .ok_or("plain_size missing in SprayTaskProperties".to_string())?,
            application_total_volume: intermediate_rep.application_total_volume.into_iter().next(),
            total_area: intermediate_rep.total_area.into_iter().next(),
            effective_total_distance: intermediate_rep.effective_total_distance.into_iter().next(),
            effective_total_time: intermediate_rep.effective_total_time.into_iter().next(),
            plant_protection: intermediate_rep
                .plant_protection
                .into_iter()
                .next()
                .ok_or("plant_protection missing in SprayTaskProperties".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SprayTaskProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SprayTaskProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SprayTaskProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SprayTaskProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SprayTaskProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SprayTaskProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SprayTaskProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SprayTaskPropertiesPlantProtection {
    /// Name of the protection fluid
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Amount of the protection fluid
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Unit of the protection fluid amount
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl SprayTaskPropertiesPlantProtection {
    pub fn new() -> SprayTaskPropertiesPlantProtection {
        SprayTaskPropertiesPlantProtection {
            name: None,
            amount: None,
            unit: None,
        }
    }
}

/// Converts the SprayTaskPropertiesPlantProtection value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SprayTaskPropertiesPlantProtection {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        if let Some(ref amount) = self.amount {
            params.push("amount".to_string());
            params.push(amount.to_string());
        }

        if let Some(ref unit) = self.unit {
            params.push("unit".to_string());
            params.push(unit.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SprayTaskPropertiesPlantProtection value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SprayTaskPropertiesPlantProtection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
            pub amount: Vec<i64>,
            pub unit: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SprayTaskPropertiesPlantProtection"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "amount" => intermediate_rep.amount.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "unit" => intermediate_rep.unit.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SprayTaskPropertiesPlantProtection"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SprayTaskPropertiesPlantProtection {
            name: intermediate_rep.name.into_iter().next(),
            amount: intermediate_rep.amount.into_iter().next(),
            unit: intermediate_rep.unit.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SprayTaskPropertiesPlantProtection> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SprayTaskPropertiesPlantProtection>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SprayTaskPropertiesPlantProtection>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SprayTaskPropertiesPlantProtection - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SprayTaskPropertiesPlantProtection>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SprayTaskPropertiesPlantProtection as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SprayTaskPropertiesPlantProtection - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Terrain {
    #[serde(rename = "id")]
    pub id: i64,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: models::MultiPointWithHeight,

    #[serde(rename = "properties")]
    pub properties: models::TerrainProperties,
}

impl Terrain {
    pub fn new(
        id: i64,
        type_: String,
        geometry: models::MultiPointWithHeight,
        properties: models::TerrainProperties,
    ) -> Terrain {
        Terrain {
            id: id,
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the Terrain value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Terrain {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("id".to_string());
        params.push(self.id.to_string());

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Terrain value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Terrain {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub type_: Vec<String>,
            pub geometry: Vec<models::MultiPointWithHeight>,
            pub properties: Vec<models::TerrainProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Terrain".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::MultiPointWithHeight as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::TerrainProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Terrain".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Terrain {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or("id missing in Terrain".to_string())?,
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in Terrain".to_string())?,
            geometry: intermediate_rep
                .geometry
                .into_iter()
                .next()
                .ok_or("geometry missing in Terrain".to_string())?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in Terrain".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Terrain> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Terrain>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Terrain>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Terrain - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Terrain> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Terrain as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Terrain - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TerrainProperties {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "field_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i64>,
}

impl TerrainProperties {
    pub fn new(description: String) -> TerrainProperties {
        TerrainProperties {
            description: description,
            field_id: None,
        }
    }
}

/// Converts the TerrainProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TerrainProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("description".to_string());
        params.push(self.description.to_string());

        if let Some(ref field_id) = self.field_id {
            params.push("field_id".to_string());
            params.push(field_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TerrainProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TerrainProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub description: Vec<String>,
            pub field_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TerrainProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "description" => intermediate_rep.description.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "field_id" => intermediate_rep.field_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TerrainProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TerrainProperties {
            description: intermediate_rep
                .description
                .into_iter()
                .next()
                .ok_or("description missing in TerrainProperties".to_string())?,
            field_id: intermediate_rep.field_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TerrainProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TerrainProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TerrainProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TerrainProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TerrainProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TerrainProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TerrainProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing a token.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Token {
    /// The token string.
    #[serde(rename = "tokenstr")]
    pub tokenstr: String,

    /// The ID of the user associated with the token.
    #[serde(rename = "user_id")]
    pub user_id: i32,
}

impl Token {
    pub fn new(tokenstr: String, user_id: i32) -> Token {
        Token {
            tokenstr: tokenstr,
            user_id: user_id,
        }
    }
}

/// Converts the Token value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Token {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("tokenstr".to_string());
        params.push(self.tokenstr.to_string());

        params.push("user_id".to_string());
        params.push(self.user_id.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Token value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub tokenstr: Vec<String>,
            pub user_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Token".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "tokenstr" => intermediate_rep.tokenstr.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "user_id" => intermediate_rep.user_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Token".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Token {
            tokenstr: intermediate_rep
                .tokenstr
                .into_iter()
                .next()
                .ok_or("tokenstr missing in Token".to_string())?,
            user_id: intermediate_rep
                .user_id
                .into_iter()
                .next()
                .ok_or("user_id missing in Token".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Token> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Token>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Token>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Token - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Token> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Token as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Token - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TokenId {
    /// Numeric ID of the Token.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl TokenId {
    pub fn new() -> TokenId {
        TokenId { id: None }
    }
}

/// Converts the TokenId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TokenId {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TokenId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TokenId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TokenId".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TokenId".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TokenId {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TokenId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TokenId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TokenId>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TokenId - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TokenId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TokenId as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TokenId - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing a user token.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserToken {
    /// The token string.
    #[serde(rename = "tokenstr")]
    pub tokenstr: String,
}

impl UserToken {
    pub fn new(tokenstr: String) -> UserToken {
        UserToken { tokenstr: tokenstr }
    }
}

/// Converts the UserToken value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserToken {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("tokenstr".to_string());
        params.push(self.tokenstr.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserToken value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserToken {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub tokenstr: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UserToken".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "tokenstr" => intermediate_rep.tokenstr.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UserToken".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserToken {
            tokenstr: intermediate_rep
                .tokenstr
                .into_iter()
                .next()
                .ok_or("tokenstr missing in UserToken".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserToken> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserToken>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UserToken>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UserToken - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserToken> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UserToken as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UserToken - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherData {
    /// The ID of the weather station.
    #[serde(rename = "station_id")]
    pub station_id: i32,

    /// The reference date of the weather data, following the format specified by RFC 3339.
    #[serde(rename = "measurement_date")]
    pub measurement_date: String,

    #[serde(rename = "fx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx: Option<models::WeatherDataFx>,

    #[serde(rename = "fm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fm: Option<models::WeatherDataFm>,

    #[serde(rename = "rsk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsk: Option<models::WeatherDataRsk>,

    /// Precipitation form (numerical code).
    #[serde(rename = "rskf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rskf: Option<isize>,

    #[serde(rename = "sdk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<models::WeatherDataSdk>,

    #[serde(rename = "shk_tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shk_tag: Option<models::WeatherDataShkTag>,

    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<models::WeatherDataNm>,

    #[serde(rename = "vpm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpm: Option<models::WeatherDataVpm>,

    #[serde(rename = "pm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pm: Option<models::WeatherDataPm>,

    #[serde(rename = "tmk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmk: Option<models::WeatherDataTmk>,

    #[serde(rename = "upm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upm: Option<models::WeatherDataUpm>,

    #[serde(rename = "txk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txk: Option<models::WeatherDataTxk>,

    #[serde(rename = "tnk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tnk: Option<models::WeatherDataTnk>,

    #[serde(rename = "tgk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgk: Option<models::WeatherDataTgk>,
}

impl WeatherData {
    pub fn new(station_id: i32, measurement_date: String) -> WeatherData {
        WeatherData {
            station_id: station_id,
            measurement_date: measurement_date,
            fx: None,
            fm: None,
            rsk: None,
            rskf: None,
            sdk: None,
            shk_tag: None,
            nm: None,
            vpm: None,
            pm: None,
            tmk: None,
            upm: None,
            txk: None,
            tnk: None,
            tgk: None,
        }
    }
}

/// Converts the WeatherData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherData {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("station_id".to_string());
        params.push(self.station_id.to_string());

        params.push("measurement_date".to_string());
        params.push(self.measurement_date.to_string());

        // Skipping fx in query parameter serialization

        // Skipping fm in query parameter serialization

        // Skipping rsk in query parameter serialization

        if let Some(ref rskf) = self.rskf {
            params.push("rskf".to_string());
            params.push(rskf.to_string());
        }

        // Skipping sdk in query parameter serialization

        // Skipping shk_tag in query parameter serialization

        // Skipping nm in query parameter serialization

        // Skipping vpm in query parameter serialization

        // Skipping pm in query parameter serialization

        // Skipping tmk in query parameter serialization

        // Skipping upm in query parameter serialization

        // Skipping txk in query parameter serialization

        // Skipping tnk in query parameter serialization

        // Skipping tgk in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub station_id: Vec<i32>,
            pub measurement_date: Vec<String>,
            pub fx: Vec<models::WeatherDataFx>,
            pub fm: Vec<models::WeatherDataFm>,
            pub rsk: Vec<models::WeatherDataRsk>,
            pub rskf: Vec<isize>,
            pub sdk: Vec<models::WeatherDataSdk>,
            pub shk_tag: Vec<models::WeatherDataShkTag>,
            pub nm: Vec<models::WeatherDataNm>,
            pub vpm: Vec<models::WeatherDataVpm>,
            pub pm: Vec<models::WeatherDataPm>,
            pub tmk: Vec<models::WeatherDataTmk>,
            pub upm: Vec<models::WeatherDataUpm>,
            pub txk: Vec<models::WeatherDataTxk>,
            pub tnk: Vec<models::WeatherDataTnk>,
            pub tgk: Vec<models::WeatherDataTgk>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherData".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "station_id" => intermediate_rep.station_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "measurement_date" => intermediate_rep.measurement_date.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fx" => intermediate_rep.fx.push(
                        <models::WeatherDataFx as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fm" => intermediate_rep.fm.push(
                        <models::WeatherDataFm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "rsk" => intermediate_rep.rsk.push(
                        <models::WeatherDataRsk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "rskf" => intermediate_rep.rskf.push(
                        <isize as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "sdk" => intermediate_rep.sdk.push(
                        <models::WeatherDataSdk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "shk_tag" => intermediate_rep.shk_tag.push(
                        <models::WeatherDataShkTag as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "nm" => intermediate_rep.nm.push(
                        <models::WeatherDataNm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "vpm" => intermediate_rep.vpm.push(
                        <models::WeatherDataVpm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "pm" => intermediate_rep.pm.push(
                        <models::WeatherDataPm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tmk" => intermediate_rep.tmk.push(
                        <models::WeatherDataTmk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "upm" => intermediate_rep.upm.push(
                        <models::WeatherDataUpm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "txk" => intermediate_rep.txk.push(
                        <models::WeatherDataTxk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tnk" => intermediate_rep.tnk.push(
                        <models::WeatherDataTnk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tgk" => intermediate_rep.tgk.push(
                        <models::WeatherDataTgk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherData".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherData {
            station_id: intermediate_rep
                .station_id
                .into_iter()
                .next()
                .ok_or("station_id missing in WeatherData".to_string())?,
            measurement_date: intermediate_rep
                .measurement_date
                .into_iter()
                .next()
                .ok_or("measurement_date missing in WeatherData".to_string())?,
            fx: intermediate_rep.fx.into_iter().next(),
            fm: intermediate_rep.fm.into_iter().next(),
            rsk: intermediate_rep.rsk.into_iter().next(),
            rskf: intermediate_rep.rskf.into_iter().next(),
            sdk: intermediate_rep.sdk.into_iter().next(),
            shk_tag: intermediate_rep.shk_tag.into_iter().next(),
            nm: intermediate_rep.nm.into_iter().next(),
            vpm: intermediate_rep.vpm.into_iter().next(),
            pm: intermediate_rep.pm.into_iter().next(),
            tmk: intermediate_rep.tmk.into_iter().next(),
            upm: intermediate_rep.upm.into_iter().next(),
            txk: intermediate_rep.txk.into_iter().next(),
            tnk: intermediate_rep.tnk.into_iter().next(),
            tgk: intermediate_rep.tgk.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherData>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherData - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherData as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherData - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of wind velocity (m/s).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataFm {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataFm {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataFm {
        WeatherDataFm {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataFm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataFm {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataFm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataFm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataFm".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataFm".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataFm {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataFm".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataFm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataFm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataFm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataFm>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataFm - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataFm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataFm as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataFm - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily maximum of wind gust (m/s).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataFx {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataFx {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataFx {
        WeatherDataFx {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataFx value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataFx {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataFx value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataFx {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataFx".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataFx".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataFx {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataFx".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataFx".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataFx> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataFx>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataFx>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataFx - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataFx> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataFx as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataFx - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of cloud cover (1/8).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataNm {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataNm {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataNm {
        WeatherDataNm {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataNm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataNm {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataNm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataNm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataNm".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataNm".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataNm {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataNm".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataNm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataNm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataNm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataNm>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataNm - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataNm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataNm as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataNm - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing the patch of weather data
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataPatch {
    #[serde(rename = "fx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx: Option<models::WeatherDataFx>,

    #[serde(rename = "fm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fm: Option<models::WeatherDataFm>,

    #[serde(rename = "rsk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsk: Option<models::WeatherDataRsk>,

    /// Precipitation form (numerical code).
    #[serde(rename = "rskf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rskf: Option<isize>,

    #[serde(rename = "sdk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<models::WeatherDataSdk>,

    #[serde(rename = "shk_tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shk_tag: Option<models::WeatherDataShkTag>,

    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<models::WeatherDataNm>,

    #[serde(rename = "vpm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpm: Option<models::WeatherDataVpm>,

    #[serde(rename = "pm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pm: Option<models::WeatherDataPm>,

    #[serde(rename = "tmk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmk: Option<models::WeatherDataTmk>,

    #[serde(rename = "upm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upm: Option<models::WeatherDataUpm>,

    #[serde(rename = "txk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txk: Option<models::WeatherDataTxk>,

    #[serde(rename = "tnk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tnk: Option<models::WeatherDataTnk>,

    #[serde(rename = "tgk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgk: Option<models::WeatherDataTgk>,
}

impl WeatherDataPatch {
    pub fn new() -> WeatherDataPatch {
        WeatherDataPatch {
            fx: None,
            fm: None,
            rsk: None,
            rskf: None,
            sdk: None,
            shk_tag: None,
            nm: None,
            vpm: None,
            pm: None,
            tmk: None,
            upm: None,
            txk: None,
            tnk: None,
            tgk: None,
        }
    }
}

/// Converts the WeatherDataPatch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataPatch {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping fx in query parameter serialization

        // Skipping fm in query parameter serialization

        // Skipping rsk in query parameter serialization

        if let Some(ref rskf) = self.rskf {
            params.push("rskf".to_string());
            params.push(rskf.to_string());
        }

        // Skipping sdk in query parameter serialization

        // Skipping shk_tag in query parameter serialization

        // Skipping nm in query parameter serialization

        // Skipping vpm in query parameter serialization

        // Skipping pm in query parameter serialization

        // Skipping tmk in query parameter serialization

        // Skipping upm in query parameter serialization

        // Skipping txk in query parameter serialization

        // Skipping tnk in query parameter serialization

        // Skipping tgk in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataPatch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataPatch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub fx: Vec<models::WeatherDataFx>,
            pub fm: Vec<models::WeatherDataFm>,
            pub rsk: Vec<models::WeatherDataRsk>,
            pub rskf: Vec<isize>,
            pub sdk: Vec<models::WeatherDataSdk>,
            pub shk_tag: Vec<models::WeatherDataShkTag>,
            pub nm: Vec<models::WeatherDataNm>,
            pub vpm: Vec<models::WeatherDataVpm>,
            pub pm: Vec<models::WeatherDataPm>,
            pub tmk: Vec<models::WeatherDataTmk>,
            pub upm: Vec<models::WeatherDataUpm>,
            pub txk: Vec<models::WeatherDataTxk>,
            pub tnk: Vec<models::WeatherDataTnk>,
            pub tgk: Vec<models::WeatherDataTgk>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataPatch".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "fx" => intermediate_rep.fx.push(
                        <models::WeatherDataFx as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "fm" => intermediate_rep.fm.push(
                        <models::WeatherDataFm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "rsk" => intermediate_rep.rsk.push(
                        <models::WeatherDataRsk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "rskf" => intermediate_rep.rskf.push(
                        <isize as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "sdk" => intermediate_rep.sdk.push(
                        <models::WeatherDataSdk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "shk_tag" => intermediate_rep.shk_tag.push(
                        <models::WeatherDataShkTag as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "nm" => intermediate_rep.nm.push(
                        <models::WeatherDataNm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "vpm" => intermediate_rep.vpm.push(
                        <models::WeatherDataVpm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "pm" => intermediate_rep.pm.push(
                        <models::WeatherDataPm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tmk" => intermediate_rep.tmk.push(
                        <models::WeatherDataTmk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "upm" => intermediate_rep.upm.push(
                        <models::WeatherDataUpm as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "txk" => intermediate_rep.txk.push(
                        <models::WeatherDataTxk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tnk" => intermediate_rep.tnk.push(
                        <models::WeatherDataTnk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tgk" => intermediate_rep.tgk.push(
                        <models::WeatherDataTgk as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataPatch".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataPatch {
            fx: intermediate_rep.fx.into_iter().next(),
            fm: intermediate_rep.fm.into_iter().next(),
            rsk: intermediate_rep.rsk.into_iter().next(),
            rskf: intermediate_rep.rskf.into_iter().next(),
            sdk: intermediate_rep.sdk.into_iter().next(),
            shk_tag: intermediate_rep.shk_tag.into_iter().next(),
            nm: intermediate_rep.nm.into_iter().next(),
            vpm: intermediate_rep.vpm.into_iter().next(),
            pm: intermediate_rep.pm.into_iter().next(),
            tmk: intermediate_rep.tmk.into_iter().next(),
            upm: intermediate_rep.upm.into_iter().next(),
            txk: intermediate_rep.txk.into_iter().next(),
            tnk: intermediate_rep.tnk.into_iter().next(),
            tgk: intermediate_rep.tgk.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataPatch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataPatch>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataPatch>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataPatch - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<WeatherDataPatch>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataPatch as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataPatch - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of pressure (hPa).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataPm {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataPm {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataPm {
        WeatherDataPm {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataPm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataPm {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataPm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataPm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataPm".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataPm".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataPm {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataPm".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataPm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataPm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataPm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataPm>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataPm - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataPm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataPm as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataPm - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily precipitation height (mm).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataRsk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataRsk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataRsk {
        WeatherDataRsk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataRsk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataRsk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataRsk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataRsk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataRsk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataRsk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataRsk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataRsk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataRsk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataRsk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataRsk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataRsk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataRsk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataRsk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataRsk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataRsk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily sunshine duration (h).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataSdk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataSdk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataSdk {
        WeatherDataSdk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataSdk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataSdk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataSdk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataSdk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataSdk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataSdk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataSdk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataSdk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataSdk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataSdk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataSdk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataSdk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataSdk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataSdk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataSdk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataSdk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily snow depth (cm).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataShkTag {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataShkTag {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataShkTag {
        WeatherDataShkTag {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataShkTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataShkTag {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataShkTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataShkTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataShkTag".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataShkTag".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataShkTag {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataShkTag".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataShkTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataShkTag> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataShkTag>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataShkTag>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataShkTag - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<WeatherDataShkTag>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataShkTag as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataShkTag - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily minimum of air temperature at 5 cm above ground (°C).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataTgk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataTgk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataTgk {
        WeatherDataTgk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataTgk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataTgk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataTgk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataTgk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataTgk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataTgk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataTgk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataTgk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataTgk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataTgk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataTgk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataTgk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataTgk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataTgk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataTgk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataTgk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of temperature (°C).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataTmk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataTmk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataTmk {
        WeatherDataTmk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataTmk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataTmk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataTmk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataTmk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataTmk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataTmk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataTmk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataTmk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataTmk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataTmk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataTmk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataTmk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataTmk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataTmk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataTmk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataTmk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily minimum of temperature at 2 m height (°C).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataTnk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataTnk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataTnk {
        WeatherDataTnk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataTnk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataTnk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataTnk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataTnk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataTnk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataTnk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataTnk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataTnk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataTnk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataTnk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataTnk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataTnk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataTnk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataTnk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataTnk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataTnk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily maximum of temperature at 2 m height (°C).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataTxk {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataTxk {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataTxk {
        WeatherDataTxk {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataTxk value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataTxk {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataTxk value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataTxk {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataTxk".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataTxk".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataTxk {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataTxk".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataTxk".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataTxk> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataTxk>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataTxk>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataTxk - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataTxk> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataTxk as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataTxk - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of relative humidity (%).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataUpm {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataUpm {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataUpm {
        WeatherDataUpm {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataUpm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataUpm {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataUpm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataUpm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataUpm".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataUpm".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataUpm {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataUpm".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataUpm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataUpm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataUpm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataUpm>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataUpm - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataUpm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataUpm as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataUpm - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Daily mean of vapor pressure (hPa).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherDataVpm {
    #[serde(rename = "value")]
    pub value: f64,

    /// The quality control procedure applied for a data report. 1 is the worst and 10 is the best. For details check the [document](https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/historical/DESCRIPTION_obsgermany_climate_daily_kl_historical_en.pdf)   * `1` - Only formal control   * `2` - Controlled with individually defined criteria   * `3` - Automatic control and correction   * `5` - Historic, subjective procedures   * `7` - Second control done, before correction   * `8` - Quality control outside routine   * `9` - Not all parameters corrected   * `10` - Quality control finished, all corrections finished   
    #[serde(rename = "quality_level")]
    pub quality_level: u32,
}

impl WeatherDataVpm {
    pub fn new(value: f64, quality_level: u32) -> WeatherDataVpm {
        WeatherDataVpm {
            value: value,
            quality_level: quality_level,
        }
    }
}

/// Converts the WeatherDataVpm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherDataVpm {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());

        params.push("quality_level".to_string());
        params.push(self.quality_level.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherDataVpm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherDataVpm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<f64>,
            pub quality_level: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherDataVpm".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "quality_level" => intermediate_rep.quality_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherDataVpm".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherDataVpm {
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or("value missing in WeatherDataVpm".to_string())?,
            quality_level: intermediate_rep
                .quality_level
                .into_iter()
                .next()
                .ok_or("quality_level missing in WeatherDataVpm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherDataVpm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherDataVpm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherDataVpm>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherDataVpm - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherDataVpm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherDataVpm as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherDataVpm - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing a weather station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherStation {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "geometry")]
    pub geometry: swagger::Nullable<PointWithHeight>,

    #[serde(rename = "properties")]
    pub properties: models::WeatherStationProperties,
}

impl WeatherStation {
    pub fn new(
        type_: String,
        geometry: swagger::Nullable<PointWithHeight>,
        properties: models::WeatherStationProperties,
    ) -> WeatherStation {
        WeatherStation {
            type_: type_,
            geometry: geometry,
            properties: properties,
        }
    }
}

/// Converts the WeatherStation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherStation {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());

        // Skipping geometry in query parameter serialization

        // Skipping properties in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherStation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherStation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub geometry: Vec<PointWithHeight>,
            pub properties: Vec<models::WeatherStationProperties>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherStation".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => return std::result::Result::Err(
                        "Parsing a nullable type in this style is not supported in WeatherStation"
                            .to_string(),
                    ),
                    "properties" => intermediate_rep.properties.push(
                        <models::WeatherStationProperties as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherStation".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherStation {
            type_: intermediate_rep
                .type_
                .into_iter()
                .next()
                .ok_or("type missing in WeatherStation".to_string())?,
            geometry: std::result::Result::Err(
                "Nullable types not supported in WeatherStation".to_string(),
            )?,
            properties: intermediate_rep
                .properties
                .into_iter()
                .next()
                .ok_or("properties missing in WeatherStation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherStation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherStation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherStation>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherStation - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WeatherStation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherStation as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherStation - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// An object representing the patch of a weather station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherStationPatch {
    /// The name of the weather station.
    #[serde(rename = "station_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_name: Option<String>,

    /// The Bundesland where the weather station is located.
    #[serde(rename = "station_bundesland")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_bundesland: Option<String>,

    #[serde(rename = "geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<models::PointWithHeight>,
}

impl WeatherStationPatch {
    pub fn new() -> WeatherStationPatch {
        WeatherStationPatch {
            station_name: None,
            station_bundesland: None,
            geometry: None,
        }
    }
}

/// Converts the WeatherStationPatch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherStationPatch {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref station_name) = self.station_name {
            params.push("station_name".to_string());
            params.push(station_name.to_string());
        }

        if let Some(ref station_bundesland) = self.station_bundesland {
            params.push("station_bundesland".to_string());
            params.push(station_bundesland.to_string());
        }

        // Skipping geometry in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherStationPatch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherStationPatch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub station_name: Vec<String>,
            pub station_bundesland: Vec<String>,
            pub geometry: Vec<models::PointWithHeight>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherStationPatch".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "station_name" => intermediate_rep.station_name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "station_bundesland" => intermediate_rep.station_bundesland.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "geometry" => intermediate_rep.geometry.push(
                        <models::PointWithHeight as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherStationPatch".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherStationPatch {
            station_name: intermediate_rep.station_name.into_iter().next(),
            station_bundesland: intermediate_rep.station_bundesland.into_iter().next(),
            geometry: intermediate_rep.geometry.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherStationPatch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherStationPatch>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherStationPatch>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherStationPatch - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<WeatherStationPatch>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherStationPatch as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherStationPatch - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, macros_lib::Model)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WeatherStationProperties {
    /// The ID of the weather station provided by Deutscher Wetterdienst (https://www.dwd.de).
    #[serde(rename = "id")]
    pub id: i32,

    /// The name of the weather station.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Bundesland where the weather station is located.
    #[serde(rename = "bundesland")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundesland: Option<String>,
}

impl WeatherStationProperties {
    pub fn new(id: i32) -> WeatherStationProperties {
        WeatherStationProperties {
            id: id,
            name: None,
            bundesland: None,
        }
    }
}

/// Converts the WeatherStationProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WeatherStationProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("id".to_string());
        params.push(self.id.to_string());

        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        if let Some(ref bundesland) = self.bundesland {
            params.push("bundesland".to_string());
            params.push(bundesland.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WeatherStationProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WeatherStationProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub name: Vec<String>,
            pub bundesland: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing WeatherStationProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "bundesland" => intermediate_rep.bundesland.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing WeatherStationProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WeatherStationProperties {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or("id missing in WeatherStationProperties".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            bundesland: intermediate_rep.bundesland.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WeatherStationProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WeatherStationProperties>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<WeatherStationProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for WeatherStationProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<WeatherStationProperties>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <WeatherStationProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into WeatherStationProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
