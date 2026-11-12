-- Your SQL goes here

CREATE TABLE public.token
(
    id SERIAL NOT NULL,
    tokenstr VARCHAR(255) NOT NULL UNIQUE, -- Token
    user_id Integer NOT NULL, -- ID of user
    CONSTRAINT token_pkey PRIMARY KEY (id)
);

CREATE TABLE public.accessrights
(
    tid Integer NOT NULL,
    path VARCHAR(255) NOT NULL, -- Path to give access to
    readonly Boolean NOT NULL, -- ReadAccess to give access to
    PRIMARY KEY(tid, path),
    CONSTRAINT accessrights_fk FOREIGN KEY(tid)  
    REFERENCES public.token(id) ON DELETE CASCADE
);