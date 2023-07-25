
use yew::prelude::*;
use gloo_net::http::Request;

mod components;
use components::recipe::{RecipeCard, RecipeData};

#[function_component]
fn App() -> Html {

    let recipes: UseStateHandle<Vec<RecipeData>> = use_state(|| vec![]);

    {
        let recipes = recipes.clone();

        use_effect_with_deps(move |_| {
            let recipes = recipes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_recipes: Vec<RecipeData> = Request::get("https://api.punkapi.com/v2/beers")
                    .send()
                    .await.expect("Unable to process the request")
                    .json()
                    .await.expect("Error occured unable to parse to json");
                println!("{:?}", fetched_recipes.first());
                recipes.set(fetched_recipes);
            });
            || ()
        }, ());
    }

    html! {
        <div class={classes!("recipe-list")}>
            { 
                recipes.iter().map(| recipe | {
                    html!{ <RecipeCard key={recipe.clone().id} recipe={recipe.clone()} /> }
                }).collect::<Html>()
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
