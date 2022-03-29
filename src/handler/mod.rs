pub mod member_handler {
    use chrono::{DateTime, Utc};
    use serde::Serialize;
    use rocket_contrib::json::Json;

    #[derive(Serialize)]
    pub struct Member {
        id: u32,
        user_id: String,
        name: String,
        #[serde(with = "date_format")]
        created_date: DateTime<Utc>,
        roles: Vec<String>,
    }

    #[get("/api/v1/web/members/<id>")]
    pub fn get_member(id: u32) -> Json<Member> {
        Json(
            Member {
                id,
                user_id: format!("member_id_{}", id).to_string(),
                name: format!("member_{}", id).to_string(),
                created_date: Utc::now(),
                roles: vec!["member".to_string()],
            }
        )
    }

    mod date_format {
        use chrono::{DateTime, Utc, TimeZone};
        use serde::{self, Deserialize, Serializer, Deserializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(
            date: &DateTime<Utc>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<DateTime<Utc>, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }
}