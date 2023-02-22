curl -L "http://localhost:3000/run" -H "Content-Type: application/json" -d "{
    \"id\": \"1\",
    \"language\": \"rust\",
    \"source_code\": \"fn main() {\n\tprintln!(\\\"Hello World\\\");\n}\",
    \"timeout\": 5,
    \"sample_testcases\": [
        \"123\"
    ]
}"
