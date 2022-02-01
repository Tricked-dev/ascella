use crate::queries::prelude::*;
use serde_json::json;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(owner: i32, amount: i32, skip: i32) -> Result<Vec<SimpleImages>> {
    let rows = get_tokio_postgres()
    .await
    .query(
        //SQL INJECTION LMAOO this lang sucks!
      format!("SELECT created, id,vanity FROM images WHERE owner = {} ORDER BY id DESC LIMIT {} OFFSET {}", owner, amount,skip).as_str(),
      &[],
    )
    .await?.iter().map(|x|
      SimpleImages::from_row_ref(x).unwrap()
  ).collect::<Vec<SimpleImages>>();
    /*  let mut new_rows: Vec<Images> = vec![];
    for row in row.iter() {
      new_rows.push(Images::from_row_ref(row).unwrap())
    }*/
    Ok(rows)
}

pub async fn delete_all(owner: i32, date: i32) -> Result<u64> {
    let row = get_tokio_postgres()
        .await
        .execute(
            &format!(
                "DELETE FROM images WHERE created < NOW() - INTERVAL '{} days' AND owner = $1",
                date,
            ),
            &[&owner],
        )
        .await?;

    Ok(row)
}
