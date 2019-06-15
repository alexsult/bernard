extern crate uuid;

use futures::Future;
use Bernard;
use Entity;

#[derive(Debug)]
pub struct BernardSearch<'a> {
    bernard: &'a mut Bernard,
    query_param: &'a str
}

impl<'a> BernardSearch<'a> {
    pub fn new(
        bernard: &'a mut Bernard,
        query_param: &'a str
    ) -> BernardSearch<'a> {

        BernardSearch {
            bernard: bernard,
            query_param: query_param
        }
    }

    pub fn load<T>(&mut self) -> Box<Future<Item = Vec<T>, Error = hyper::Error>>
    where
        T: Entity,
    {
        self.bernard.set_param("query", self.query_param) ;
        return T::search(self.bernard);
    }
}
