use pg_sgr_from_row::FromRow;

use crate::connect;

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
        main: "main value".into(), 
        optional: Some("optional".into()), 
        with_from: vec![ "A".into() ], 
        renamed_column: true, 
        default: 12, 
        column_and_default: "cad value".into()
      },
      FromRowTestStruct { 
        main: "another".into(), 
        optional: None, 
        with_from: vec![ "A".into() ], 
        renamed_column: false, 
        default: 42, 
        column_and_default: "<NO CAD>".into()
      }
    ]
  );
}

#[derive(Debug, PartialEq, FromRow)]
struct FromRowTestStruct {
  pub main: String,
  pub optional: Option<String>,
  #[from_row(from = vec![ "A".into() ])]
  pub with_from: Vec<String>,
  #[from_row(column = "renamedColumn")]
  pub renamed_column: bool,
  #[from_row(default = 42)]
  pub default: i32,
  #[from_row(column = "cad", default = "<NO CAD>".into())]
  pub column_and_default: String
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES 
  (
    'main value', 
    'optional', 
    '{}'::VARCHAR[], 
    true, 
    12, 
    'cad value'
  ),
  (
    'another', 
    NULL, 
    NULL, 
    false, 
    NULL, 
    NULL
  ) 
) AS t(
  "main", 
  "optional", 
  "with_from", 
  "renamedColumn", 
  "default", 
  "cad"
)
"#;