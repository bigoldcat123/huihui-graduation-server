-- Food attributes table
CREATE TABLE public.food_attribute (
    food_id integer NOT NULL REFERENCES public.food(id),
    calories numeric(10,2) DEFAULT 0 NOT NULL,
    protein numeric(10,2) DEFAULT 0 NOT NULL,
    fat numeric(10,2) DEFAULT 0 NOT NULL,
    carbohydrates numeric(10,2) DEFAULT 0 NOT NULL,
    fiber numeric(10,2) DEFAULT 0 NOT NULL,
    sugar numeric(10,2) DEFAULT 0 NOT NULL,
    sodium numeric(10,2) DEFAULT 0 NOT NULL,
    serving_size character varying(100) DEFAULT '' NOT NULL,
    is_vegetarian boolean DEFAULT false NOT NULL,
    is_vegan boolean DEFAULT false NOT NULL,
    is_gluten_free boolean DEFAULT false NOT NULL,
    allergens character varying(255) DEFAULT '' NOT NULL,
    PRIMARY KEY (food_id)
);

ALTER TABLE public.food_attribute OWNER TO admin;
