# Components

Componentes são como caixinhas onde podemos guardar pequenos pedaços de código para podermos reutilizá-los.
Vamos construir uma aplicação de 5 páginas e todas elas tem conteúdos diferentes, mas o header e o footer são os mesmos. Antigamente tinhamos que reescreve-lo no
html 5 vezes (1 por arquivo html). Dado esse problema, já consegue identificar a vantagem dos componentes? Podemos escrever o header e o footer apenas 1 vez e 
depois chamá-los em seus respectivos lugares.
Vários componentes podem formar um software completo porque se interligam por meio de uma interface, essa comunicação entre eles é chamada de composição.
Imagine uma hot page que é formada por um componente que tem o header, um com um formulário e um com o footer: temos uma composição de componentes formando um site
completo.

## [Components no Angular](https://angular.io/guide/architecture-components)

- Todo projeto Angular tem pelo menos um component
- Eles são criados por meio de Classes
- São vinculados a um template
- Controla um pedaço de visualização que é o menor agrupamento de elementos que podem ser criados e destruídos juntos dado uma diretiva
- Deve estar na ngModules para poder ser usado em outras partes do projeto
- Deve acompanhar o decorador `@Component` na sua declaração para receber diretivas
- Podemos definir diretivas para ele como:
  - Ciclo de vida
  - `selector` (css)
  - `templateUrl` (html)
  - `providers` (dependências)
- A visualização é por hierarquia
- Tem a comunicação de dados com o template bidirecional
- São dinâmicos
- São organizados em uma árvore de componentes dependentes

## [Components no Vue](https://br.vuejs.org/v2/guide/components.html)

- Podem ser definidos com `Vue.component()` para serem globais na aplicação ou em um arquivo `.vue` com template, script e style
- São instancias reutilizáveis
- Aceitam:
  - Ciclo de vida
  - `data`
  - `computed`
  - `watch`
  - `methods`
- Podem ser reutilizados infinitas vezes
- Você pode usar o mesmo componente mais de uma vez e por regras para afetar só o component que recebeu uma interação
- São organizados em uma árvore de componentes aninhados
- Você pode criá-lo em qualquer lugar da sua aplicação
- Recebe quantas props você quiser
- Só aceita um elemento raiz
- Podem ser dinâmicos
- Podem ouvir os eventos dos filhos e também alterá-los


## [Components no React](https://pt-br.reactjs.org/docs/components-and-props.html)

- São funções Javascripts
- Recebem props
- Retornam elementos React
- Podem ser declarados por meio de Classes ou Funções
- Podem ser dinâmicos
- Seu comportamento pode ser definido por props
- Pode ser composto por outros components

# Resumo

Web Components resolvem o problema de reutilização de código e os Frameworks e Lib o usam de uma forma que vai além de código estático. Agora além de você poder
usar o Header nas 5 páginas, você pode mudar o tipo de opção a ser exibida em cada menu do header dado a página que o usuário esteja.
Cada ferramenta tem sua maneira de declarar, organizar e usar os components, os filhos, montar sua árvore, etc. Mas entendendo o conceito e o que resolve fica
muito mais fácil de usá-lo e aproveitar o melhor do recurso.
