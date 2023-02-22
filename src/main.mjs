import init, {WasmGame} from "./tetris.js"

console.log("toll")

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
        }
    }, false);

    let colors = []
    for (let i = 0; i < 10; i++) {
        colors.push(Math.floor(Math.random() * 16777215).toString(16));
    }
    console.log(colors)
    window.setInterval(() => {
        game.tick();
        let bytes = game.draw();
        for (let i = 0; i < bytes.length; i++) {
            // draw the cells
            let x = i % 10;
            let y = Math.floor(i / 10);
            ctx.fillStyle = "#" + colors[bytes[i]];
            if (bytes[i] == 0) {
                ctx.fillStyle = "#000000";
            }
            ctx.fillRect(x * 25, y * 25, 25, 25);
        }
    }, 10);
})