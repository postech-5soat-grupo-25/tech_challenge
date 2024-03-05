# Tech Challenge | PosTech 5SOAT • Grupo 25

## Sobre o Projeto

Este projeto é desenvolvido como parte do Tech Challenge, um requisito para a conclusão do curso de Pós-Graduação em Software Architecture da FIAP. O desafio proposto visa solucionar problemas reais enfrentados por uma lanchonete em expansão, através do desenvolvimento de um sistema de autoatendimento eficiente.

### Documentação

Para mais informações sobre o projeto, incluindo detalhes técnicos e guias de utilização, acesse nossa *documentação completa* em: [postech-5soat-grupo-25.github.io](https://postech-5soat-grupo-25.github.io/)

### Equipe

| Membro                                                                        | RM       |
|-------------------------------------------------------------------------------|----------|
| [Alan Marques Molina](https://www.linkedin.com/in/alanmmolina/)               | `353062` |
| [Albert Dias Moreira](https://www.linkedin.com/in/albert-moreira-62b9272b/)   | `352569` |
| [Bruno Mafra Pelence](https://www.linkedin.com/in/bruno-mafra-pelence/)       | `352939` |
| [Lucas Felipe Rebello](https://www.linkedin.com/in/lucas-rebello-b01849112/)  | `352982` |
| [Matheus Bachiste Lopes](https://www.linkedin.com/in/matheus-bachiste-lopes/) | `352783` |

### Contexto

Com a expansão de uma lanchonete e a ausência de um sistema de controle de pedidos, o atendimento pode se tornar caótico, afetando a satisfação dos clientes. Nosso sistema busca otimizar o processo de pedidos, pagamento, acompanhamento e entrega, garantindo uma experiência fluida e satisfatória para os clientes e uma gestão eficaz para o estabelecimento.

#### Funcionalidades

- **Pedido**: Interface de seleção de produtos com opções de montagem de combos.
- **Pagamento**: Integração com Mercado Pago para pagamento via QRCode.
- **Acompanhamento**: Monitoramento em tempo real do progresso e status do pedido.
- **Entrega**: Notificação ao cliente e atualização do status após a retirada.
- **Acesso Administrativo**:
    - *Gerenciamento de clientes*: Administração do cadastro de clientes.
    - *Gerenciamento de produtos*: Administração dos produtos disponíveis.
    - *Acompanhamento de pedidos*: Visualização dos pedidos em andamento.

## Como Executar o Projeto

> **Requisitos**
> - [Rust](https://www.rust-lang.org/tools/install)
> - [Docker](https://docs.docker.com/get-docker/)
> - [Docker Compose](https://docs.docker.com/compose/install/)
> - [Rust Analyzer](https://rust-analyzer.github.io/manual.html#installation)


- Clone o repositório

```bash
git clone git@github.com:postech-5soat-grupo-25/tech_challenge.git
```

- Entre na pasta do projeto

```bash
cd tech_challenge
```

- Execute o projeto

Se for a primeira vez que você executa a aplicação, execute o seguinte comando para rodar os contêiners e executar as migrações (criar as tabelas e inserir os dados básicos):

```bash
make build
```

Se você já tiver tudo configurado, poderá executar os contêiners com o seguinte:

```bash
make run
```

Se você quiser parar todos os contêineres relacionados, use o seguinte:

```bash
make down
```
