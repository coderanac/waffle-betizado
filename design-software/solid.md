# SOLID

SOLID são 5 princípios da Programação Orientada a Objetos ([POO](https://pt.wikipedia.org/wiki/Programa%C3%A7%C3%A3o_orientada_a_objetos)).

### S (SRP - Single Responsability principle)

Esse é o princípio da responsabilidade única, ou seja, uma classe só deve ter uma responsabilidade e um único motivo para existir. Por exemplo:
Se eu tenho um banco e nele eu guardo quem é meu cliente e qual é o cartão dele, posso ter uma classe que cria esse cliente e uma para o cartão, mas se apenas uma classe
é responsável por esses dois trabalhos está errado segundo o SOLID, porque deixamos de ter um código limpo.

### O (OCP - Open-close principle)

Esse princípio diz que objetos e entidades devem estar abertos à extensões e fechados para alterações. Que sempre que tivermos uma nova coisa a ser adicionada deveremos
extender e não modificar o código fonte. Isso é para evitar Bugs em um código que já está funcionando.

*** em alguns casos podemos substituir o método por uma interface.

### L (LSP - Liskov Substitution Principle)

"As classes derivadas devem ser substituíveis por suas classes bases"

O LSP nos alerta sobre a utilização de heranças e está diretamente ligado com o OCP.

### I (ISP - Interface Segregation Principle)

Princípio da segregação da interface diz que uma classe não deve ter interfaces que não vai usar. É melhor ela ter uma interface específica do que uma genérica que não
usará toda.

### D (DIP - Dependency Inversion Principle)

O princípio da inversão de dependência, diz que a aplicação deve depender de abstrações e não de implementações. Reduzir aclopamento de classe.
