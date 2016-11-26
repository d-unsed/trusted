use ruru::Object;

use ruby::request::Observer;
use ruby::Response;
use ruby::Request;

class!(ProcessingPool);

impl ProcessingPool {
    pub fn process(&self, request: Request, response: Response, observer: Observer) {
        let args = vec![request.to_any_object(),
                        response.to_any_object(),
                        observer.to_any_object()];

        self.send("execute_future", args);
    }
}
