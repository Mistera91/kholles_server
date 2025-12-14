use crate::md_to_html::md_to_html;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Proof {
    pub pid: <Self as ProofTrait>::ProofIdType,
    pub title: String,
    pub note: Option<String>,
    pub authors: Vec<String>,
    #[serde(deserialize_with = "deserialize_date")]
    #[serde(serialize_with = "serialize_date")]
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub content: String,
}

pub trait ProofTrait {
    type ProofIdType;
}

impl ProofTrait for Proof {
    type ProofIdType = u32;
}

impl PartialOrd for Proof {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl Ord for Proof {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl Proof {
    pub fn as_html_proof(&self) -> Proof {
        Proof {
            content: md_to_html(self.content.as_str()),
            ..self.clone()
        }
    }
}

impl Proof {
    pub fn get_html(&self) -> String {
        md_to_html(self.content.as_str())
    }
}

pub trait WeekTrait {
    type WeekNumberType;
}

impl WeekTrait for Week {
    type WeekNumberType = u8;
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Week {
    #[serde(skip_deserializing)]
    pub number: <Self as WeekTrait>::WeekNumberType,
    pub date: String, // TODO: Change
    pub description: String,
    pub proofs: Vec<<Proof as ProofTrait>::ProofIdType>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let dt = NaiveDateTime::parse_from_str(&String::deserialize(deserializer)?, "%m/%d/%Y").map_err(serde::de::Error::custom)?;

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

fn serialize_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.naive_utc().format("%m/%d/%Y").to_string())
}
