use orgize::{Org, ParseConfig};
use tokio::io::AsyncReadExt;
use glob::glob;
use std::{collections::HashMap, path::Path, borrow::BorrowMut};
use notify::{EventKind, Watcher, RecommendedWatcher, RecursiveMode, ReadDirectoryChangesWatcher};
use std::sync::{Arc, Weak, Mutex};
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
    pub by_file: HashMap<String,Org<'a>>,
    pub watcher: Option<ReadDirectoryChangesWatcher>
}

//static mut db: Arc<OrgDb> = Arc::new(OrgDb::new()); 

impl OrgDb<'_> {
   pub fn new<'a>() -> Arc<Mutex<OrgDb<'a>>> {
        Arc::new(Mutex::new(OrgDb { by_file: HashMap::new() , watcher: None}))
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

    pub async fn reload(&mut self, nm: &String) {
        let org: Org = OrgDb::parse_org_file(nm).await.unwrap();
        self.by_file.insert(nm.clone(), org);
    }

    pub async fn list_all_files(&self) {
        for (name, _) in &self.by_file {
            println!("-> {name}");
        }
    }

    pub fn watch_handler(weak_db: &Weak<Mutex<OrgDb>>, res: Result<notify::Event, notify::Error>) -> () {
            match res {
                Ok(event) => {
                    println!("event: {:?}", event);
                    match event.kind {
                        Modify => {
                            for name in event.paths {
                                let tmp = String::from(name.to_string_lossy());
                                weak_db.upgrade()
                                    .map( |db| {
                                        let xdb = db.borrow_mut();
                                        match xdb.lock() {
                                            Ok(ydb) => {
                                                ydb.reload(&tmp);
                                            }
                                            Err(e) => {
                                                println!("Failed to update, could not aquire lock!");
                                            }
                                        }
                                    });
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("watch error: {:?}", e);
                }
            }
    }

    pub fn watch(mut db: Arc<Mutex<OrgDb>>, path: &String) -> notify::Result<()> {
        let weak_db = Arc::downgrade(&db.clone());

        // Automatically select the best implementation for your platform.

        let xdb = db.borrow_mut();
        let mut ydb = xdb.lock().unwrap();
        ydb.watcher = Some(
                notify::recommended_watcher(move |res: Result<notify::Event, notify::Error> | {
                    OrgDb::watch_handler(&weak_db, res)
                })?
            );
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.A
        match ydb.watcher {
            Some(w) => {
                w.borrow_mut().watch(Path::new(path), RecursiveMode::Recursive)?;
            }
            None => {
                println!("Failed to start up watcher!");
            }
        }

        Ok(())
    }
}