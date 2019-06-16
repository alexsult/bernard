extern crate uuid;

use futures::Future;
use uuid::Uuid;
use Bernard;
use Entity;

#[derive(Debug)]
pub struct Lookup<'a> {
    bernard: &'a mut Bernard,
    mbid: Uuid,
    includes: Vec<String>,
}

impl<'a> Lookup<'a> {
    pub fn new(bernard: &'a mut Bernard, mbid: &'a str) -> Lookup<'a> {
        Lookup {
            bernard: bernard,
            mbid: Uuid::parse_str(mbid).unwrap(),
            includes: Vec::new(),
        }
    }

    pub fn include(&'a mut self, param: &str) -> &'a mut Lookup {
        self.includes.push(param.to_string());
        self
    }

    pub fn load<T>(&mut self) -> Box<Future<Item = T, Error = hyper::Error>>
    where
        T: Entity,
    {
        let mut inc: String = String::new();

        for include in &self.includes {
            if inc.len() == 0 {
                inc = format!("{}", include)
            } else {
                inc = format!("{},{}", inc, include)
            }
        }

        if inc.len() > 0 {
            self.bernard.set_param("inc", &inc);
        }

        self.bernard.mbid = self.mbid;

        return T::lookup(self.bernard);
    }
}
