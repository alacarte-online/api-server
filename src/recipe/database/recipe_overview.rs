use anyhow::bail;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct RecipeOverview {
    pub recipe_id: i64,
    pub recipe_name: String,
    pub brief_description: String,
    pub image_uri: String,
    pub user_id: i64,
    pub user_name: String,
}

#[derive(Debug, Serialize)]
struct RecipeOverviewViewItem {
    pub recipe_id: Option<i64>,
    pub recipe_name: Option<String>,
    pub brief_description: Option<String>,
    pub image_uri: Option<String>,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
}

impl RecipeOverview {
    pub async fn get_all_recipe_overviews(db_pool: &PgPool) -> anyhow::Result<Vec<RecipeOverview>> {
        let recipes = sqlx::query_as!(RecipeOverviewViewItem, "SELECT * FROM recipe_overviews;").fetch_all(db_pool).await?;
        let mut recipe_vec = vec![];
        for recipe in recipes {
            match recipe.try_into() {
                Ok(recipe) => recipe_vec.push(recipe),
                Err(err) => println!("{}", err)
            }
        }
        Ok(recipe_vec)
    }

    pub async fn get_recipe_overview(recipe_id: i64, db_pool: &PgPool) -> anyhow::Result<Option<RecipeOverview>> {
        let recipe = sqlx::query_as!(RecipeOverviewViewItem,
            "SELECT * FROM recipe_overviews WHERE recipe_id = $1;", recipe_id).fetch_optional(db_pool).await?;
        match recipe {
            Some(recipe) => {
                let recipe = recipe.try_into()?;
                Ok(Some(recipe))
            },
            None => Ok(None)
        }
    }
}

impl TryFrom<RecipeOverviewViewItem> for RecipeOverview {
    type Error = anyhow::Error;

    fn try_from(value: RecipeOverviewViewItem) -> Result<Self, Self::Error> {
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
        let image_uri = match value.image_uri {
            Some(image) => image,
            None => bail!("RecipeOverviewViewItem missing image_uri"),
        };
        let user_id = match value.user_id {
            Some(id) => id,
            None => bail!("RecipeOverviewViewItem missing user_id"),
        };
        let user_name = match value.user_name {
            Some(name) => name,
            None => bail!("RecipeOverviewViewItem missing user_name"),
        };

        Ok(RecipeOverview {
            recipe_id,
            recipe_name,
            brief_description,
            image_uri,
            user_id,
            user_name
        })
    }
}