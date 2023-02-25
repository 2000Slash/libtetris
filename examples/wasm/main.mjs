import init, {WasmGame} from "./pkg/tetris.js"

init().then(() => {
    let ctx = document.getElementById("canvas").getContext("2d");
    ctx.canvas.width = 250;
    ctx.canvas.height = 500;
    ctx.font = "48px serif";
    let game = WasmGame.new();

    document.addEventListener("keydown", (e) => {
        //wasd controlls
        switch (e.key) {
            case "a":
                game.left();
                break;
            case "s":
                game.down();
                break;
            case "d":
                game.right();
                break;
            case "e":
                game.rotate_right();
                break;
            case "q":
                game.rotate_left();
                break;
            case "Shift":
                game.store();
                break;
            case " ":
                game.hard_drop();
                break;
        }
    }, false);

    let colors = [0, "#00FFFF", "#0000FF", "#FFA500", "#ffff00", "#00ff00", "#800080", "#ff0000"];
    console.log(colors)
    window.setInterval(() => {
        game.tick();
        let bytes = game.draw();
        for (let i = 0; i < bytes.length; i++) {
            // draw the cells
            let x = i % 10;
            let y = Math.floor(i / 10);
            switch (bytes[i]) {
                case 0: 
                    ctx.fillStyle = "#000000";
                    break;
                case 1:
                case 2:
                case 3:
                case 4:
                case 5:
                case 6:
                case 7:
                    ctx.fillStyle = colors[bytes[i]];
                    break;
                case -1:
                    ctx.fillStyle = "#FFFFFF";
                    break;
            }
            ctx.fillRect(x * 25, y * 25, 25, 25);
        }
    }, 10);
})