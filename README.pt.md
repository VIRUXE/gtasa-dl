# GTASA Downloader

![image](https://github.com/VIRUXE/gtasa-dl/assets/1616657/30d37642-ada4-49f7-b943-eb4dfdd084bb)

## Introdução

Este downloader tem como objetivo tornar fácil para qualquer pessoa ter uma cópia portátil do jogo, junto com *SA-MP*, um *ASI loader* (para seus scripts variados) e *SAMPCAC* (SA-MP Client Anti-Cheat, que está descontinuado, mas ainda funciona bem e é melhor que nada...).

## Por que

1. Não há muitos lugares para baixar o jogo atualmente. E quando há, você nunca sabe o que vai obter.
2. O arquivo que ele baixa é realmente pequeno (~500MB), comparado aos ~4.7GB quando descompactado. Então, não me importo em hospedá-lo.
3. Está hospedado em um dos meus VPSs que nunca vai sair do ar (custa ~50USD por ano). Seu único propósito é servir arquivos. Está limitado a downloads de 1Mb/s e 1 conexão simultânea.
4. Juntar todos os arquivos corretos/instalar o software correto é sempre um incômodo.
5. O script ASI *SAMPCAC* é marcado como malware (não é), então uma exceção precisa ser adicionada ao *Windows Defender* para evitar que ele seja removido do diretório do jogo. (É por isso que precisa ser executado como *Administrador*.)
6. Considerei este um exercício simples o suficiente para começar com a linguagem Rust.

## ASI Loader

O loader fornecido é [meu fork](https://github.com/viruxe/sa-asi-loader) do [*ASI Loader* de Carlos Menezes](https://github.com/carlos-menezes/sa-asi-loader).

## SAMPCAC

Todos os anti-cheats do lado do cliente foram burlados em algum momento, e este não foi exceção. Mas tudo bem, está incluído, pois alguns servidores ainda o usam, já que nem todos conseguem burlá-lo.

### Problemas com Anti-Cheat

Pode não funcionar junto com outros scripts ASI. (Não funcionou para mim ao tentar rodar um ASI de Modo Janela.) Remova-o se não precisar ou renomeie a extensão se achar que precisará dele depois.

## Problemas Conhecidos

- Como estou usando *WinGet* para instalar *7Zip* (para descompactar o arquivo do jogo) e *DirectX* (Sim, instalações novas do *Windows* não vêm com *DirectX* pré-instalado.), você precisa aceitar os termos do *msstore* (*Microsoft* Store - essa é a fonte para este pacote WinGet) antes de baixar qualquer coisa. Então, se não funcionar na primeira vez, abra o prompt de comando e execute `winget install --id=7zip.7zip -e` para aceitar esses termos e instalar o *7Zip*.

## Como funciona

1. Você abre o executável como **Administrador**.
2. Baixa um arquivo 7Zip ("gtasa.7z") para o mesmo diretório do executável.
3. Pergunta a você por um nome de diretório (pressione enter para usar o padrão - "GTA San Andreas") para onde descompactará o arquivo do jogo, igualmente, no mesmo diretório do executável.
4. Descompacta o arquivo no diretório especificado.
5. Adiciona uma exceção/exclusão ao *Windows Defender* para evitar que o script ASI do *SAMPCAC* seja removido.
6. Baixa "sampcac_client.asi".

O arquivo do jogo permanecerá para que possa ser reutilizado na próxima vez que quiser uma instalação nova ou se quiser armazená-lo para si mesmo.