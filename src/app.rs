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
    pub images: Option<Vec<AttachmentShort>>,
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
        Ok(records) => {
            info!("{:?}", records);
            Ok(records)
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn ShowData(data: Result<Vec<Record<Item>>, ServerFnError>) -> impl IntoView {
    let the_data = data.unwrap();
    view! {
        <div class="m-auto grid grid-cols-3 gap-4">
            {the_data.into_iter()
                // .map(|d| view! { <li>{d.fields.description}</li> })
                .map(|d| {
                    let item: Item = d.fields;
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
            <p class="text-2xl capitalize">{item.name}</p>
            <p class="text-sm">{item.description}</p>
            { if item.price.is_some() {
                    let price = item.price.unwrap();
                    view! { <p>{format!("${:.2}", price)}</p> }
                } else {
                    view! { <p></p> }
                }

            }
            { if item.images.is_some() {
                let images = item.images.unwrap();
                images.into_iter()
                    .map(|i| view! {<img src={i.url} />})
                    .collect_view()
                } else {
                    view! {}.into_view()
                }
            }
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let data = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <Title text="Unwedding Unregistry"/>
        <main class="mx-10 grid justify-items-center">
            <h1 class="text-5xl my-10">Unwedding Unregistry</h1>
            <div>
                <Suspense fallback=|| ()>
                        {move || match data.get() {
                            None => view! { <p>"Loading..."</p> }.into_view(),
                            Some(data) => {
                                view! { <ShowData data/> }.into_view()
                            }
                        }}
                </Suspense>
            </div>
        </main>
    }
}
