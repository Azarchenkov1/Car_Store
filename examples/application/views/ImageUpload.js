console.log("ImageUpload.js");

function upload()
{
    console.log("upload()");
    var photo = document.getElementById('input').files[0];

    var formdata = new FormData();
    formdata.append('name', 'name');

    var sender = new XMLHttpRequest();
    sender.open('POST', 'http://localhost:8000/photo');
    
    sender.send(formdata);
    console.log("photo sended!");
}