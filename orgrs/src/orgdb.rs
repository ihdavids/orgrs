use orgize::{Org, ParseConfig};
use tokio::io::AsyncReadExt;
use glob::glob;
use std::{collections::HashMap, path::Path};
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
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

pub struct OrgDb<'a> {
    pub by_file: HashMap<String,Org<'a>>
}

impl OrgDb<'_> {
   pub fn new<'a>() -> OrgDb<'a> {
        OrgDb { by_file: HashMap::new() }
   }

   pub async fn parse_org_file<'a>(filename: &String) -> Result<Org<'a>,std::io::Error>
   {
        println!("READING: {}", filename.as_str());
        // Have UTF-8 problems with this pathway.
        //let contents = tokio::fs::read_to_string(filename).await?;

        // This is more expensive than the alternative but it can handle non UTF-8 Files
        // Not sure which one I should support. Or optional?
        let mut buf = vec![];
        {
            let mut file = tokio::fs::File::open(filename).await?;
            file.read_to_end(&mut buf).await?;
        }
        let contents = String::from_utf8_lossy (&buf);

        println!("PARSING: {}", filename.as_str());
        Ok(Org::parse_string(contents.to_string()))
   }

   pub async fn reload_all(&mut self, path: &String) {
        for entry in glob(path.as_str()).unwrap() {
            let f = entry.unwrap();
            let name = f.as_os_str().to_str().unwrap();
            let nm = String::from(name);
            let org: Org = OrgDb::parse_org_file(&nm).await.unwrap();
            self.by_file.insert(nm.clone(), org);
        }
    }

    pub async fn list_all_files(&self) {
        for (name, _) in &self.by_file {
            println!("-> {name}");
        }
    }

    pub async fn watch(&self, path: &String) -> notify::Result<()> {
            // Automatically select the best implementation for your platform.
        let mut watcher = notify::recommended_watcher(|res| {
            match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        })?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

        Ok(())
    }
}