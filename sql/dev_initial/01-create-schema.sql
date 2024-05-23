CREATE TABLE "aluno" (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    nome varchar(128) NOT NULL
);