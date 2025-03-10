﻿# tic_tac_toe_rust
Este projeto implementa um simples jogo da velha (tic-tac-toe) em Rust para ser jogado no terminal.

## 📌 Funcionalidades
- Jogo para dois jogadores (X e O) em um tabuleiro 3x3.
- Entrada de jogador via terminal.
- Verificação automática de vitória ou empate.

## 📂 Estrutura do Código

### 1. Função `main`
- Inicializa o tabuleiro vazio.
- Alterna entre os jogadores até que haja um vencedor ou empate.
- Lê a entrada do usuário e valida a jogada.
- Atualiza o tabuleiro com a jogada do jogador.

### 2. Função `print_board(board: &Vec<char>)`
- Exibe o tabuleiro no terminal.

### 3. Função `check_winner(board: &Vec<char>, player: char) -> bool`
- Verifica se um jogador venceu com base nas linhas, colunas e diagonais.

## 🚀 Como Executar

1. Clone este repositório:
   ```bash
   git clone <URL_DO_REPOSITORIO>
   cd <NOME_DO_PROJETO>
   ```
2. Compile e execute o código:
   ```bash
   rustc main.rs -o tic_tac_toe
   ./tic_tac_toe
   ```

## 📝 Exemplo de Uso

```bash
Jogador X, é sua vez. Escolha um número de 1 a 9:
5
Jogador O, é sua vez. Escolha um número de 1 a 9:
3
...
Parabéns! Jogador X venceu!
```

## 📜 Licença

---
📌 Desenvolvido para fins educacionais.
