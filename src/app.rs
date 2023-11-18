use crate::airtable::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::env;

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

fn get_items_by_category(items: Vec<Record<Item>>, category: Option<String>) -> Vec<Record<Item>> {
    let published = items
        .clone()
        .into_iter()
        .filter(|i| i.fields.publish.is_some())
        .collect();

    match category {
        None => published,
        Some(c) => published
            .clone()
            .into_iter()
            .filter(|p| {
                p.clone()
                    .fields
                    .categories
                    .unwrap()
                    .into_iter()
                    .rfind(|i| i == &c)
                    .is_some()
            })
            .collect(),
    }
}

#[component]
pub fn App() -> impl IntoView {
    let initial_items: Vec<Record<Item>> = vec![];
    let (items, set_items) = create_signal(initial_items);
    let data = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <Stylesheet id="leptos" href="/pkg/unwedding-unregistry.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css" />
        <Router>
            <div class="absolute top-0 right-0 m-10"><a href="/cart"><img src="/cart.svg"/></a></div>
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
                                Err(_) => {
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
    view! { <img src="/home-page.png" /> }
}

#[component]
fn Stuff(stuff: Vec<Record<Item>>) -> impl IntoView {
    let initial_category: Option<String> = None;
    let (category, set_category) = create_signal(initial_category);
    let categories = get_categories(stuff.clone());
    let items = get_items_by_category(stuff.clone(), Some("sports".to_string())); //TODO
    info!("ITEMS: {:?}", items);
    view! {
        <div class="flex items-start">
            <div class="pr-10">
                <ul>
                    {categories.into_iter()
                        .map(|c| {
                            let text = c.clone();
                            view! {
                                <li class="mb-3 w-full">
                                    <button
                                        class="text-white bg-gray-400 font-bold rounded px-3 py-1"
                                        on:click=move |_| {
                                            set_category(Some(c.to_string()));
                                            info!("{:?}", category().unwrap());
                                        }
                                    >{text}</button>
                                </li>
                            }
                        }).collect_view()}

                </ul>
            </div>
            <div class="flex-1">
                <div class="m-auto grid grid-cols-6 gap-4">
                    // TODO: fix reactivity
                    {items.into_iter()
                        .filter(|i| i.fields.publish.is_some())
                        .map(|i| {
                            let item: Item = i.fields;
                            view! {
                                <ItemForSale item /> }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Cart() -> impl IntoView {
    view! { <p>The cart page</p> }
}

#[component]
pub fn ItemForSale(item: Item) -> impl IntoView {
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
