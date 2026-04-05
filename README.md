# Darevault CLI

- An api version of Darevault that lets _anyone_ add to a shared database with a username and dare and provides enpoints to access them randomly or by their id.
- _Note - I will not be taking any responsibility for any dares tht have been added to this database as it is publicly accesible and does not have filtering of any kind. User discretion advised_

## API Documentation

### API Health check - GET

### All platforms

- Returns whether the api is running and ready to accept requests
  <br>
  ` curl http://13.49.74.9:3000/health`

### Adding Dares - POST

- Adds a dare along with the username of the person who added it into the database.

### MacOs/ Linux

`curl -X POST http://13.49.74.9:3000/add -H "Content-Type: application/json" -d '{"dare":"creative dare idea","created_by":"your username"}'`

### Windows (Powershell)

`curl.exe -X POST http://13.49.74.9:3000/add -H "Content-Type: application/json" -d '{"dare":"creative dare idea","created_by":"your username"}'`
<br>
<small>Apparently normal curl doesnt work because it invokes a webrequest that doesnt support flags </small>

## Random Dare - GET

- Return a random dare that is present in the database

### All Platforms

`curl http://13.49.74.9:3000/random`

## Find Dare - GET

- Returns the dare corresponding dare provided by the user

### All platforms

`curl http://13.49.74.9:3000/find/{id}`
<br>

<p>
where {id} is an integer
eg - "curl http://13.49.74.9:3000/find/3"</p>
