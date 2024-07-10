import * as wasm from "wasm-client";

const mForm = document.getElementById('form');

mForm.addEventListener('submit', (e) => {
    e.preventDefault();

    const name = document.getElementById('name').value;
    const desc = document.querySelector('#description').value;

    wasm.add_course(name, desc).then((json) => {
        alert("添加成功！");

        window.location.reload();
    })
})
