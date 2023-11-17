use crate::airtable::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::env;

// We will iterate through the references to the element returned by
// env::vars();
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
}

// #[derive(Clone, Debug, Default)]
// struct Cart {
//     items: Vec<CartItem>,
// }

// #[derive(Clone, Debug, Default)]
// struct CartItem {
//     item_name: String,
//     item_thumbnail_url: Option<String>,
//     quantity: i32,
// }

// #[derive(Clone, Debug, Default)]
// struct GlobalState {
//     items: Option<Vec<Item>>,
//     cart: Option<Cart>,
// }

#[server(LoadData, "/api", "GetJson")]
pub async fn load_data() -> Result<Vec<Record<Item>>, ServerFnError> {
    // Initialize the Airtable client.
    let airtable = Airtable::new_from_env();

    // Get the current records from a table.
    match airtable.list_records::<Item>("items", "Grid view").await {
        Ok(records) => Ok(records),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

pub fn get_categories(items: Vec<Record<Item>>) -> Vec<String> {
    let mut categories: Vec<String> = items
        .into_iter()
        .map(|i| i.fields.categories)
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .flatten()
        .collect();

    categories.sort();
    categories.dedup();

    categories
}

#[component]
pub fn App() -> impl IntoView {
    let initial_items: Vec<Record<Item>> = vec![];
    let (items, set_items) = create_signal(initial_items);
    let data = create_resource(|| (), |_| async move { load_data().await });
    // let envs = env::vars();
    // info!("{:?}", envs);

    view! {
        // <Stylesheet id="leptos" href="/pkg/unwedding-unregistry.css"/>
        <Link rel="stylesheet" href="/pkg/unwedding-unregistry.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css" />
        <Router>
            <div class="absolute top-0 right-0 m-10"><a href="/cart">Cart Icon</a></div>
            <nav class="flex h-40 p-8">
                <div class="flex items-start self-stretch flex-col justify-center">
                    <a href="/" class="text-6xl text-gray-800 font-bold">Goodbye Stuff</a>
                    <span class="text-lime-500 font-bold">starting a new life, getting rid of our things</span>
                </div>
                <div class="flex-1 my-auto flex-grow">
                    <ul class="flex justify-center font-bold text-gray-700">
                        <li><a href="/" class="mx-5">Why</a></li>
                        <li><a href="/stuff" class="mx-5">Stuff</a></li>
                    </ul>
                </div>
            </nav>
            <main class="flex px-8 pt-10 pb-24 flex-col items-center self-stretch text-gray-800">
                <Routes>
                    <Route path="" view=  move || view! { <Home/> }/>
                    <Route path="stuff" view=  move || view! { <Stuff stuff=items()/> }/>
                    <Route path="cart" view=  move || view! { <Cart/> }/>
                </Routes>
                <Suspense fallback=|| ()>
                        {move || match data.get() {
                            None => { view! { <p>"Loading..."</p> }.into_view()},
                            Some(data) => {
                                // fix this
                                match data {
                                    Ok(data) => {
                                        set_items(data);
                                        view! { "" }.into_view()
                                    },
                                    Err(e) => {
                                        info!("{:?}", e);
                                        view! { "" }.into_view()
                                    },
                                }
                            }
                        }}
                </Suspense>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <p>The Home Page</p> }
}

#[component]
fn Stuff(stuff: Vec<Record<Item>>) -> impl IntoView {
    view! {
        <div class="flex items-start">
            <ShowCategories categories=get_categories(stuff.clone())/>
            <ShowData stuff=stuff.clone() />
        </div>
    }
}

#[component]
fn Cart() -> impl IntoView {
    view! { <p>The cart page</p> }
}

#[component]
pub fn ShowCategories(categories: Vec<String>) -> impl IntoView {
    view! {
        <div class="pr-10">
            <ul>
                {categories.into_iter()
                    .map(|c| view! {
                        <li class="mb-3 w-full">
                            <a href="#" class="text-white bg-gray-400 font-bold rounded px-3 py-1">{c}</a>
                        </li>
                    } )
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn ShowData(stuff: Vec<Record<Item>>) -> impl IntoView {
    view! {
        <div class="flex-1">
            <div class="m-auto grid grid-cols-6 gap-4">
                {stuff.into_iter()
                    .filter(|i| i.fields.publish.is_some())
                    .map(|i| {
                        let item: Item = i.fields;
                        view! {
                            <ItemForSale item /> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn ItemForSale(item: Item) -> impl IntoView {
    // info!("{:?}", item);
    view! {
        <div class="bg-gray-100 p-3 rounded">
            { if item.images.is_some() {
                let images = item.images.unwrap();
                images.first()
                    .cloned()
                    .into_iter()
                    .map(|i| view! {<img src={i.thumbnails.large.url} class="rounded" />})
                    .collect_view()
                } else {
                    view! {}.into_view()
                }
            }
            <p class="text-2xl capitalize font-bold text-center m-2 text-gray-800">{item.name}</p>
            <div class="flex items-start self-stretch p-2">
                <p class="text-sm items-center flex-1">{item.description}</p>
                { if item.price.is_some() {
                        let price = item.price.unwrap();
                        view! { <p class="font-bold text-2xl">{format!("${:.2}", price)}</p> }
                    } else {
                        view! { <p>tbd</p> }
                    }

                }
            </div>
        </div>
    }
}
