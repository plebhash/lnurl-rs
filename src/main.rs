mod lib;

use std::str::FromStr;
use lightning_invoice::Bolt11Invoice;

use lib::lightning_address::LightningAddress;
use lib::Builder;
use lib::LnUrlResponse::LnUrlPayResponse;

#[tokio::main]
async fn main() {

    // todo
    let ln_addr = LightningAddress::from_str("ben@zaps.benthecarman.com").unwrap();
    let async_client = Builder::default().build_async().unwrap();

    let url = ""; // todo
    let res = async_client.make_request(url).await.unwrap();

    if let LnUrlPayResponse(pay) = res {
        let msats = 1_000_000;
        let pay_result = async_client.get_invoice(&pay, msats, None, None).await.unwrap();

        let invoice = Bolt11Invoice::from_str(&pay_result.invoice()).unwrap();

        assert_eq!(invoice.amount_milli_satoshis(), Some(msats));
    } else {
        panic!("Wrong response type");
    }
}