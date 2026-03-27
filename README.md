# ⚽ Head Soccer — Jeu en Rust

Un jeu de **Head Soccer** en 2D développé en Rust avec le framework [Macroquad](https://macroquad.rs/).

## 🎮 Contrôles

### Joueur 1
| Touche | Action |
|--------|--------|
| ← → | Se déplacer |
| ↑ | Sauter (double saut possible) |
| Espace | Tirer |

### Joueur 2
| Touche | Action |
|--------|--------|
| Q / D | Se déplacer |
| Z | Sauter (double saut possible) |
| A | Tirer |

> **Astuce** : Appuyez sur **Y** pour afficher les hitboxes de debug.

## 🏗️ Architecture du projet

```
src/
├── main.rs              # Point d'entrée, boucle de jeu
├── input.rs             # Gestion des entrées clavier
├── render.rs            # Affichage (joueurs, ballon, stade, debug)
├── ia.rs                # Intelligence artificielle
├── models/
│   ├── joueur.rs        # Structure du joueur et hitboxes
│   └── ballon.rs        # Structure du ballon
├── physics/
│   ├── player_physics.rs # Gravité et physique du joueur
│   ├── ball_physics.rs   # Gravité, rebonds et limites du ballon
│   └── collision.rs      # Collisions joueur-ballon et joueur-joueur
└── assets/
    ├── joueur/          # Textures (tête, pied)
    ├── ballon/          # Texture du ballon
    └── stade/           # Texture du stade
```

## 🚀 Lancer le jeu

### Prérequis

- [Rust](https://rustup.rs/) (édition 2021+)

### Exécution

```bash
cargo run
```

## 📦 Dépendances

- **macroquad** `0.4.14` — Moteur de jeu 2D léger et multiplateforme