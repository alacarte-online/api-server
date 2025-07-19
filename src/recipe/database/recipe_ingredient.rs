use sqlx::PgPool;

pub struct RecipeIngredientToInsert {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub amount: String
}

impl RecipeIngredientToInsert {
    pub async fn insert_into_db(&self, db_pool: &PgPool) -> anyhow::Result<()> {
        let _result = sqlx::query!("INSERT INTO recipe_ingredients
            (recipe_id, ingredient_id, amount)
            VALUES
            ($1, $2, $3);", self.recipe_id, self.ingredient_id, self.amount).execute(db_pool).await?;
        Ok(())
    }
}