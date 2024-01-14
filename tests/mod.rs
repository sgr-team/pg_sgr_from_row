mod macro_from_json;
mod macro_from_row;
mod macro_from_rows;

#[cfg(feature = "sync")]
pub(crate) fn connect() -> postgres::Client {
  postgres::Client::connect(CONNECTION_STRING, postgres::NoTls).unwrap()
}

#[cfg(feature = "tokio")]
pub(crate) async fn connect() -> tokio_postgres::Client {
  let (client, connection) = tokio_postgres::connect(CONNECTION_STRING, tokio_postgres::NoTls)
    .await
    .unwrap();

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  client
}

const CONNECTION_STRING: &str = "postgresql://postgres:123456@localhost:5432/main";