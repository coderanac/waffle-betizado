# Tipos de testes

## Testes unitários

Teste simples e automatizado para provar o comportamento de um método.

- Precisa de mocks para serviços externos
- São independetes
- Verifica se o método funciona como deveria

Exemplo:

Imagine uma função que deve retornar `"Olá, {Nome}!"` e eu quero testar se tenho esse retorno quando chamo ela, então eu posso chama-lá passando um nome fake
e ver se o resultado obtido é o mesmo que eu esperava. Exemplo:

~~~
olaNome('Carol') // return "Olá, Carol!"
~~~  

## Testes de integração

Teste para validar se os serviços estão funcionando em conjunto.

- Não usa mocks
- Usa o retorno reais dos serviços para validar

Exemplo:

Agora imagine que eu tenho uma função chamada `printaNome` que quando um usuário escreve seu nome no popup, chama a função `olaNome` e printa na tela o retorno da `olaNome`. Então eu vou testar se a `printaNome` pega o nome certo para passar para a `olaNome`, se o retorno da `olaNome` é o esperado e por fim se estou conseguindo printar na tela. Eu testei o componente funcionando como um todo com valores e retornos reais.

## Testes e2e

Teste para analizar a parte externa da aplicação, a usabilidade.

- Valida o fluxo de funcionamento da aplicação

## Diferença entre unitário, integrado e 2e2

<p align="center">
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/unit.gif" />
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/integrado.gif" />
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/e2e.gif" />
</p>

O unitário moca os serviços, ele testa apenas a função como uma unidade. Já o integrado testa se o método ou componente está funcionando bem em conjunto com suas dependências. E o e2e já traz o teste de usabilidade, da navegação propriamente dita. 

Exemplo:

Agora quero testar se quando eu abro meu site aparece um popup perguntando meu nome e se quando eu escrevo meu nome e envio, recebo um olá com meu nome na tela. Isso é um teste e2e.

## Diferenças entre BDD e TDD 

<p align="center">
  <img width="600px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/tdd-vs-bdd.png" />
</p>

O TDD (Test Driven Development) é uma metodologia de desenvolviment de softwares que visa escrever primeiro testes e, posteriormente, funções que passem nesses testes. O BDD (Behavior Driven Development) é uma técnica de desenvolvimento ágil que pratica apresentar ao desenvolvedor o comportamento esperado de uma feature x e deixar que ele escolha a melhor forma de implementa-lá, e assim como no TDD primeiro se escrevem os testes e posteiormente códigos. A diferença entre os dois é que no TDD a função é definida, no BDD o comportamento é definido, mas a escolha da forma de execução é de responsabilidade do Dev. 

### TDD - red-green-refactor 

- Você cria um teste para uma função
- Ele falha porque você está testando algo que não funciona
- Então você escreve um teste "burrinho" só para passar no teste
- E agora é hora de refatorar: melhorar o código e transformá-lo num código muito inteligente

Vantagens: um design de código muito mais clean e livre de redundâncias e um código muito mais escalável e mutável.

### BDD give-when-then

- Estado inicial, ações, entregas esperadas
- Envolve todos os responsáveis pelo produto
- Deve ter requisitos legíveis
- Os requisitos devem ser convertidos em testes executáveis
- Usa mocks para os testes para os recursos que ainda não foram escritos

Dan, o criador do BDD, diz que ela é uma evolução do TDD. Para ele o desenvolvimento deveria ir muito além das pessoas técnicas e juntar todas as pessoas envolvidas no produto.

## Ainda na onda dos DD temos o DDD 

O Domain-Driven Design ficou por último porque ele é mais uma filosofia/conceito. Ele diz que o "idioma" do seu código deve ser o mesmo do modelo de negócio.
Exemplo: se o produto é um aplicativo de convênio as funções devem se chamar marcaConsulta, formularioDoPreExame...

## Teste funcional ou não funcional

### Testes funcionais ou caixa preta

Entra nesse conceito todos os que testam comportabilidade (entradas e saídas)

- Testes unitários
- Testes de integração
- Testes de aceitação
- Testes de sistemas

### Testes não funcionais

O conceito de testes não funcionais se refere a todos os testes que não estão relacionados a comportamentos funcionais. Alguns exemplos do que entra como teste não funcional:

- Desempenho
- Performance
- Se a UI é intuitiva e fácil de aprender

## :tada: E PRA FINALIZAR :tada:

Usuário não é tester 

<p align="center">
  <img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/testado-no-ar.gif" />
</p>
