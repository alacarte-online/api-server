use anyhow::bail;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct RecipeDetails {
    pub recipe_id: i64,
    pub recipe_name: String,
    pub brief_description: String,
    pub method: String,
    pub image_uri: Option<String>,
    pub user_id: i64,
    pub user_name: String,
}

#[derive(Debug, Serialize)]
struct RecipeDetailsViewItem {
    pub recipe_id: Option<i64>,
    pub recipe_name: Option<String>,
    pub brief_description: Option<String>,
    pub method: Option<String>,
    pub image_uri: Option<String>,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
}

impl RecipeDetails {
    pub async fn fetch_from_recipe_id(db_pool: &PgPool, recipe_id: i64) -> anyhow::Result<Option<Self>> {
        let recipe_details = sqlx::query_as!(RecipeDetailsViewItem, "SELECT * FROM recipe_details WHERE recipe_id = $1;", recipe_id).fetch_optional(db_pool).await?;
        let recipe_details: Option<RecipeDetails> = match recipe_details {
            Some(recipe) => Some(recipe.try_into()?),
            None => None
        };
        Ok(recipe_details)
    }
}

impl TryFrom<RecipeDetailsViewItem> for RecipeDetails {
    type Error = anyhow::Error;
    fn try_from(value: RecipeDetailsViewItem) -> Result<Self, Self::Error> {
        let recipe_id = match value.recipe_id {
            Some(id) => id,
            None => bail!("RecipeOverviewViewItem missing recipe_id"),
        };
        let recipe_name = match value.recipe_name {
            Some(name) => name,
            None => bail!("RecipeOverviewViewItem missing recipe_name"),
        };
        let brief_description = match value.brief_description {
            Some(description) => description,
            None => bail!("RecipeOverviewViewItem missing brief_description"),
        };
        let method = match value.method {
            Some(method) => method,
            None => bail!("RecipeOverviewViewItem missing method"),
        };
        let image_uri = value.image_uri;
        let user_id = match value.user_id {
            Some(id) => id,
            None => bail!("RecipeOverviewViewItem missing user_id"),
        };
        let user_name = match value.user_name {
            Some(name) => name,
            None => bail!("RecipeOverviewViewItem missing user_name"),
        };

        Ok(RecipeDetails {
            recipe_id,
            recipe_name,
            brief_description,
            method,
            image_uri,
            user_id,
            user_name
        })
    }
}