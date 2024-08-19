mod chromiums;

use reqwest::Client;

#[tokio::main]
async fn main(){
    let client = Client::new();
    let response = client.get("https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=&carCode=")
        .send().await.unwrap();
    let html_content = response.text().await.unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let product_selector = scraper::Selector::parse("a.owl-carousel").unwrap();
    let collected = document
        .select(&product_selector)
        .map(| value | value.inner_html())
        .collect::<Vec<String>>();

    for e in collected {
        println!("{}", e)
    }


    // let html_products = document.select(&html_product_selector);
    // // println!("{:?}", html_products);
    // for product in html_products{
    //
    //      let url = product
    //         .select(&scraper::Selector::parse("a").unwrap())
    //         .next()
    //         .map(| a |a.value().attr("href"));




        // let product_name = product
        //     .select(&scraper::Selector::parse("strong").unwrap())
        //     .next()
        //     .map(| h2 |h2.text().collect::<String>());
        //
        // let product_price = product
        //     .select(&scraper::Selector::parse(".price").unwrap())
        //     .next()
        //     .map(| h2 |h2.text().collect::<String>());

        // println!("url = {:?}",  url);
    // }
}
