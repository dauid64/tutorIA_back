CREATE TABLE "usuario" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    pwd varchar(256)
);

CREATE TABLE "aluno" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    nome varchar(128) NOT NULL,

    usuario_id uuid UNIQUE NOT NULL,

    FOREIGN KEY (usuario_id) 
        REFERENCES usuario(id)
        ON DELETE CASCADE
);