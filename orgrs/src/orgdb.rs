//use orgize::{Org, ParseConfig};
use orgize::{Org};
use tokio::io::AsyncReadExt;
use glob::glob;
use std::{collections::HashMap, path::Path, borrow::BorrowMut, io::Read};
use notify::{Watcher, RecursiveMode, ReadDirectoryChangesWatcher};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use log::{info, trace, error};

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
    pub watcher: ReadDirectoryChangesWatcher
}


impl OrgDb<'_> {
   pub fn get() -> Arc<Mutex<OrgDb<'static>>> {
        static DB: Lazy<Arc<Mutex<OrgDb>>> = Lazy::new( || {
            Arc::new(Mutex::new(OrgDb { by_file: HashMap::new() , watcher: notify::recommended_watcher(|res: Result<notify::Event, notify::Error> | { OrgDb::watch_handler(res) }).unwrap() }))
        });
        return DB.clone();
   }

   pub async fn parse_org_file<'a>(filename: &String) -> Result<Org<'a>,std::io::Error>
   {
        trace!("READING: {}", filename.as_str());
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

        trace!("PARSING: {}", filename.as_str());
        Ok(Org::parse_string(contents.to_string()))
   }

   pub fn parse_org_file_sync<'a>(filename: &String) -> Result<Org<'a>,std::io::Error>
   {
        trace!("READING SYNC: {}", filename.as_str());
        // Have UTF-8 problems with this pathway.
        //let contents = tokio::fs::read_to_string(filename).await?;

        // This is more expensive than the alternative but it can handle non UTF-8 Files
        // Not sure which one I should support. Or optional?
        let mut buf = vec![];
        {
            let mut file = std::fs::File::open(filename)?;
            file.read_to_end(&mut buf)?;
        }
        let contents = String::from_utf8_lossy (&buf);

        trace!("PARSING SYNC: {}", filename.as_str());
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

    pub fn reload(&mut self, nm: &String) {
        let org: Org = OrgDb::parse_org_file_sync(nm).unwrap();
        info!("RELOADING: {}",nm);
        self.by_file.insert(nm.clone(), org);
    }

    pub fn delete(&mut self, nm: &String) {
        info!("DELETING: {}",nm);
        self.by_file.remove(nm);
    }

    pub async fn list_all_files(&self) {
        for (name, _) in &self.by_file {
            println!("-> {name}");
        }
    }

    pub fn watch_handler(res: Result<notify::Event, notify::Error>) -> () {
            match res {
                Ok(event) => {
                    //println!("event: {:?}", event);
                    if event.kind.is_modify() || event.kind.is_create() {
                        let mut db = OrgDb::get();
                        for name in event.paths {
                            let tmp = String::from(name.to_string_lossy());
                            let xdb = db.borrow_mut();
                            match xdb.lock() {
                                Ok(mut ydb) => {
                                    ydb.reload(&tmp);
                                }
                                Err(_e) => {
                                    error!("Failed to update, could not aquire lock!");
                                }
                            }
                        }
                    } else if event.kind.is_remove() {
                        let mut db = OrgDb::get();
                        for name in event.paths {
                            let tmp = String::from(name.to_string_lossy());
                            let xdb = db.borrow_mut();
                            match xdb.lock() {
                                Ok(mut ydb) => {
                                    ydb.delete(&tmp);
                                }
                                Err(_e) => {
                                    error!("Failed to update, could not aquire lock!");
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("watch error: {:?}", e);
                }
            }
    }

    pub fn watch(path: &String) -> notify::Result<()> {
        let mut db = OrgDb::get();
        let xdb = db.borrow_mut();
        match xdb.lock() {
            Ok(mut ydb) => {
                ydb.watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
            }
            Err(_e) => {
                error!("Failed to update, could not aquire lock!");
            }
        }

        Ok(())
    }
}