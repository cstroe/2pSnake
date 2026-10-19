# 2-Player Tron Snake (Bevy 2D)

A 2-player competitive arcade game built using the **Bevy 2D game framework**, fusing classic Snake mechanics with Tron light-cycle territory battles.

## Overview & Game Rules

- **Arena**: 485 squares wide by 300 squares tall (145,500 cells).
- **Adaptive Resolution**: Cell sizing automatically scales according to the window/monitor resolution (1080p, 1440p, 4K, 16:9 large TVs) while leaving space for the top HUD.
- **Objective**: Collect various fruits scattered across the arena while cutting off and boxing in the opponent.
- **Victory**: The first player to collect the level goal (15 fruits) wins the match! Alternatively, if the 5-minute round timer expires, the player with the highest score wins.
- **Growth**: Each fruit type provides score points and fixed length growth (+2 to +6 segments).
- **Collision & Respawn**: Hitting the arena border, yourself, or the other player's snake results in death. The snake respawns in a safe, empty area of the grid with its starting length while preserving its score.
- **Clock Tick**: Starts at 200 milliseconds (0.2s) per tick on level 1, adjustable dynamically during gameplay via controller bumpers or keyboard.
- **Responsive Head Look**: When a player presses a direction on the gamepad or keyboard, the snake's head and eyes immediately turn to look in that direction, showing the next move before the tick occurs.

---

## Controllers & Input Setup

### Pre-Game Controller Selection Screen
Before the game starts, a dedicated **Controller Selection Room** is displayed:
- Automatically detects connected gamepads (e.g. Xbox-style USB / wireless controllers) and keyboards.
- Each player uses their controller to pick their player slot (**Player 1: Cyan** on the left, **Player 2: Orange** on the right).
- Press **A** on the controller (or **Space** / **Enter** on keyboard) to **READY UP**.
- Once both players are ready, the match automatically begins!

### In-Game Controls

| Action | Gamepad (Xbox-style) | Player 1 Keyboard | Player 2 Keyboard |
|---|---|---|---|
| **Move Up** | D-Pad Up / Left Stick Up | `W` | `Up Arrow` |
| **Move Down** | D-Pad Down / Left Stick Down | `S` | `Down Arrow` |
| **Move Left** | D-Pad Left / Left Stick Left | `A` | `Left Arrow` |
| **Move Right** | D-Pad Right / Left Stick Right | `D` | `Right Arrow` |
| **Speed Up Tick** | Left Bumper (`LB`) | `+` or `]` | `+` or `]` |
| **Slow Down Tick** | Right Bumper (`RB`) | `-` or `[` | `-` or `[` |
| **Ready / Rematch** | `A` Button | `Space` | `Enter` |

---

## Game Flow

1. **Controller Select Screen**: Assign devices and press A to ready up.
2. **3-Second Countdown**:
   - The 485x300 arena is displayed with Player 1 (Cyan) and Player 2 (Orange) stationed on opposite sides.
   - Animated countdown: `3` -> `2` -> `1` -> `START!`.
   - After "START!" disappears (1-second delay), player controls unlock and the game tick begins.
3. **Active Gameplay**:
   - Both snakes advance on each clock tick.
   - Top HUD displays player scores, current snake lengths, round time remaining, and tick speed.
   - Head and eyes immediately face the pressed direction.
   - Reversing 180 degrees into oneself is blocked.
   - Eating fruits triggers growth and replacement fruit spawns.
   - Collisions trigger respawns without pausing the opponent.
4. **Round Over Screen**:
   - Displays the winner, final scores, and rematch button prompt (`A` / `Space` / `Enter`).

---

## Running the Game

To run the game using Cargo:

```bash
cargo run --release
```

Or for debug mode:

```bash
cargo run
```
