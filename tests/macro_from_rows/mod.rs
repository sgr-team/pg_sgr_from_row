use pg_sgr_from_row::{FromRow, FromRows};

use super::connect;

#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)]
#[tokio::test]
async fn main() {
  #[allow(unused_mut)]
  let mut client = connect().await;
  
  assert_eq!(
    client.query(SQL, &[])
      .await
      .map(FromRowsTestStruct::from_rows)
      .unwrap(),
    vec![
      FromRowsTestStruct { first_name: "Edgar".into(), last_name: "Poe".into() },
      FromRowsTestStruct { first_name: "John".into(), last_name: "Tolkien".into() }
    ]
  );
}

#[derive(Debug, PartialEq, FromRow, FromRows)]
struct FromRowsTestStruct {
  #[from_row(column = "firstName")]
  pub first_name: String,
  #[from_row(column = "lastName")]
  pub last_name: String,
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES 
  ('Edgar', 'Poe'),
  ('John', 'Tolkien')
) AS t(
  "firstName", 
  "lastName"
)
"#;