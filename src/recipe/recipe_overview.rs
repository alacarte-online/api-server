use anyhow::bail;
use serde::Serialize;

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
pub struct RecipeOverviewViewItem {
    pub recipe_id: Option<i64>,
    pub recipe_name: Option<String>,
    pub brief_description: Option<String>,
    pub image_uri: Option<String>,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
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