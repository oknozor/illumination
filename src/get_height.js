function get_line_number () {
    var lineHeight = document.body.style.lineHeight;
    var scrollHeight = document.body.scrollHeight;
    document.body.style.height = scrollHeight; // this is just for showing purposes
    return Math.floor( scrollHeight / lineHeight );
}