document.addEventListener('wheel', function(e) {
    document.getElementById('hero').scrollBy(0, e.deltaY);
});
