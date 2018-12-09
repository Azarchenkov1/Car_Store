console.log("Store.js");

function init()
{
    console.log("init()");
    document.addEventListener('click', function(e){
        e = e || window.event;
        var target = e.target || e.srcElement,
        index = target.id;
        console.log("id of clicked element");
        console.log(index);
        document.cookie = index;
    }, false);
}

init();

var http_get_json = new XMLHttpRequest();

http_get_json.open('GET', 'model.json', false);

http_get_json.send();

if(http_get_json.status != 200)
{
    alert (http_get_json.status + ': ' + http_get_json.statusText);
} else {
    console.log("data succesfuly received!");
}

var data = JSON.parse(http_get_json.responseText);

function addElement(data, i) {
    var End = document.getElementById("End");
    var div = document.createElement("div");
    div.innerHTML = '<a href="http://localhost:8000/details.html" style="color: black; text-decoration: none;"><div id="' + i + '" style="background-color: white; margin: 10px; border-style: solid; border-width: 2px; border-color: black"><p id="' + i + '">' + data[i].modelname + '</p></div></a>';

    var parentDiv = End.parentNode;

    parentDiv.insertBefore(div, End);
  }

for(var i = 0; i < data.length; i++)
{
    if(data[i].modelname != "null")
    {
        addElement(data, i);
    }
}