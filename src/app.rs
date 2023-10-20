use crate::airtable::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde::{Deserialize, Serialize};

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
}

#[derive(Clone, Debug, Default)]
struct Cart {
    items: Vec<CartItem>,
}

#[derive(Clone, Debug, Default)]
struct CartItem {
    item_name: String,
    item_thumbnail_url: Option<String>,
    quantity: i32,
}

#[derive(Clone, Debug, Default)]
struct GlobalState {
    items: Option<Vec<Item>>,
    cart: Option<Cart>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

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

#[component]
pub fn ShowCategories(categories: Vec<String>) -> impl IntoView {
    view! {
        <div class="col-span-full">
            <ul>
                {categories.into_iter()
                    .map(|c| view! { <li>{c}</li>} )
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn ShowData(stuff: Vec<Record<Item>>) -> impl IntoView {
    // let items = use_context::<ReadSignal<Vec<Record<Item>>>>().unwrap();
    view! {
        <div class="m-auto grid grid-cols-3 gap-4">
            {stuff.into_iter()
                // .map(|d| view! { <li>{d.fields.description}</li> })
                .map(|i| {
                    let item: Item = i.fields;
                    view! {
                        <ItemForSale item /> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn ItemForSale(item: Item) -> impl IntoView {
    view! {
        <div class="border-gray-700 border-2 p-10">
            { if item.images.is_some() {
                let images = item.images.unwrap();
                images.into_iter()
                    .map(|i| view! {<img src={i.thumbnails.large.url} />})
                    .collect_view()
                } else {
                    view! {}.into_view()
                }
            }
            <p class="text-2xl capitalize">{item.name}</p>
            <p class="text-sm">{item.description}</p>
            { if item.price.is_some() {
                    let price = item.price.unwrap();
                    view! { <p>{format!("${:.2}", price)}</p> }
                } else {
                    view! { <p></p> }
                }

            }
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let initial_items: Vec<Record<Item>> = vec![];
    let (items, set_items) = create_signal(initial_items);
    let data = create_resource(|| (), |_| async move { load_data().await });
    provide_context(items);

    view! {
        <Title text="Unwedding Unregistry"/>
        <main class="mx-10 grid justify-items-center">
            <h1 class="text-5xl my-10">Unwedding Unregistry</h1>
            <div>
                <Suspense fallback=|| ()>
                        {move || match data.get() {
                            None => view! { <p>"Loading..."</p> }.into_view(),
                            Some(data) => {
                                set_items(data.unwrap());
                                let all_stuff = items();
                                let mut categories: Vec<String> = all_stuff
                                    .into_iter()
                                    .map(|i| i.fields.categories)
                                    .filter(|c| c.is_some())
                                    .map(|c| c.unwrap())
                                    .flatten()
                                    .collect();
                                categories.sort();
                                categories.dedup();

                                let stuff = items();

                                view! {
                                    <ShowCategories categories />
                                    <ShowData stuff />
                                }.into_view()
                            }
                        }}
                </Suspense>
            </div>
        </main>
    }
}
