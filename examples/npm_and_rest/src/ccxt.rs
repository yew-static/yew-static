use stdweb::Value;
use stdweb::unstable::TryInto;

pub struct CcxtService(Option<Value>);

impl CcxtService {
    pub fn new() -> Self {
        let lib = js! {
            return ccxt;
        };
        CcxtService(Some(lib))
    }

    pub fn exchanges(&mut self) -> Vec<String> {
        let lib = self.0.as_ref().expect("ccxt library object lost");
        let v: Value = js! {
            var ccxt = @{lib};
            console.log(ccxt.exchanges);
            return ccxt.exchanges;
        };
        let v: Vec<String> = v.try_into().unwrap();
        v
    }
}