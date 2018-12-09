console.log("Details.js");

function detailsinitializer()
{
    var index = document.cookie;
    console.log("initializing data for object number: " + index);

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

    document.getElementById("modelname").innerHTML=data[index].modelname;
    document.getElementById("engine").innerHTML="Двигатель " + data[index].engine;
    document.getElementById("mileage").innerHTML="Пробег: " + data[index].mileage + "км";
    document.getElementById("price").innerHTML="Цена: " + data[index].price + "$";
}

detailsinitializer();