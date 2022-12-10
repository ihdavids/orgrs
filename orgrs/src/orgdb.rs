use orgize::{Org, ParseConfig};
use tokio::io::AsyncReadExt;

/* 
Org::parse_custom(
    "* TASK Title 1",
    &ParseConfig {
        // custom todo keywords
        todo_keywords: (vec!["TASK".to_string()], vec![]),
        ..Default::default()
    },
);
*/

pub struct OrgDb;

impl OrgDb {
   async fn parse_org_file(filename: &String) -> Result<Org,std::io::Error>
   {
        let contents = tokio::fs::read_to_string(filename).await?;
        Ok(Org::parse_string(contents))
   }
}