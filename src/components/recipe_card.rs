
use yew::prelude::*;
use super::recipe::RecipeData;

// use gloo_console::log;
// use wasm_bindgen::JsValue;

#[derive(Properties, Clone, PartialEq)]
pub struct RecipeCardProps {
    pub recipe: RecipeData,
}

#[function_component(RecipeCard)]
pub fn recipe_card(props: &RecipeCardProps) -> Html {
    let recipe = props.recipe.clone();

    // For Logging into browser console
    // let object = JsValue::from(props.recipe.name.clone());
    // log!(object);

    html!(
        <div class={classes!("recipe-card")}>
            <div class={classes!("data")}>
                <span class={classes!("name")}>{recipe.name.clone()}</span>
                <span class={classes!("tagline")}>{recipe.tagline}</span>
                <hr />
                <span class={classes!("description")}>{recipe.description}</span>

                <a href={format!("/{}", recipe.id)} class={classes!("read-more")} >{"Read more."}</a>
            </div>
            <img src={recipe.image_url} alt={recipe.name.clone()} />
        </div>
    )
}
