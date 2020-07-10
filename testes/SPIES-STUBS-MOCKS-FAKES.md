## Spies, stubs, Mocks e fakes

### Spies

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/spies.gif" />
</p>

Eles espionam se a execução esperada ocorreu, ele verifica se o método foi chamado, como foi chamado, os argumentos que recebeu e retornou. Usamos ele para 
testar callbacks e como métodos são usados dentro do sistema. Por exemplo: testar console.log.

Um exemplo para ver se a função `dizOla` foi chamada uma vez.

~~~
const callback = sinon.spy();

dizOla("Carol", callback);

assertTrue(callback.calledOnce);
~~~

### Stubs 

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/stubs.gif" />
</p>

Os stubs são bem parecidos com os spies, mas além de vigiar o método, usando eles, é possível substituir o comportamento dele. Podemos usar ele para forçar um
caso de exceção, para pular uma parte da função que não seja necessária para o teste ou para simplificar o teste assíncrono.

Imagine que a função `dizOla` recebe um objeto com nomes e eu quero substituir o valor do primeiro nome para ver se meu método está funcionando como eu espero.

~~~
sinon.stub(names, 'name1').get(() => 'Carol');

names.name1 // return 'Carol'
~~~

### Mocks

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/mocks.gif" />
</p>

Os mocks servem para fakear tanto os valores de entrada e saída, alterar o comportamento do método e pré-programar as respostas e expectations. Ou seja, com ele
conseguimos modificar tudo do método. Usamos ele quando precisamos de um Stub, mas também precisamos verificar vários comportamentos do nosso método.

Ainda na função `dizOla`, imagine que além de montar a frase ela salva o nome no Banco de Dados. E eu quero testar se ela está salvando uma vez no banco de 
dados, mas não quero salvar meu teste no BD, então vou mocar ele.

É importante sempre no fim do mock restaurar os valores das variáveis mocadas.

~~~
expectedName = { name: 'Carol' };

const bd = sinon.mock(Database);

bd.expects('save').once().withArgs(expectedName);

dizOla(expectedName, () => {});

bd.verify();
bd.restore();
~~~
