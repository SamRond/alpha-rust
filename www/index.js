import * as wasm from "wasm-alpha-rust";

let board = new wasm.BoardSingleton();

let str = board.get_board_string();

let div = document.getElementById( 'root' );

div.insertAdjacentHTML( 'beforeend', str );

const form = document.getElementById('fen-form');
form.addEventListener('submit', event => {
    event.preventDefault();

    let input = document.getElementById("fen")
    setFen(input.value);

    input.value = "";
});

function setFen(fen) {
    board.set_fen(fen);

    let str = board.get_board_string();
    let div = document.getElementById( 'root' );

    div.innerHTML = str;
}