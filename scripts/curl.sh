
## LIST

# Get All lists
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists --verbose

# Get list by id
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists/65df3d5934761793df5fbe46 --verbose

# Create list
curl -X POST \
-H "Content-Type: application/json" \
-d @insert_list.json \
http://localhost:8000/lists --verbose

# Edit list
curl -X PUT \
-H "Content-Type: application/json" \
-d @update_list.json \
http://localhost:8000/lists/65f4421a7157d5922f45d5d3 --verbose

# Delete List
curl -X DELETE -H "Content-Type: application/json"  http://localhost:8000/lists/65df3d5934761793df5fbe46 --verbose

ITEM

# Create item by id
curl -X POST \
-H "Content-Type: application/json" \
-d @insert_item.json \
http://localhost:8000/lists/65df3d5934761793df5fbe46/items --verbose

# Get item by id
curl -X "GET" \
-H "Content-Type: application/json"  \
http://localhost:8000/lists/65df3d5934761793df5fbe46/items/65e66a627b947912c00a34cf --verbose

# Edit item by id 
curl -X "PUT" \
-H "Content-Type: application/json"  \
-d @update_item.json \
http://localhost:8000/lists/65df3d5934761793df5fbe46/items/65df3e7134761793df5fbe4d --verbose

# Delete item by id
curl -X "DELETE" \
-H "Content-Type: application/json" \
http://localhost:8000/lists/65df3d5934761793df5fbe46/items/65e66a597b947912c00a34cd --verbose

curl -X POST -H "Content-Type: application/json" -d @insert_item.json http://localhost:8000/lists/65f4421a7157d5922f45d5d3/items --verbose
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists/65f4421a7157d5922f45d5d3/items --verbose