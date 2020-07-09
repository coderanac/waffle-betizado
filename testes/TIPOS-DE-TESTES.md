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

## Diferenças entre BDD, TDD e ODD

## Teste funcional ou não funcional
