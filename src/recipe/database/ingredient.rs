use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct IngredientToInsert {
    pub name: String
}

impl Ingredient {
    pub async fn try_fetch_from_ingredient_name(db_pool: &PgPool, ingredient_name: &str) -> anyhow::Result<Option<Self>> {
        let ingredient_details = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients WHERE name = $1;", ingredient_name).fetch_optional(db_pool).await?;
        let ingredient_details = match ingredient_details {
            Some(ingredient_details) => ingredient_details,
            None => return Ok(None)
        };
        Ok(Some(ingredient_details))
    }
}

impl IngredientToInsert {
    pub async fn insert_into_db(&self, db_pool: &PgPool) -> anyhow::Result<i64> {
        let inserted_ingredient = sqlx::query!("INSERT INTO ingredients
        (name)
        VALUES ($1)
        RETURNING id;", self.name).fetch_one(db_pool).await?;
        Ok(inserted_ingredient.id)
    }
}