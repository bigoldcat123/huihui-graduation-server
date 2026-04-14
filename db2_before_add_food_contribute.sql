--
-- PostgreSQL database dump
--


-- Dumped from database version 18.2 (Debian 18.2-1.pgdg13+1)
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: pg_database_owner
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO pg_database_owner;

--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: pg_database_owner
--

COMMENT ON SCHEMA public IS 'standard public schema';


--
-- Name: suggestion_status; Type: TYPE; Schema: public; Owner: admin
--

CREATE TYPE public.suggestion_status AS ENUM (
    'PENDING',
    'APPROVED',
    'REJECTED',
    'PREPARING',
    'PROCESSING',
    'FINISHED'
);


ALTER TYPE public.suggestion_status OWNER TO admin;

--
-- Name: suggestion_type; Type: TYPE; Schema: public; Owner: admin
--

CREATE TYPE public.suggestion_type AS ENUM (
    'ADD_FOOD',
    'UPDATE_FOOD',
    'OTHER'
);


ALTER TYPE public.suggestion_type OWNER TO admin;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _user; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public._user (
    id integer NOT NULL,
    email character varying(255) NOT NULL,
    username character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    profile character varying(255)
);


ALTER TABLE public._user OWNER TO admin;

--
-- Name: _user_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public._user ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public._user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: food; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.food (
    id integer NOT NULL,
    restaurant_id integer NOT NULL,
    name character varying(255) NOT NULL,
    description text NOT NULL,
    image character varying(255) NOT NULL,
    price numeric(10,2) DEFAULT 0.0 NOT NULL
);


ALTER TABLE public.food OWNER TO admin;

