CREATE TABLE "alunos" (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    username varchar(128) NOT NULL UNIQUE,
    nome varchar(128) NOT NULL,
    pwd varchar(256),
    pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
    token_salt uuid NOT NULL DEFAULT gen_random_uuid()
);