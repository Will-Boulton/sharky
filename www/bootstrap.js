fetch("shark")
    .then(r => r.text())
    .then(r => {
        document.getElementById("footer").innerHTML = `<i>${r} shark fans</i>`;
    });