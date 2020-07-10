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
