
## LIST

# Get All lists
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists --verbose

# Get list by id
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists/65f4421a7157d5922f45d5d3 --verbose

# Create list
curl -X POST \
-H "Content-Type: application/json" \
-d @insert_list.json \
http://localhost:8000/lists --verbose

ITEM

curl -X POST -H "Content-Type: application/json" -d @insert_item.json http://localhost:8000/lists/65f4421a7157d5922f45d5d3/items --verbose
curl -X "GET" -H "Content-Type: application/json"  http://localhost:8000/lists/65f4421a7157d5922f45d5d3/items --verbose