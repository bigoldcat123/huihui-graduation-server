

## food
id pk
name

## tag
id pk
name

## food_tag
food_id fk(food.id)
tag_id fk(tag.id)

## operation
id pk
user_id fk(user.id)
name
weight f32
