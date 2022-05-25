// use std::io;

// use actix::fut;
// use actix::prelude::*;
// use futures::{stream, Future, Stream};
// use tokio_postgres::NoTls;
// use tokio_postgres::{connect, Client, Statement};
// use serde::{Deserialize, Serialize};

// pub struct PgConnection {
//   pg_client: Option<Client>,
//   // write all the statements
//   write_to_db: Option<Statement>,
//   read_from_db: Option<Statement>,
// }

// impl Actor for PgConnection {
//   type Context = Context<Self>;
// }

// impl PgConnection {
//   pub fn connect(db_url: &str) -> Addr<PgConnection> {
//     let pg_connection = connect(db_url, NoTls).await?;

//     PgConnection::create(move |ctx| {
//       let pg_actor = PgConnection {
//         pg_client: None,
//         write_to_db: None,
//         read_from_db: None,
//       };

//       pg_connection
//         .into_actor(&pg_actor)
//         .and_then(|(pg_client, conn), pg_actor, ctx| {
//           // implement all statements which we may need

//           ctx.wait(
//             pg_client
//               .prepare("INSERT INTO city_api (department_code, insee_code, zip_code, name, lat, lon) VALUES ($1, $2, $3, $4, $5, $6)")
//               .into_actor(pg_actor)
//               .and_then(|st, pg_actor, _| {
//                 pg_actor.write_to_db = Some(st);
//                 fut::ok(())
//               }),
//           );

//           // SELECT id, name, data
//           // 	FROM public.person;
//           ctx.wait(
//             pg_client
//               .prepare("SELECT department_code, insee_code, zip_code, name, lat, lon FROM city_api")
//               .map_err(|_| ())
//               .into_actor(pg_actor)
//               .and_then(|st, pg_actor, _| {
//                 pg_actor.read_from_db = Some(st);
//                 fut::ok(())
//               }),
//           );

//           // end for prepared statements
//           pg_actor.pg_client = Some(pg_client);
//           Arbiter::spawn(conn.map_err(|e| panic!("{}", e)));
//           fut::ok(())
//         }).wait(ctx);

//       pg_actor
//     })
//   }
// }

// // All statements to run from the client
// #[derive(Serialize, Deserialize, Debug)]
// pub struct AddCity {
//   department_code: String,
//   insee_code: String,
//   zip_code: String,
//   name: String,
//   lat: f64,
//   lon: f64,
// }

// impl Message for AddCity {
//   type Result = io::Result<()>;
// }

// impl Handler<AddCity> for PgConnection {
//   type Result = ResponseFuture<(), io::Error>;

//   fn handle(&mut self, data: AddCity, _: &mut Self::Context) -> Self::Result {
//     Box::new(
//       self
//         .pg_client
//         .as_mut()
//         .unwrap()
//         .query(
//           self.write_to_db.as_ref().unwrap(),
//           &[&data.department_code, &data.insee_code, &data.zip_code, &data.name, &data.lat, &data.lon],
//         ).into_future()
//         .map_err(|e| io::Error::new(io::ErrorKind::Other, e.0))
//         .and_then(|(_, _)| Ok(())),
//     )
//   }
// }

// // Read from the connection
// #[derive(Debug)]
// pub struct City {
//   id: i32,
//   department_code: String,
//   insee_code: String,
//   zip_code: String,
//   name: String,
//   lat: f64,
//   lon: f64,
// }

// // pub struct ReadCities {
// // }

// // impl Message for ReadCities {
// //   type Result = io::Result<Vec<City>>;
// // }

// // impl Handler<ReadCities> for PgConnection {
// //   type Result = ResponseFuture<Vec<City>, io::Error>;

// //   fn handle(&mut self, msg: ReadCities, _: &mut Self::Context) -> Self::Result {
// //     let mut worlds = Vec::new();

// //     Box::new(
// //       self
// //         .pg_client
// //         .as_mut()
// //         .unwrap()
// //         .query(
// //           self.read_from_db.as_ref().unwrap(),
// //           &[],
// //         ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
// //         .fold(worlds, move |mut worlds, row| {
// //           worlds.push(City {
// //             id: row.get(0),
// //             department_code: row.get(1),
// //             insee_code: row.get(2),
// //             zip_code: row.get(3),
// //             name: row.get(4),
// //             lat: row.get(5),
// //             lon: row.get(6),
// //           });
// //           Ok::<_, io::Error>(worlds)
// //         }).and_then(|worlds| Ok(worlds)),
// //     )
// //   }
// // }
