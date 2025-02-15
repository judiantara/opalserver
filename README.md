# Opal web server

Simple web server that able to handle form submission and return back submitted data rendered using handlebar template.

This program use by my kid when learn HTML form authoring. 

Hardcoded to listen on port 8080

For url `http://localhost:8080/upload/info` response rendered using template from `templates/info.html`, any submitted files will stored into `/assets/upload` directory.

Example html form

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Belajar HTML Form</title>
</head>
<body>
    <h1>Masukkan Identitas Diri Anda</h1>
    <hr style="height:5px;border-width:0;color:black;background-color:black":">
    <form name="info" id="info" action="http://localhost:8080/upload/info" enctype="multipart/form-data" method="post">
        <br>
        <label for="nis">NIS</label><br>
        <input type="text" id="nis" name="nis"><br>
        <label for="fname">Nama:</label><br>
        <input type="text" id="fname" name="fname"><br>
        <label for="sekolah">Sekolah:</label><br>
        <input type="text" id="sekolah" name="sekolah"><br>
        <label for="kelas">Kelas:</label><br>
        <input type="text" id="kelas" name="kelas"><br>
        <label for="tl">Tempat Lahir:</label><br>
        <input type="text" id="tl" name="tl"><br>
        <label for="dob">Tanggal Lahir</label><br>
        <input type="date" id="dob" name="dob"><br>
        <input type="hidden" name="id" value="fname">
        <hr>
        <label for="myfile">Ubah Foto:</label>
        <input type="file" id="myfile" name="myfile"><br><br>
        <hr>
        <input type="reset">
        <input type="submit">
    </form>
</body>
</html>
```

Example handlebar template to render response

```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Belajar HTML Form</title>
    </head>
<body>
    <h1>Identitas Diri anda</h1>
    <div id="detail">
        <ul>
            <li>NIS: {{ nis }}</li>
            <li>Nama: {{ fname }}</li>
            <li>Sekolah: {{ sekolah }}</li>
            <li>Kelas: {{ kelas }}</li>
            <li>Tempat Lahir: {{ tl }}</li>
            <li>Tanggal lahir: {{ dob }}</li>
        </ul>
    </div>
    <div id="photo">
        <img src="/assets/upload/{{ myfile }}">
    </div>
</div>
</body>
</html>
```
