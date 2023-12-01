document.body.addEventListener("notify", (evt) => {
  Toastify({
    text: evt.detail.value,
    gravity: "bottom",
    position: "right",
  }).showToast();
});
