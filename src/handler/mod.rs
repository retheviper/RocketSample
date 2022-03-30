pub mod member_handler {
    use chrono::{DateTime, Utc};
    use rocket_contrib::json::Json;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Member {
        id: u32,
        user_id: String,
        name: String,
        #[serde(with = "date_format")]
        created_date: DateTime<Utc>,
        roles: Vec<String>,
    }

    #[get("/api/v1/web/members")]
    pub fn list_member() -> Json<Vec<Member>> {
        Json(
            (1..=10).map(|id| create_member_for_test(id)).collect()
        )
    }

    #[get("/api/v1/web/members/<id>")]
    pub fn get_member(id: u32) -> Json<Member> {
        Json(
            create_member_for_test(id)
        )
    }

    fn create_member_for_test(id: u32) -> Member {
        Member {
            id,
            user_id: format!("member_id_{}", id),
            name: format!("member_{}", id),
            created_date: Utc::now(),
            roles: vec![String::from("member")],
        }
    }

    mod date_format {
        use chrono::{DateTime, TimeZone, Utc};
        use serde::{self, Deserialize, Deserializer, Serializer};

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