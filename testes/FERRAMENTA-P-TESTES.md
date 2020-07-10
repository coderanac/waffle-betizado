# Ferramenta para testes 
 
### Mocha

Com o Mocha consigo fazer testes `assíncronos`. Ele é uma estrutura de códigos Javascripts que pode ser rodado no `node.js` e nos `browsers`. Ele tem sua execução
sincrona.

### Chai

O chai é uma biblioteca para asserts e pode ser usada em qualquer estrutura de testes Javascript. Ele tem uma interface legível e clara.

Exemplo de Mocha usando Chai:

~~~
describe('dizOla', () => {
  it('Should him to say hello when he gets a name', () => {
    expect(dizOla('Carol')).to.be.a('Carol');
  });
});
~~~

### SinonJs

O SinonJS é uma biblioteca de spies, stubs e mocks para testes unitários. E funciona com qualquer ferramenta de testes unitários.

Exemplo: 

~~~
expectedName = { name: 'Carol' };

const bd = sinon.mock(Database);

bd.expects('save').once().withArgs(expectedName);

dizOla(expectedName, () => {});

bd.verify();
bd.restore();
~~~

### Jest

Jest é framework para testes em Javascript. Ao contrário do Mocha, ele já tem seus próprios matchers, asserts, setups, mocks e outros recursos necessários para
desenvolver os testes completos. Já o Mocha precisa de recursos externos como Chai, Sinon e Nock.

Exemplo:

~~~
test('dizOla', () => {
  expect(dizOla('Carol')).toBe('Olá, Carol');
});
~~~

### Jasmine

Jasmine é um framework de testes que segue o conceito de testes BDD (orientado a comportamento) para testar Javascript. Ela também tem uma sintaxe bem 
limpa e objetiva.

Exemplo:

~~~
describe('diz ola', () => { 
  it('Should him to say hello when he gets a name', () => { 
    expect(dizOla()).toEqual('Olá!'); 
  });
});
~~~

### NockJs

O NockJS é uma lib para NodeJS que moca requisições HTTP. Ele é usado para interceptar uma requisição e simular um retorno.

Exemplo:

~~~
import axios from 'axios';
import nock from 'nock';
import test from 'ava';

test('can fetch test response', async t => {
  // Set up the mock request.
  const scope = nock('http://localhost')
    .get('/test')
    .reply(200, 'test response')

  await axios.get('http://localhost/test')

  scope.done()
})
~~~

### Selenium

O Selenium é uma ferramenta para simular o navegador, com ele você consegue executar qualquer tipo de teste que dependa de um browser para ser
executado, como por exemplo: os testes end-to-end. Além de ser extremamente completo, ele também suporta diversas linguagens como C#, Java, Python, Perl. Além de
possuir sua própria IDE.

### Protactor

Agora entrando em testes end-to-end (e2e). O Protactor serve para fazer testes de UI, ele simula a navegação de um usuário e metrifica e registra os testes.
Ele é usado para fazer testes e2e no framework Angular.

### Cypress

Assim como o Protactor, o Cypress é usado para testes end-to-end. Suas principais diferenças é que o Cypress não precisa do selenium, toda a arquitetura usada é
própria visando melhorar a performance dos testes, e que o Cypress roda em todos os frameworks front end.

### Cucumber

Cucumber é um software para desenvolvimento de testes. Tem sua prórpia linguagem e fornece testes para diversas linguagens de programação: javascript, java, ruby,
php, lua...
Ele é focado em testes BDD.

