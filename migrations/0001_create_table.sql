-- Criacao da tabela de usuario
CREATE TABLE IF NOT EXISTS usuario (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    email TEXT NOT NULL,
    cpf TEXT NOT NULL,
    senha TEXT NOT NULL,
    tipo TEXT NOT NULL,
    status TEXT NOT NULL, 
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

-- Criacao do tipo enum para categoria
CREATE TYPE CATEGORIA_ENUM AS ENUM ('lanche', 'bebida', 'acompanhamento', 'sobremesa');

-- Criacao da tabela de produto
CREATE TABLE IF NOT EXISTS produto (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    foto TEXT NOT NULL,
    descricao TEXT NOT NULL, 
    categoria CATEGORIA_ENUM NOT NULL,
    preco FLOAT NOT NULL,
    ingredientes TEXT[] NOT NULL,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

-- Criacao da tabela de cliente
CREATE TABLE IF NOT EXISTS cliente (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    email TEXT NOT NULL,
    cpf TEXT NOT NULL,
    data_criacao TIMESTAMP ,
    data_atualizacao TIMESTAMP
);

-- Criacao da tabela de pedido
CREATE TABLE IF NOT EXISTS pedido (
    id SERIAL PRIMARY KEY,
    cliente INT NOT NULL,
    lanche INT,
    acompanhamento INT,
    bebida INT,
    pagamento TEXT,
    status INT NOT NULL,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP,
    CONSTRAINT fk_cliente FOREIGN KEY (cliente) REFERENCES cliente(id),
    CONSTRAINT fk_lanche FOREIGN KEY (lanche) REFERENCES produto(id),
    CONSTRAINT fk_acompanhamento FOREIGN KEY (acompanhamento) REFERENCES produto(id),
    CONSTRAINT fk_bebida FOREIGN KEY (bebida) REFERENCES produto(id)
);
