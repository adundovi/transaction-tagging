pub mod csv;

pub mod date_serializer {
    use chrono::{Datelike, NaiveDate};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
        format!("{}.{}.{}.", date.day(), date.month(), date.year()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDate::parse_from_str(&time, "%d.%m.%Y.").map_err(D::Error::custom)?)
    }
}

pub mod currency_serializer {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
        value.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(value
            .trim()
            .replace('.', "")
            .replace(',', ".")
            .parse::<f64>()
            .map_err(D::Error::custom)?)
    }
}

pub mod currency_serializer_option {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(v) => format!("{}", v).serialize(serializer),
            None => format!("").serialize(serializer),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<f64>, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        let r = if value.trim().len() == 0 {
            None
        } else {
            Some(
                value
                    .trim()
                    .replace('.', "")
                    .replace(',', ".")
                    .parse::<f64>()
                    .map_err(D::Error::custom)?,
            )
        };
        Ok(r)
    }
}

