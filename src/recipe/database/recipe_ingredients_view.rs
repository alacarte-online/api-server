use anyhow::bail;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct RecipeIngredientsView {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub recipe_name: String,
    pub ingredient_name: String,
    pub amount: String
}

#[derive(Debug, Serialize)]
struct RecipeIngredientsViewItem {
    pub recipe_id: Option<i64>,
    pub ingredient_id: Option<i64>,
    pub recipe_name: Option<String>,
    pub ingredient_name: Option<String>,
    pub amount: Option<String>
}

impl TryFrom<RecipeIngredientsViewItem> for RecipeIngredientsView {
    type Error = anyhow::Error;
    fn try_from(value: RecipeIngredientsViewItem) -> Result<Self, Self::Error> {
        let recipe_id = match value.recipe_id {
            Some(id) => id,
            None => bail!("RecipeIngredientsViewItem missing recipe_id"),
        };
        let ingredient_id = match value.ingredient_id {
            Some(id) => id,
            None => bail!("RecipeIngredientsViewItem missing ingredient_id"),
        };
        let recipe_name = match value.recipe_name {
            Some(name) => name,
            None => bail!("RecipeIngredientsViewItem missing recipe_name"),
        };
        let ingredient_name = match value.ingredient_name {
            Some(name) => name,
            None => bail!("RecipeIngredientsViewItem missing ingredient_name"),
        };
        let amount = match value.amount {
            Some(amount) => amount,
            None => bail!("RecipeIngredientsViewItem missing amount"),
        };

        Ok(RecipeIngredientsView {
            recipe_id,
            ingredient_id,
            recipe_name,
            ingredient_name,
            amount
        })
    }
}

impl RecipeIngredientsView {
    pub async fn fetch_from_recipe_id(db_pool: &PgPool, recipe_id: i64) -> anyhow::Result<Option<Vec<Self>>> {
        let recipe_ingredients = sqlx::query_as!(RecipeIngredientsViewItem, "SELECT * FROM recipe_ingredients_list WHERE recipe_id = $1;", recipe_id).fetch_all(db_pool).await?;
        if recipe_ingredients.len() == 0 {
            return Ok(None);
        }

        let mut recipe_ingredients_vec = vec![];
        for item in recipe_ingredients {
            match item.try_into() {
                Ok(recipe_ingredients) => recipe_ingredients_vec.push(recipe_ingredients),
                Err(err) => println!("{}", err)
            }
        }
        Ok(Some(recipe_ingredients_vec))
    }
}