use pg_sgr_from_row::{FromRow, FromJson};
use serde::Deserialize;

use super::connect;

#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)]
#[tokio::test]
async fn main() {
  #[allow(unused_mut)]
  let mut client = connect().await;

  let data: Vec<FromRowTestStruct> = client.query(SQL, &[])
    .await
    .map(|x| x.iter().map(|row| row.into()).collect())
    .unwrap();

  assert_eq!(
    data,
    vec![
      FromRowTestStruct {
        main: FromJsonTestStruct { first_name: "Edgar".into(), last_name: "Poe".into() },
        optional: Some(FromJsonTestStruct { first_name: "John".into(), last_name: "Tolkien".into() }),
        vec: vec![ 
          FromJsonTestStruct { first_name: "Neil".into(), last_name: "Gaiman".into() },
          FromJsonTestStruct { first_name: "Hunter".into(), last_name: "Thompson".into() },
        ],
        optional_vec: Some(vec![
          FromJsonTestStruct { first_name: "Alexander".into(), last_name: "Pushkin".into() },
          FromJsonTestStruct { first_name: "Mikhail".into(), last_name: "Lermontov".into() },
        ])
      },
      FromRowTestStruct {
        main: FromJsonTestStruct { first_name: "Edgar".into(), last_name: "Poe".into() },
        optional: None,
        vec: vec![ ],
        optional_vec: None
      }
    ]
  );
}

#[derive(Debug, PartialEq, FromRow)]
struct FromRowTestStruct {
  pub main: FromJsonTestStruct,
  pub optional: Option<FromJsonTestStruct>,
  pub vec: Vec<FromJsonTestStruct>,
  #[from_row(column = "optionalVec")]
  pub optional_vec: Option<Vec<FromJsonTestStruct>>,
}

#[derive(Debug, PartialEq, Deserialize, FromJson)]
#[serde(rename_all = "camelCase")]
struct FromJsonTestStruct {
  pub first_name: String,
  pub last_name: String,
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES 
  (
    '{"firstName": "Edgar", "lastName": "Poe"}'::JSON, 
    '{"firstName": "John", "lastName": "Tolkien"}'::JSON, 
    ARRAY[
      '{"firstName": "Neil", "lastName": "Gaiman"}'::JSON,
      '{"firstName": "Hunter", "lastName": "Thompson"}'::JSON
    ],
    ARRAY[
      '{"firstName": "Alexander", "lastName": "Pushkin"}'::JSON,
      '{"firstName": "Mikhail", "lastName": "Lermontov"}'::JSON
    ]
  ),
  (
    '{"firstName": "Edgar", "lastName": "Poe"}'::JSON, 
    NULL, 
    ARRAY[]::JSON[],
    NULL
  )
) AS t(
  "main", 
  "optional", 
  "vec", 
  "optionalVec"
)
"#;