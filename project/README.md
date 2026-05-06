Bonjour 

voici le projet BBS, BattelBot Script

C'est un langage de programmation fait en rust dont l'objectif premier et de securiser l'utilisations de bots.

Le projet de passe une simple implementation logique. l'objectif et de finir une suite de tools permetant de manipuler, load, executer et limiter les perfs et ressources.

## BBS

BBS est donc un langage interpretté dans un premier temps, compiler dans un second

Il doit:
- etre similaire a du C
    - if
    - else
    - while
    - for
    - switch (dans un second temps)
- avoir les structs
- avoir les classes (dans un second temps)
- avoir les await / async pour permettre le multi thread (est-ce une bonne chose) ?
    - TODO il faut reflechir a await ou une fonction thread
- permettre d'avoir des lambda
- permettre de passer des fonctions en parametre.

## Interpretteur BBS

L'interpreteur doit permettre de prendre un fichier .bbs, le verifier, l'optimiser et le run

Ce Que battelBot veux:
- elle veut pouvoir charger un fichier
- avoir les fonctions charger accessibles,
- permettre de definir ses fonctions a elles

- elle veut pourvoir limiter les fonctionnalitees d'un scritp:
    - limitation memoire
    - limitation de fonctionnalite (thread, objets, les libs utilises)

- 

## Decomposition

il faut une lib, elle contiendra toutes etapes de la compilation,

# Reflexion

Ce que je pense etre bien:

Je veux faire en sorte que dans un programme rust, on puisse charger un fichier rust et ensuite qu'on puisse executer ses fonctions. 


## Truc cool

une fois une lib charger, on peut appeler une fonction qui verifie les fonctions dispo et leurs leurs contents.
























