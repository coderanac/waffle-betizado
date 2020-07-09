# Tipos de testes

## Testes unitários 

Teste simples e automatizado para provar o comportamento de um método.

- Precisa de mocks para serviços externos
- São independetes
- Verifica se o método funciona como deveria

## Testes de integração

Teste para validar se os serviços estão funcionando em conjunto.

- Não usa mocks
- Usa o retorno reais dos serviços para validar

## Testes e2e

Teste para analizar a parte externa da aplicação, a usabilidade.

- Valida o fluxo de funcionamento da aplicação

## Diferença entre unitário, integrado e 2e2

O unitário moca os serviços, ele testa apenas a função como uma unidade. Já o integrado testa se o método ou componente está funcionando bem em conjunto com suas dependências. E o e2e já traz o teste de usabilidade, da navegação propriamente dita. 

## Diferenças entre BDD, TDD e ODD

## Teste funcional ou não funcional
