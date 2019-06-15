extern crate uuid;

use futures::Future;
use Bernard;
use Entity;

#[derive(Debug)]
pub struct BernardBrowse<'a> {
    bernard: &'a mut Bernard,
    includes: Vec<String>
}

impl<'a> BernardBrowse<'a> {
    pub fn new(
        bernard: &'a mut Bernard,
    ) -> BernardBrowse<'a> {

        BernardBrowse {
            bernard: bernard,
            includes: Vec::new()
        }
    }

    pub fn area(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("area", &param);
        self
    }

    pub fn artist(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("artist", &param);
        self
    }

    pub fn collection(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("collection", &param);
        self
    }

    pub fn editor(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("editor", &param);
        self
    }

    pub fn event(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("event", &param);
        self
    }

    pub fn label(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("label", &param);
        self
    }

    pub fn place(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("place", &param);
        self
    }

    pub fn recording(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("recording", &param);
        self
    }

    pub fn release(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("release", &param);
        self
    }

    pub fn release_group(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("release-group", &param);
        self
    }

    pub fn track(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("track", &param);
        self
    }

    pub fn track_artist(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("track_artist", &param);
        self
    }

    pub fn work(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.bernard.set_entity("work", &param);
        self
    }

    pub fn include(&'a mut self, param: &str) -> &'a mut BernardBrowse {
        self.includes.push(param.to_string());
        self
    }

    pub fn load<T>(&mut self) -> Box<Future<Item = Vec<T>, Error = hyper::Error>>
    where
        T: Entity,
    {
        let mut inc: String = String::new();

        for include in &self.includes {
            if inc.len() == 0 {
                inc = format!("{}", include)
            }
            else {
                inc = format!("{},{}", inc, include)
            }
        }

        if inc.len() > 0 {
            self.bernard.set_param("inc", &inc) ;
        }

        return T::browse(self.bernard);
    }
}
