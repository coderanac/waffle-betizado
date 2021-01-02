# CSS-IN-JS

## Um resumo da história

O css-in-js foi apresentado pela primeira vez na [React Conf em 2014 por Christopher Chedeau (vjeux)](https://speakerdeck.com/vjeux/react-css-in-js). Ele propunha
a solução para problemas como classes cruzadas, código "morto" que foi pintado mas já não deveria estar renderizado, redução de repetição de código, redução de classes globais,
entre outros problemas na utilização do css puro no React.  
O Angular já lidava muito bem com css encapsulado e escopado para módulos e páginas. O VueJS trouxe com ele também uma solução nativa de CSS escopado nativamente.
Mas até então o React ainda sofria muito com essas questões e soluções como o css bem não bastavam. Porque poderia ter isso em mais de um lugar no meu projeto:

```
.form-button__submit {
  ...
}
```

Então dado o conceito, surgiram bibliotecas que ofertavam o recurso. E agora não precisávamos nos preocupar de ter mais de um ```.menu```, porque desde que estivessem em folhas
separadas, ele se transformaria em algo assim: ```<p class="menu_746732">...</>```. Além de hashs malucos, as libs de css-in-js administravam os estilos conforme a demanda,
se eram necessários eram renderizados, se não, eram retirados do documento final. E o gerenciamento de isolamento e compartilhamento ficou bem mais simples.

## Mas por que CSS com Javascript? 

A ideia de vjeux era manter o desenvolvimento simples e legível para os desenvolvedores. Então adicionar o css para a sintaxe do javascript e transformar tudo em um trabalho
só foi a ideia para resolver o problema de forma menos exaustiva, com uma curva de adaptação menor. Então o css que era assim:

```
<style>
  .button {
    color: red;
  }
</style>
```

Ficou assim: 

```
const Box = style.div(
  {
    color: 'blue',
  }
);
```

## Prós e contras:

### Prós

- Gerenciamento dinâmico de estilo quando nasce e morre
- Estilo escopado sem precisar se preocupar em configurar um rollup ou webpack
- Redução de repetição de código
- Compartilhamento inteligente de variáveis
- Redução de dependências 
- Fácil escalabilidade

Constras
- Nomes finais são hashs estranhos, o que gera um fluxo na hora de localizar a classe original

## Recomendações

Uma Lib para quem está trabalhando com Javascript: [Styled Component](https://styled-components.com/).  
Uma Lib para quem está trabalhando com Typescript: [TypeStyle](https://github.com/typestyle/typestyle).
