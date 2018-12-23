-- Your SQL goes here
CREATE TABLE public.users
(
    id serial NOT NULL,
    password character varying(128) NOT NULL,
    email character varying(255) NOT NULL,
    created timestamp with time zone NOT NULL,
    modified timestamp with time zone NOT NULL,
    settings jsonb,
    PRIMARY KEY (id),
    UNIQUE (email)

)
WITH (
    OIDS = FALSE
);
ALTER TABLE public.users
    OWNER to apartment_app;
