import * as wasm from "wasm-alpha-rust";

let board = new wasm.BoardSingleton();

let str = board.get_board_string();

let div = document.getElementById( 'root' );

div.insertAdjacentHTML( 'beforeend', str );

const form = document.getElementById('fen-form');
form.addEventListener('submit', event => {
    event.preventDefault();

    let input = document.getElementById("fen")

    if(input.value.match(/\s*([rnbqkpRNBQKP1-8]+\/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*/)) {
        setFen(input.value);
        input.value = "";
    } else {
        input.value = "invalid FEN string"
    }
});

function setFen(fen) {
    board.set_fen(fen);

    let str = board.get_board_string();
    let div = document.getElementById( 'root' );

    div.innerHTML = str;
}