--
-- Name: food_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.food ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.food_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: food_tag; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.food_tag (
    food_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE public.food_tag OWNER TO admin;

--
-- Name: operation; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.operation (
    id integer NOT NULL,
    user_id integer NOT NULL,
    food_id integer NOT NULL,
    name character varying(255) NOT NULL,
    weight real NOT NULL
);


ALTER TABLE public.operation OWNER TO admin;

--
-- Name: operation_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.operation ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.operation_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: reply; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.reply (
    comment_id integer NOT NULL,
    comment_to_id integer NOT NULL
);


ALTER TABLE public.reply OWNER TO admin;

--
-- Name: restaurant; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.restaurant (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    location character varying(255) NOT NULL,
    image character varying(255) NOT NULL
);


ALTER TABLE public.restaurant OWNER TO admin;

--
-- Name: restaurant_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.restaurant ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.restaurant_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: suggestion; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.suggestion (
    id integer NOT NULL,
    content text NOT NULL,
    images text,
    type public.suggestion_type NOT NULL,
    status public.suggestion_status DEFAULT 'PENDING'::public.suggestion_status NOT NULL,
    food_id integer,
    restaurant_id integer,
    reviewer_id integer,
    review_comment text,
    user_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    reviewed_at timestamp with time zone
);


ALTER TABLE public.suggestion OWNER TO admin;

--
-- Name: suggestion_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.suggestion ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.suggestion_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: tag; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.tag (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    image character varying(255) NOT NULL
);


ALTER TABLE public.tag OWNER TO admin;

--
-- Name: tag_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.tag ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.tag_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: todo_log; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.todo_log (
    id integer NOT NULL,
    suggestion_id integer NOT NULL,
    suggestion_status public.suggestion_status NOT NULL,
    content text NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.todo_log OWNER TO admin;

--
-- Name: todo_log_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.todo_log ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.todo_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: topic; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.topic (
    id integer NOT NULL,
    user_id integer NOT NULL,
    title character varying(255) NOT NULL,
    content text NOT NULL,
    images text,
    create_at timestamp with time zone DEFAULT '2026-02-13 05:24:35.438949'::timestamp without time zone NOT NULL,
    is_top boolean DEFAULT true NOT NULL,
    deleted boolean DEFAULT false NOT NULL
);


ALTER TABLE public.topic OWNER TO admin;

--
-- Name: topic_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

ALTER TABLE public.topic ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.topic_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: topic_like; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.topic_like (
    user_id integer NOT NULL,
    topic_id integer NOT NULL
);


ALTER TABLE public.topic_like OWNER TO admin;

--
-- Name: _user _user_email_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public._user
    ADD CONSTRAINT _user_email_key UNIQUE (email);


--
-- Name: _user _user_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public._user
    ADD CONSTRAINT _user_pkey PRIMARY KEY (id);


--
-- Name: _user _user_username_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public._user
    ADD CONSTRAINT _user_username_key UNIQUE (username);


--
-- Name: food food_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.food
    ADD CONSTRAINT food_pkey PRIMARY KEY (id);


--
-- Name: food_tag food_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.food_tag
    ADD CONSTRAINT food_tag_pkey PRIMARY KEY (food_id, tag_id);


--
-- Name: operation operation_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.operation
    ADD CONSTRAINT operation_pkey PRIMARY KEY (id);


--
-- Name: reply reply_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.reply
    ADD CONSTRAINT reply_pkey PRIMARY KEY (comment_id, comment_to_id);


--
-- Name: restaurant restaurant_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.restaurant
    ADD CONSTRAINT restaurant_pkey PRIMARY KEY (id);


--
-- Name: suggestion suggestion_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.suggestion
    ADD CONSTRAINT suggestion_pkey PRIMARY KEY (id);


--
-- Name: tag tag_name_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT tag_name_key UNIQUE (name);


--
-- Name: tag tag_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (id);


--
-- Name: todo_log todo_log_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.todo_log
    ADD CONSTRAINT todo_log_pkey PRIMARY KEY (id);


--
-- Name: topic_like topic_like_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.topic_like
    ADD CONSTRAINT topic_like_pkey PRIMARY KEY (user_id, topic_id);


--
-- Name: topic topic_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.topic
    ADD CONSTRAINT topic_pkey PRIMARY KEY (id);


--
-- Name: food food_restaurant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.food
    ADD CONSTRAINT food_restaurant_id_fkey FOREIGN KEY (restaurant_id) REFERENCES public.restaurant(id);


--
-- Name: food_tag food_tag_food_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.food_tag
    ADD CONSTRAINT food_tag_food_id_fkey FOREIGN KEY (food_id) REFERENCES public.food(id);


--
-- Name: food_tag food_tag_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.food_tag
    ADD CONSTRAINT food_tag_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.tag(id);


--
-- Name: operation operation_food_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.operation
    ADD CONSTRAINT operation_food_id_fkey FOREIGN KEY (food_id) REFERENCES public.food(id);


--
-- Name: operation operation_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.operation
    ADD CONSTRAINT operation_user_id_fkey FOREIGN KEY (user_id) REFERENCES public._user(id);


--
-- Name: reply reply_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.reply
    ADD CONSTRAINT reply_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.topic(id);


--
-- Name: reply reply_comment_to_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.reply
    ADD CONSTRAINT reply_comment_to_id_fkey FOREIGN KEY (comment_to_id) REFERENCES public.topic(id);


--
-- Name: suggestion suggestion_food_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.suggestion
    ADD CONSTRAINT suggestion_food_id_fkey FOREIGN KEY (food_id) REFERENCES public.food(id) ON DELETE SET NULL;


--
-- Name: suggestion suggestion_restaurant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.suggestion
    ADD CONSTRAINT suggestion_restaurant_id_fkey FOREIGN KEY (restaurant_id) REFERENCES public.restaurant(id) ON DELETE SET NULL;


--
-- Name: suggestion suggestion_reviewer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.suggestion
    ADD CONSTRAINT suggestion_reviewer_id_fkey FOREIGN KEY (reviewer_id) REFERENCES public._user(id) ON DELETE SET NULL;


--
-- Name: suggestion suggestion_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.suggestion
    ADD CONSTRAINT suggestion_user_id_fkey FOREIGN KEY (user_id) REFERENCES public._user(id);


--
-- Name: todo_log todo_log_suggestion_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.todo_log
    ADD CONSTRAINT todo_log_suggestion_id_fkey FOREIGN KEY (suggestion_id) REFERENCES public.suggestion(id) ON DELETE CASCADE;


--
-- Name: topic_like topic_like_topic_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.topic_like
    ADD CONSTRAINT topic_like_topic_id_fkey FOREIGN KEY (topic_id) REFERENCES public.topic(id) ON DELETE CASCADE;


--
-- Name: topic_like topic_like_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.topic_like
    ADD CONSTRAINT topic_like_user_id_fkey FOREIGN KEY (user_id) REFERENCES public._user(id) ON DELETE CASCADE;


--
-- Name: topic topic_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.topic
    ADD CONSTRAINT topic_user_id_fkey FOREIGN KEY (user_id) REFERENCES public._user(id);
