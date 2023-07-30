
use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::recipe::RecipeData;

// use gloo_console::log;
// use wasm_bindgen::JsValue;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32
}

#[function_component(RecipeInfo)]
pub fn recipe_info(props: &Props) -> Html {

    let recipe: UseStateHandle<RecipeData> = use_state(|| RecipeData::default());
    let recipe_id = props.id.to_string();
    let recipe_url = format!("https://api.punkapi.com/v2/beers/{}", recipe_id);

    {
        let recipe = recipe.clone();

        use_effect_with_deps(move |_| {
            let recipe = recipe.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_recipe: Vec<RecipeData> = Request::get(&recipe_url)
                    .send()
                    .await.expect("Unable to get the recipe")
                    .json()
                    .await.expect("Unable to parse the data to json");

                recipe.set(fetched_recipe[0].clone());
            });
            || ()
        }, ());
    }

    html!(
        <div class={classes!("container")}>
            <pre>
                {recipe.get_string()}
            </pre>
        </div>
    )
}
