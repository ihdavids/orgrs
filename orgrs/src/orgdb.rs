//use orgize::{Org, ParseConfig};
use orgize::{Org, Event, Element};
use orgize::elements::{Title};
use indextree::{NodeId,Node,NodeEdge};
use tokio::io::AsyncReadExt;
use glob::glob;
use std::{collections::HashMap, path::Path, borrow::BorrowMut, io::Read};
use notify::{Watcher, RecursiveMode, ReadDirectoryChangesWatcher};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use log::{info, trace, error};
use colored::Colorize;
use std::default::Default;
use std::rc::Rc;

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

pub struct TitleInfo {
    pub id: NodeId,
    pub children: Vec<Arc<TitleInfo>>,
    pub parent: Option<Arc<TitleInfo>>,
    pub level: usize,
}

impl TitleInfo {
    pub fn new(nid: NodeId, level: usize) -> TitleInfo {
        return TitleInfo {id: nid , children: Vec::new(), parent: None,  level: level};
    }
}

pub struct OrgInfo<'a> {
    pub filename: String,
    pub org:   Org<'a>,
    pub nodes: Vec<Arc<TitleInfo>>,
    pub root: Arc<TitleInfo>,
}


impl OrgInfo<'_> {
    pub fn new(org: Org, id: NodeId, name: String) -> OrgInfo {
        return OrgInfo {filename: name, org: org, nodes: Vec::new(), root: Arc::new(TitleInfo::new(id, 0)) };
    }
}

pub struct OrgDb<'a> {
    pub by_file: HashMap<String,OrgInfo<'a>>,
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

   /* 
   pub fn Headings(org: &Org) -> impl Iterator<Item = &Title<'_>> {
        org.root
            .descendants(&org.arena)
            .skip(1)
            .filter_map(move |node| match &org[node] {
                Element::Title(t) => Some(t),
                _ => None,
            })
   }
   */

   pub fn root(org: & Org) -> Option<NodeId> {
        for (item) in org.arena().iter() {
            let rootId = org.arena().get_node_id(item);
            return rootId;
        }
        return None;
   }

   pub fn titles<'a>(org: &'a Org) -> Box<dyn Iterator<Item = &'a Title<'a>>+'a> {
        if let Some(root) = Self::root(org) {
            Box::new(root.descendants(org.arena())
                .skip(1)
                .filter_map(move |node| match &org[node] {
                    Element::Title(t) => Some(t),
                    _ => None,
            }))
        } else {
            Box::new(::std::iter::empty())
        }
   }

   pub fn nodes<'a>(org: &'a Org) -> Box<dyn Iterator<Item = NodeEdge>> {
        if let Some(root) = Self::root(org) {
            Box::new(root.traverse(org.arena()))
        } else {
            Box::new(::std::iter::empty())
        }
   }

   pub async fn reload_all(&mut self, path: &String) {
        for entry in glob(path.as_str()).unwrap() {
            let f = entry.unwrap();
            let name = f.as_os_str().to_str().unwrap();
            let nm = String::from(name);
            let org: Org = OrgDb::parse_org_file(&nm).await.unwrap();
            if nm.contains("\\inxile_tasks_old") {
			// TODO: Iterate over "nodes" and extract info.
			// TODO: Handle that expect
            println!("--> {name}");
            //Headings(&org);
            //for (item) in Self::titles(&org) {
            //    println!("{:?}", item);
            //}
            }
            if let Some(rid) = Self::root(&org) {
                let mut orgref = OrgInfo::new(org, rid, nm.clone());
                let mut cur = orgref.root.clone();

                for (edge) in Self::nodes(&orgref.org) {
                    match edge {
                        NodeEdge::Start(node) => { 
                          let e = &orgref.org[node];
                          match e {
                            Element::Title(t) => {
                                let mut tifo = Arc::new(TitleInfo::new(node, t.level));

                                orgref.nodes.push(tifo.clone());

                                if t.level > cur.level {
                                   let mut mifo = tifo.borrow_mut();
                                   mifo.parent = Some(cur.clone());
                                   cur.children.push(tifo.clone());
                                } else {
                                    error!("PARSE ERROR!!!!, LEVEL DOES NOT MATCH");
                                }
                            }
                            _ => {}
                          }
                        },
                        NodeEdge::End(node) => { 
                          let e = &orgref.org[node];
                          if let Some(x) = cur.parent.clone() {
                            cur = x;
                          } else {
                            cur = orgref.root.clone();
                          }
                        }
                    }
                }
                self.by_file.insert(nm.clone(), orgref);
                /* 
			    for (item) in orgref.org.iter() {
				    if let Event::Start(e) = item {
                        //println!("{:?}", e);
					    if let Element::Title(t) = e {
                            orgref.nodes.append(orgref.org.arena().get_node_id(e));

					//	if(eval_node(node, name, &compiled, &slab).expect("Hi")) {
					//	}
					    }
				}
                */
            }
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
        println!("[{}]","DB FILE LIST".blue());
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