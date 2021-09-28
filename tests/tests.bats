@test "env" {
    FOO=bar run $TEMPLE --env templates/simple.txt
    [[ $status == 0 ]]
    [[ $output == "bar" ]]    
}

@test "json-stdin" {
    run $TEMPLE --format=json templates/simple.txt < contexts/simple.json
    [[ $status == 0 ]]
    [[ $output == "bar" ]]    
}

@test "yaml-stdin" {
    run $TEMPLE --format=yaml templates/simple.txt < contexts/simple.yml
    [[ $status == 0 ]]
    [[ $output == "bar" ]]    
}

@test "kv-stdin" {
    run $TEMPLE --format=kv templates/simple.txt < contexts/simple_kv.txt
    [[ $status == 0 ]]
    [[ $output == "bar" ]]

    run $TEMPLE --format=kv templates/simple.txt <<< "FOO = bar"
    [[ $status == 0 ]]
    [[ $output == "bar" ]]
}

@test "json-file" {
    run $TEMPLE --context=contexts/simple.json templates/simple.txt
    [[ $status == 0 ]]
    [[ $output == "bar" ]]
}

@test "yaml-file" {
    run $TEMPLE --context=contexts/simple.yml templates/simple.txt
    [[ $status == 0 ]]
    [[ $output == "bar" ]]
}

@test "kv-file" {
    run $TEMPLE --context=contexts/simple_kv.txt --format=kv templates/simple.txt
    [[ $status == 0 ]]
    [[ $output == "bar" ]]
}

@test "json-empty" {
    run $TEMPLE --format=json templates/simple.txt <<< "{}"
    [[ $status == 0 ]]
    [[ $output == "" ]]
}

@test "yaml-empty" {
    run $TEMPLE --format=yaml templates/simple.txt <<< "-"
    [[ $status == 0 ]]
    [[ $output == "" ]]
}

@test "kv-empty" {
    run $TEMPLE --format=kv --context=contexts/invalid_malformed_kv.txt templates/simple.txt
    [[ $status == 0 ]]
    [[ $output == "" ]]
}

@test "invalid-empty" {
    run $TEMPLE --context=contexts/empty.txt templates/simple.txt
    [[ $status == 64 ]]
}

@test "invalid-json-malformed" {
    run $TEMPLE --context=contexts/invalid_malformed.json templates/simple.txt
    [[ $status == 65 ]]
}

@test "invalid-yaml-malformed" {
    run $TEMPLE --context=contexts/invalid_malformed.yaml templates/simple.txt
    [[ $status == 65 ]]
}

@test "extends" {
    run $TEMPLE --context=contexts/simple.json --templates=templates templates/extends.txt
    [[ $status == 0 ]]
    [[ $output == "BASE: bar" ]]
}

@test "include" {
    run $TEMPLE --context=contexts/simple.json --templates=templates templates/include.txt
    [[ $status == 0 ]]
    [[ $output == "INCLUDE: bar" ]]
}

@test "autoescape" {
    run $TEMPLE --context=contexts/autoescape.json --templates=templates templates/autoescape.html
    [[ $status == 0 ]]
    [[ $output == "&lt;script&gt;bar&lt;/script&gt;" ]]
}

@test "no-autoescape" {
    run $TEMPLE --no-autoescape --context=contexts/autoescape.json --templates=templates templates/autoescape.html
    [[ $status == 0 ]]
    [[ $output == "<script>bar</script>" ]]
}
