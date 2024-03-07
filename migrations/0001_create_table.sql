-- Criacao do tipo enum para status do usuário
CREATE TYPE STATUS_USUARIO_ENUM AS ENUM (
    'Ativo',
    'Inativo'
);

-- Criacao do tipo enum para tipo do usuário
CREATE TYPE TIPO_USUARIO_ENUM AS ENUM (
    'Admin',
    'Cozinha'
);

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
CREATE TYPE CATEGORIA_ENUM AS ENUM (
    'Lanche',
    'Bebida',
    'Acompanhamento',
    'Sobremesa'
);

-- Criacao da tabela de produto
CREATE TABLE IF NOT EXISTS produto (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    foto TEXT NOT NULL,
    descricao TEXT NOT NULL,
    categoria CATEGORIA_ENUM NOT NULL,
    preco FLOAT NOT NULL,
    ingredientes TEXT [] NOT NULL,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

-- Criacao da tabela de cliente
CREATE TABLE IF NOT EXISTS cliente (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    email TEXT NOT NULL,
    cpf TEXT NOT NULL,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP
);

-- Criacao do tipo enum para status
CREATE TYPE STATUS_PEDIDO_ENUM AS ENUM (
    'Pendente',
    'Pago',
    'EmPreparacao',
    'Pronto',
    'Finalizado',
    'Cancelado',
    'Invalido'
);

-- Criacao da tabela de pedido
CREATE TABLE IF NOT EXISTS pedido (
    id SERIAL PRIMARY KEY,
    cliente_id INT,
    lanche_id INT,
    acompanhamento_id INT,
    bebida_id INT,
    pagamento TEXT,
    status STATUS_PEDIDO_ENUM NOT NULL,
    data_criacao TIMESTAMP,
    data_atualizacao TIMESTAMP,
    CONSTRAINT fk_cliente FOREIGN KEY (cliente_id) REFERENCES cliente(id),
    CONSTRAINT fk_lanche FOREIGN KEY (lanche_id) REFERENCES produto(id),
    CONSTRAINT fk_acompanhamento FOREIGN KEY (acompanhamento_id) REFERENCES produto(id),
    CONSTRAINT fk_bebida FOREIGN KEY (bebida_id) REFERENCES produto(id)
);

-- Criacao da tabela de pagamento
CREATE TABLE IF NOT EXISTS pagamento (
    id SERIAL PRIMARY KEY,
    id_pedido INT,
    estado  TEXT NOT NULL,
    valor FLOAT NOT NULL,
    metodo TEXT NOT NULL,
    referencia TEXT,
    data_criacao TIMESTAMP
);