use std::{
    collections::{HashMap, HashSet},
};

use crate::{
    service::error::ServiceError,
    source::{self, food::FoodRow},
};

pub struct ItemCf {
    u_id: i32,
    n: usize,
}

impl ItemCf {
    pub async fn top_n(&self) -> Result<Vec<FoodRow>, ServiceError> {
        let candidate = self.candidate().await?;
        Ok(candidate)
    }

    async fn candidate(&self) -> Result<Vec<FoodRow>, ServiceError> {
        let all_foods = source::food::list_foods().await?;

        let sim_matrix = Self::sim_matrix(&all_foods).await?;

        let liked_food = source::food::list_user_liked_foods(self.u_id).await?;
        let mut map = HashMap::new();
        for f in all_foods.iter() {
            let food_id = f.id;
            let mut food_s = 0f32;
            for like_id in liked_food.iter().map(|x| x.id) {
                food_s += sim_matrix[&food_id][&like_id];
            }
            map.insert(food_id, food_s);
        }
        let mut map = map.into_iter().collect::<Vec<(i32, f32)>>();
        map.sort_by(|a, b| b.1.total_cmp(&a.1));
        let map = map.into_iter().take(self.n).collect::<HashMap<i32, f32>>();

        Ok(all_foods
            .into_iter()
            .filter(|x| map.contains_key(&x.id))
            .collect())
    }

    async fn sim_matrix(
        all_foods: &[FoodRow],
    ) -> Result<HashMap<i32, HashMap<i32, f32>>, ServiceError> {
        let mut w: HashMap<i32, HashMap<i32, f32>> = all_foods
            .iter()
            .map(|x| (x.id, all_foods.iter().map(|x| (x.id, 0.0_f32)).collect()))
            .collect();
        let op = source::operation::list_like_operations().await?;
        let mut food_user: HashMap<i32, HashSet<i32>> = HashMap::new();
        for o in op {
            food_user.entry(o.food_id).or_default().insert(o.u_id);
        }
        for i in all_foods.iter() {
            let i = &i.id;
            for j in all_foods.iter() {
                let j = &j.id;
                if i == j {
                    continue;
                }
                let coexist = food_user[i].intersection(&food_user[j]).count() as f32;

                *w.get_mut(i).unwrap().get_mut(j).unwrap() = coexist
                    / ((food_user[i].len() * food_user[j].len()) as f32)
                        .max(1.0)
                        .sqrt();
            }
        }
        Ok(w)
    }
}
