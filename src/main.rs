
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

mod components;
use components::recipe_card::RecipeCard;
use components::recipe::RecipeData;
use components::recipe_info::RecipeInfo;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/:id")]
    RecipeInfo { id: u32 },

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!( <RecipeList /> ),
        Route::RecipeInfo { id } => html!( <RecipeInfo id={id} /> ),
        Route::NotFound => html!( <h1>{"404, Not Found!"}</h1> ),
    }
}

#[function_component(RecipeList)]
fn recipe_list () -> Html {
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

    html!(
        <div class={classes!("recipe-list")}>
            { 
                recipes.iter().map(| recipe | {
                    html!{ <RecipeCard key={recipe.clone().id} recipe={recipe.clone()} /> }
                }).collect::<Html>()
            }
        </div>
    )
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
