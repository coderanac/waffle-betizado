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

