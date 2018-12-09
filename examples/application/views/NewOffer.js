console.log("NewOffer.js");

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

function output()
{
    for (var i = 0; i < data.length; i++)
    {
        console.log(data[i].modelname);
    }
}

function http_send()
{
    var http_post = new XMLHttpRequest();
    var url = "http://localhost:8000/json";
    http_post.open("POST", url, true);
    http_post.setRequestHeader("Content-Type", "application/json");

    var json_data = JSON.stringify(data);

    console.log(json_data);
    http_post.send(json_data);
}

function MakeNewOffer()
{
    var modelname = document.getElementById("modelname").value;
    console.log(modelname);
    var engine = document.getElementById("engine").value;
    console.log(engine);
    var mileage = document.getElementById("mileage").value;
    console.log(mileage);
    var price = document.getElementById("price").value;
    console.log(price);

    var num_mileage = Number(mileage);
    var num_price = Number(price);

    for(var i = 0; i < data.length; i++)
    {
        console.log("iteration");
        if(data[i].modelname == "null")
        {
            data[i].modelname = modelname;
            data[i].engine = engine;
            data[i].mileage = num_mileage;
            data[i].price = num_price;
            break;
        }
    }

    output();
    http_send();
}