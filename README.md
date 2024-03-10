# Tech Challenge | PosTech 5SOAT • Grupo 25

## Sobre o Projeto

Este projeto é desenvolvido como parte do Tech Challenge, um requisito para a conclusão do curso de Pós-Graduação em Software Architecture da FIAP. O desafio proposto visa solucionar problemas reais enfrentados por uma lanchonete em expansão, através do desenvolvimento de um sistema de autoatendimento eficiente.

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

---

### Equipe

| Membro                                                                        | RM       |
|-------------------------------------------------------------------------------|----------|
| [Alan Marques Molina](https://www.linkedin.com/in/alanmmolina/)               | `353062` |
| [Albert Dias Moreira](https://www.linkedin.com/in/albert-moreira-62b9272b/)   | `352569` |
| [Bruno Mafra Pelence](https://www.linkedin.com/in/bruno-mafra-pelence/)       | `352939` |
| [Lucas Felipe Rebello](https://www.linkedin.com/in/lucas-rebello-b01849112/)  | `352982` |
| [Matheus Bachiste Lopes](https://www.linkedin.com/in/matheus-bachiste-lopes/) | `352783` |

---

### Documentação

Para mais informações sobre o projeto, incluindo detalhes técnicos e guias de utilização, acesse nossa *documentação completa* em: [postech-5soat-grupo-25.github.io](https://postech-5soat-grupo-25.github.io/). O sumário abaixo irá direcioná-lo para as documentações de cada etapa do desenvolvimento do projeto.

#### Fase 1

##### Entregável 1

- [Domain-Driven Design](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#domain-driven-design)
  - [Glossário da Linguagem Ubíqua](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#glossário-da-linguagem-ubíqua)
  - [Técnias de Modelagem de Domínio](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#técnias-de-modelagem-de-domínio)
    - [Domain Storytelling](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#domain-storytelling)
    - [Event Storming](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#event-storming)
  - [Entidades e Objetos de Valor](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#entidades-e-objetos-de-valor)
    - [Entidades](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#entidades)
    - [Objetos de Valor](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_1/#objetos-de-valor)

##### Entregável 2

- [Arquitetura Hegaxonal](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_2/#arquitetura-hegaxonal)
  - [Estrutura de Diretórios](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_2/#estrutura-de-diretórios)
  - [Frameworks e Bibliotecas](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_2/#frameworks-e-bibliotecas)
    - [Rocket](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_2/#rocket)
    - [Tokio Postgres](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_2/#tokio-postgres)

##### Entregável 3

- [Dockerfile](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#dockerfile)
  - [Estágio de Construção](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#estágio-de-construção)
  - [Estágio de Produção](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#estágio-de-produção)
- [Docker Compose](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#docker-compose)
  - [Serviço de Aplicação](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#serviço-de-aplicação)
  - [Serviço de Banco de Dados](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#serviço-de-banco-de-dados)
- [Tutorial para Execução da Aplicação com Docker](https://postech-5soat-grupo-25.github.io/fase_1/entregavel_3/#tutorial-para-execução-da-aplicação-com-docker)


#### Fase 2

##### Entregável 1

- [Clean Architecture e Clean Code](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#clean-architecture-e-clean-code)
  - [Ports \& Adapters x Clean Architecture](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#ports--adapters-x-clean-architecture)
  - [Conversão para Clean Architecture](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#conversão-para-clean-architecture)
    - [Camadas da Arquitetura](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#camadas-da-arquitetura)
      - [Entidades](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#entidades)
      - [Casos de Uso](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#casos-de-uso)
      - [Gateways](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#gateways)
      - [Controllers](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_1/#controllers)

##### Entregável 2

- [Infraestrutura Kubernetes](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#infraestrutura-kubernetes)
  - [Construção dos Manifestos](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#construção-dos-manifestos)
    - [app-deployment](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#app-deployment)
    - [app-hpa](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#app-hpa)
    - [app-metrics](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#app-metrics)
    - [app-svc](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#app-svc)
    - [configmap](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#configmap)
    - [db-postgres](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#db-postgres)
    - [db-svc](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#db-svc)
    - [mock-pagamentos-pod](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#mock-pagamentos-pod)
    - [mock-pagamentos-svc](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_2/#mock-pagamentos-svc)
  
##### Entregável 3

- [Arquitetura Kubernetes](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#arquitetura-kubernetes)
- [Tutorial para Execução da Aplicação com Kubernetes](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#tutorial-para-execução-da-aplicação-com-kubernetes)
- [Tutorial para Interação com as APIs](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#tutorial-para-interação-com-as-apis)
  - [Autenticação](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#autenticação)
  - [Usuários](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#usuários)
  - [Clientes](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#clientes)
  - [Produtos](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#produtos)
  - [Pedidos](https://postech-5soat-grupo-25.github.io/fase_2/entregavel_3/#pedidos)

---
