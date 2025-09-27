use http::Response;
use sqlx::PgPool;
use crate::http::responses::{internal_server_error_response, json_ok};
use crate::recipe::database::RecipeOverview;

struct ScoredRecipeOverview {
    overview: RecipeOverview,
    score: u16
}

pub async fn get_all_recipes(db_pool: &PgPool) -> Response<Vec<u8>> {
    let recipes = RecipeOverview::get_all_recipe_overviews(db_pool).await;
    let score_sorted_recipes = match recipes {
        Ok(recipes) => Ok(score_recipes(recipes)),
        Err(err) => Err(err)
    };
    let json = match score_sorted_recipes {
        Ok(recipe_vec) => serde_json::to_string(&recipe_vec),
        Err(err) => {
            log::info!("Error handling get all recipes request: {}", err);
            return internal_server_error_response()
        }
    };

    match json {
        Ok(json) => json_ok(json),
        Err(err) => {
            log::info!("Error handling get all recipes request: {}", err);
            internal_server_error_response()
        }
    }
}

fn score_recipes(recipes: Vec<RecipeOverview>) -> Vec<RecipeOverview> {
    let mut scored_recipes = recipes.into_iter().map(|recipe_overview| ScoredRecipeOverview {
        score: score_recipe(&recipe_overview),
        overview: recipe_overview
    }).collect::<Vec<ScoredRecipeOverview>>();
    scored_recipes.sort_by(|a, b| b.score.cmp(&a.score)); // Sort high to low
    scored_recipes.into_iter().map(|recipe_overview| recipe_overview.overview).collect()
}

fn score_recipe(recipe: &RecipeOverview) -> u16 {
    let mut score: u16 = 0;
    if recipe.image_uri.is_some() {
        score += 100;
    }
    score
}

