CREATE TABLE "usuario" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    pwd varchar(256)
);

CREATE TABLE "professor" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    nome varchar(128) NOT NULL,
    usuario_id uuid UNIQUE NOT NULL,


    FOREIGN KEY (usuario_id) 
        REFERENCES usuario(id)
        ON DELETE CASCADE
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

CREATE TABLE "materia" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    nome varchar(128) NOT NULL,
    descricao TEXT NOT NULL,
    professor_id uuid NOT NULL,
    conteudos TEXT[] NOT NULL,

    FOREIGN KEY (professor_id) 
        REFERENCES professor(id)
        ON DELETE CASCADE
);

CREATE TABLE "aluno_materia" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    aluno_id uuid NOT NULL,
    materia_id uuid NOT NULL,
    PRIMARY KEY (aluno_id, materia_id),
    FOREIGN KEY (aluno_id) REFERENCES aluno(id),
    FOREIGN KEY (materia_id) REFERENCES materia(id)
);

CREATE TABLE "tutor" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    nome varchar(128) NOT NULL,
    materia_id uuid NOT NULL,
    assistant_id varchar(256) NOT NULL,

    FOREIGN KEY (materia_id)
        REFERENCES materia(id)
        ON DELETE CASCADE
);

CREATE TABLE "chat" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    aluno_id uuid NOT NULL,
    tutor_id uuid NOT NULL,

    thread_id varchar(256) NOT NULL UNIQUE,

    FOREIGN KEY (aluno_id) 
        REFERENCES aluno(id)
        ON DELETE CASCADE,
    
    FOREIGN KEY (tutor_id) 
        REFERENCES tutor(id)
        ON DELETE CASCADE
);

CREATE TABLE "mensagem" (
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    conteudo TEXT NOT NULL,
    chat_id uuid NOT NULL,
    tipo varchar(50) NOT NULL,
    
    FOREIGN KEY (chat_id) 
        REFERENCES chat(id)
        ON DELETE CASCADE
);