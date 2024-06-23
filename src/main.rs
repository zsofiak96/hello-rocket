#[macro_use] extern crate rocket;

use std::fmt::Debug;
use rocket::request::FromParam;
use rocket::http::Status;
use rocket::serde::json::Json;
use tokio_postgres::types::{IsNull, ToSql, Type};
use tokio_postgres::{NoTls, Error};
use chrono::NaiveDate;
use serde::Serialize;
use bytes::BytesMut;

#[derive(Serialize, Debug)]
struct RocketResponse {
    name: String
}

#[derive(Debug)]
struct LaunchDate(NaiveDate);

impl<'r> FromParam<'r> for LaunchDate {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(param, "%Y-%m-%d").map(LaunchDate).map_err(|_| param)
    }
}

impl ToSql for LaunchDate {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty, &Type::DATE)
    }

    tokio_postgres::types::to_sql_checked!();
}

async fn fetch_rockets(launch_date: LaunchDate) ->  Result<Vec<RocketResponse>, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=rocket user=foo password=bar", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    let rows = client
        .query("SELECT name FROM rocket WHERE launch_date > $1", &[&launch_date])
        .await?;
    let mut rockets = Vec::new();
    for row in rows {
        let rocket = RocketResponse  {
            name: row.get(0),
        };
        rockets.push(rocket);
    }
    Ok(rockets)
}

#[get("/<launch_date>")]
async fn get_rockets(launch_date: LaunchDate) -> Result<Json<Vec<RocketResponse>>, Status> {
    match fetch_rockets(launch_date).await {
        Ok(rockets) => Ok(Json(rockets)),
        Err(e) => {
            eprintln!("Handler error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/launch", routes![get_rockets])
}

#[cfg(test)]
mod test {
    use super::{get_rockets as test_get_rockets, rocket};
    use rocket::local::asynchronous::Client;
    use rocket::http::Status;

    #[rocket::async_test]
    async fn rockets() {
        let rocket = rocket::build().mount("/launch", routes![test_get_rockets]);
        let client = Client::tracked(rocket).await.unwrap();
        let req = client.get("/launch/2023-06-29");

        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);

        let (s1, s2) = (r1.into_string().await, r2.into_string().await);
        assert_eq!(s1, s2);
        assert_eq!(s1.unwrap(), "[{\"name\":\"Falcon 9\"},{\"name\":\"H-3\"}]");
    }
}
