CREATE TABLE "_user" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE "restaurant" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255) NOT NULL,
    image VARCHAR(255) NOT NULL
);

CREATE TABLE "food" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    restaurant_id int NOT NULL REFERENCES "restaurant"(id),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR(255) NOT NULL
);

CREATE TABLE "tag" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL UNIQUE,
    image VARCHAR(255) NOT NULL
);

CREATE TABLE "food_tag" (
    food_id int NOT NULL REFERENCES "food"(id),
    tag_id int NOT NULL REFERENCES "tag"(id),
    PRIMARY KEY (food_id, tag_id)
);

CREATE TABLE "operation" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id int NOT NULL REFERENCES "_user"(id),
    food_id int NOT NULL REFERENCES "food"(id),
    name VARCHAR(255) NOT NULL,
    weight FLOAT4 NOT NULL
);

CREATE TABLE "topic" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id int NOT NULL REFERENCES "_user"(id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    is_top Boolean NOT NULL DEFAULT TRUE,
    images TEXT NULL,
    create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "reply" (
    comment_id int NOT NULL REFERENCES "topic"(id),
    comment_to_id int NOT NULL REFERENCES "topic"(id),
    PRIMARY KEY (comment_id, comment_to_id)
);

CREATE TABLE "topic_like" (
    user_id int NOT NULL REFERENCES "_user"(id) ON DELETE CASCADE,
    topic_id int NOT NULL REFERENCES "topic"(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, topic_id)
);


CREATE TYPE suggestion_type AS ENUM (
    'ADD_FOOD',
    'UPDATE_FOOD',
    'OTHER'
);

CREATE TYPE suggestion_status AS ENUM (
    'PENDING',
    'APPROVED',
    'REJECTED',
    'PREPARING',
    'PROCESSING',
    'FINISHED'
);

CREATE TABLE "suggestion" (
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    content TEXT NOT NULL,
    images TEXT,
    type suggestion_type NOT NULL,
    status suggestion_status NOT NULL DEFAULT 'PENDING',
    food_id int REFERENCES "food"(id) ON DELETE SET NULL,
    restaurant_id int REFERENCES "restaurant"(id) ON DELETE SET NULL,
    reviewer_id int REFERENCES "_user"(id) ON DELETE SET NULL,
    review_comment TEXT,
    user_id int NOT NULL REFERENCES "_user"(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reviewed_at TIMESTAMP
);
