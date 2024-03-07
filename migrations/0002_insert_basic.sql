-- Inserts para `cliente`
-- Insert de 'Fulano da Silva'
INSERT INTO cliente (nome, email, cpf, data_criacao, data_atualizacao)
VALUES (
        'Fulano da Silva',
        'fulano.silva@exemplo.com',
        '123.456.789-09',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Inserts para `produto`
-- Insert para categoria 'Lanche'
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Hamburguer',
        'hamburguer.jpg',
        'Delicioso hamburguer artesanal',
        'Lanche',
        10.99,
        '{"carne bovina", "queijo cheddar", "alface" }',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Insert para um Cheeseburguer
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Cheeseburguer',
        'cheeseburguer.jpg',
        'Irresistível cheeseburguer com queijo derretido',
        'Lanche',
        12.99,
        '{"carne bovina", "queijo cheddar", "cebola" }',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Insert para um Lanche de Frango
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Hamburguer de Frango',
        'lanche_frango.jpg',
        'Delicioso lanche com carne de frango grelhada',
        'Lanche',
        11.99,
        '{"carne frango", "queijo prato", "alface" }',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Insert para categoria 'Bebida'
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Refrigerante',
        'refrigerante.jpg',
        'Bebida gelada para acompanhar',
        'Bebida',
        3.99,
        '{}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Insert para categoria 'Acompanhamento'
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Batata Frita',
        'batata_frita.jpg',
        'Batatas crocantes e saborosas',
        'Acompanhamento',
        5.99,
        '{}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Insert para categoria 'Sobremesa'
INSERT INTO produto (
        nome,
        foto,
        descricao,
        categoria,
        preco,
        ingredientes,
        data_criacao,
        data_atualizacao
    )
VALUES (
        'Sorvete',
        'sorvete.jpg',
        'Sorvete cremoso para adoçar o paladar',
        'Sobremesa',
        7.99,
        '{}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

-- Inserts para `pedido`
-- Insert combo completo com 'cliente' identificado
INSERT INTO pedido (
        cliente_id,
        lanche_id,
        acompanhamento_id,
        bebida_id,
        pagamento,
        status,
        data_criacao,
        data_atualizacao
    )
VALUES (
        1,
        1,
        5,
        4,
        'Mercado Pago',
        'Pendente',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );
-- Insert combo incompleto com 'cliente' não identificado
INSERT INTO pedido (
        cliente_id,
        lanche_id,
        acompanhamento_id,
        bebida_id,
        pagamento,
        status,
        data_criacao,
        data_atualizacao
    )
VALUES (
        NULL,
        1,
        NULL,
        NULL,
        'Mercado Pago',
        'Pronto',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

INSERT INTO pagamento (
        id_pedido,
        estado,
        valor,
        metodo,
        referencia,
        data_criacao
    )
VALUES (
        1,
        'pendente',
        100.00,
        'Mercado Pago',
        'abc1234567890',
        CURRENT_TIMESTAMP
    );
