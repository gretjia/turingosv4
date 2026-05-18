# Attempt 1 — Qwen3-Coder generated game is non-functional

## Bug
Lines 277-296 in the generated index.html (the keydown handler):

```js
document.addEventListener('keydown', event => {
    if (gameOver) {
        if (event.code === 'Space') {
            resetGame();
            instructions.style.display = 'none';
            update();
        }
        return;
    }

    if (event.code === 'Space') {
        if (player.matrix === null) {  // <-- never true on first start
            player.matrix = createPiece();
            player.pos.y = 0;
            player.pos.x = Math.floor(COLS / 2) - 1;
            instructions.style.display = 'none';
            update();
        }
        return;
    }
    ...
});

// 初始化游戏
init();
```

`init()` calls `resetGame()` (line 114), which sets `player.matrix = createPiece()` (line 132). So by the time the keydown listener fires for the first Space press, `player.matrix !== null`, so the early-return on line 295 is taken and `update()` is never invoked. The rAF loop never starts. The game is frozen.

## Symptoms observed
- Game renders initial UI: black playfield rectangle, "分数: 0", "最高分: 0", "按空格开始" instructions
- Press Space: nothing happens (no piece appears, no animation)
- Press 30 ArrowDown: nothing happens
- Console: no errors logged

## Mechanical test results for attempt 1
- T1 HTTP 200: PASS
- T2 playfield exists: PASS (canvas#game-board with width=200 height=400 = 10x20 cells)
- T3 score visible: PASS ("分数" and "最高分" rendered)
- T4 sandbox safe: PASS (sandbox="allow-scripts", no allow-same-origin)
- T5 keyboard reactive: FAIL (Space press does not start game)
- T6 rotation works: FAIL (cannot test — no piece visible)
- T7 no console errors: PASS (no errors observed)
- T8 plays through: FAIL (game state never advances)

Score: 5/8 PASS, 3/8 FAIL (the 3 fails are the critical "is it playable" tests)
