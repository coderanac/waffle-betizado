# Suites de Testes

### Suítes e Describes

Imagine que você tem o arquivo `diz-ola.js` e deseja testá-lo. Então você cria o `diz-ola.spec.js`. Nele você faz os importes ou requires do que precisará para
testar suas funções. 

Então o próximo passo é chamar o método `describe`. Ele cria um conjunto de testes, esse conjunto é chamado de `suite`. Os describes recebem uma string com o 
nome da suíte de testes e uma função que define as suítes internas e specs.

Exemplo:

~~~
describe('dizOla', () => {
 //aqui estarão as suítes e specs
});
~~~

### Specs e its

Agora dentro do describe vamos chamar o método `it`, que vai ser quem de fato será executado. Ele recebe uma string  que descreve o que se espera quando algo for execurado (`should/when`), essa descrição se chama `spec`, e uma função que recebe o código de teste.

Exemplo: 

~~~
it('Espera que diga olá quando receber um nome', () => {
 //aqui estarão os códigos de teste
};
~~~

### Expectations

O expect é o que você espera que aconteça quando você executar aquele teste. Por exemplo: quando o método `dizOla` recebe um nome, se espera que ele retorne o nome recebido.

~~~
expect(dizOla('Carol')).to.be.a('Carol);
~~~


### Matchers



### Setups e teardown/hooks

Os `setups` são organizações de métodos que serão usados na execução dos testes. Ele recebe `hooks`.
Os hooks são códigos rodados a partir de alguma ação. Eles servem para evitar códigos replicados. Ele pode ser usado para criar métodos que devem ser executados antes de cada teste, ou limpar e reescrever váriaveis das suítes antes de cada execução.

Imagine que o meu arquivo `diz-ola.js` tem uma função chamada `dizOla` que recebe um nome e outra função chamada `dizTchau` que recebe um nome e diz "Tchau {Nome}".  
Para evitar escrever duas variáveis com o nome, eu posso usar o hook para servir as duas e restaurá-la a cada uso para evitar erros. 


#### Tipos de Hooks

~~~
before(() => {roda uma vez antes do bloco});

after(() => {roda uma vez depois do bloco});

beforeEach(() => {roda uma vez antes de CADA bloco});

afterEach(() => {roda uma vez depois de CADA bloco});
~~~

Exemplo: 

~~~
//setup

beforeEach(() => {
 conts nome = 'Carol';
});

afterEach(() => {
 nome.restore();
});

//specs

it('Espera que diga olá quando receber um nome, () => {
  expect(dizOla(nome).to.be.a('Olá, Carol');
});

it('Espera que diga tchau quando receber um nome, () => {
 expect(diztchau(nome).to.be.a('Tchau, Carol');
});
~~~
