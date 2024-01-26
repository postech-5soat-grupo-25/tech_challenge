CREATE TABLE IF NOT EXISTS usuario (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(255),
    email VARCHAR(255),
    cpf VARCHAR(255),
    senha VARCHAR(255),
    tipo INT,
    status INT, 
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

CREATE TABLE IF NOT EXISTS produto (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(255),
    foto VARCHAR(255),
    descricao VARCHAR(255), 
    categoria INT,
    preco MONEY,
    ingredientes VARCHAR(255)[],
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

CREATE TABLE IF NOT EXISTS cliente (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(255),
    email VARCHAR(255),
    cpf VARCHAR(255),
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pedido (
    id SERIAL PRIMARY KEY,
    cliente INT,
    lanche INT,
    acompanhamento INT,
    bebida INT,
    pagamento VARCHAR(255),
    status INT,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP,
    CONSTRAINT fk_cliente FOREIGN KEY (cliente) REFERENCES cliente(id),
    CONSTRAINT fk_lanche FOREIGN KEY (lanche) REFERENCES produto(id),
    CONSTRAINT fk_acompanhamento FOREIGN KEY (acompanhamento) REFERENCES produto(id),
    CONSTRAINT fk_bebida FOREIGN KEY (bebida) REFERENCES produto(id)
);
