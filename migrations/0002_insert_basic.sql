-- Insert para categoria 'lanche'
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Hamburguer', 'hamburguer.jpg', 'Delicioso hamburguer artesanal', 'lanche', 10.99, '{"carne bovina", "queijo cheddar", "alface" }', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert para um Cheeseburguer
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Cheeseburguer', 'cheeseburguer.jpg', 'Irresistível cheeseburguer com queijo derretido', 'lanche', 12.99, '{"carne bovina", "queijo cheddar", "cebola" }', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert para um Lanche de Frango
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Hamburguer de Frango', 'lanche_frango.jpg', 'Delicioso lanche com carne de frango grelhada', 'lanche', 11.99, '{"carne frango", "queijo prato", "alface" }', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert para categoria 'bebida'
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Refrigerante', 'refrigerante.jpg', 'Bebida gelada para acompanhar', 'bebida', 3.99, '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert para categoria 'acompanhamento'
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Batata Frita', 'batata_frita.jpg', 'Batatas crocantes e saborosas', 'acompanhamento', 5.99, '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert para categoria 'sobremesa'
INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao)
VALUES ('Sorvete', 'sorvete.jpg', 'Sorvete cremoso para adoçar o paladar', 'sobremesa', 7.99, '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
