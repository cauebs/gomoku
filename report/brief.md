- [X] modelo dos estados e do jogo

    - O estado do jogo é representado por uma matriz 15x15 cujos elementos
      indicam se há uma pedra e, caso haja, a qual jogador pertence.

    - A jogo é modelado como uma estrutura que contém o estado atual do
      tabuleiro, os dois jogadores, um elemento que marca de quem é a vez e o
      número total de jogadas.


- [X] definição matemática das funções de utilidade e heurística
    - Heurística:
        - Cenários de vitória iminente:
            - Quádrupla (não necessariamente contígua) com espaço para virar
              uma quíntupla
            - Duas triplas (não necessariamente contíguas) com um espaço
              compartilhado
            - Dois espaços separados apenas por uma permutação de 4 elementos
              <X, X, X, E>, em que X é uma peça do jogador atual
        - Cenários de possível vitória:
            - Qualquer cenário de vitória iminente com uma das peças do jogador
              substituída por um espaço

        - Função: {
            contém vitória iminente => inf,
            senão => Cp * Qp + sum(Ce * Np for each e)
        }
    - Utilidade:
        - Cn * Número de jogadas;

- [ ] estruturas de dados auxiliares (se necessário)
    - Árvore cujos nós representam os estados do jogo com seu alfa, beta, valor
      da heurística, e seus filhos.

- [ ] otimizações planejadas para o algoritmo (quanto a podas, como lidar com o
  espaço de estados, etc.)
    - A princípio, minimax com podas alfa-beta como visto em aula;
    - Também é planejado aproveitar a árvore para evitar recálculos;
    - Apenas calcular heurística para os nós em que `heuristic_value` é `None`.
    - (Memória) Ignorar jogadas não mais alcançáveis (descendo a árvore);

Quanto à definição matemática da utilidade e heurística, será avaliado um
estudo das possibilidades que o grupo pretende adotar e o resultado do estudo
em uma função matemática.
