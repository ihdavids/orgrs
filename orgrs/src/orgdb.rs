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
   async fn parse_org_file(filename: &String) 
   {
        let mut file = tokio::fs::File::open("foo.txt").await.expect("Failed to read org file off disk");
        let mut contents = vec![];
        file.read_buf(&mut contents).await.expect("Test?");
   }
}