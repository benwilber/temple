@test "env" {
    FOO=bar run $TEMPLE --env templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    FOO=bar run $TEMPLE -E templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "json-stdin" {
    run $TEMPLE --format=json templates/simple.txt < contexts/simple.json
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE --format=json templates/simple.txt < contexts/simple.json
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE -F json templates/simple.txt < contexts/simple.json
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "yaml-stdin" {
    run $TEMPLE --format=yaml templates/simple.txt < contexts/simple.yml
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE --context=- --format=yaml templates/simple.txt < contexts/simple.yml
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE -F yaml templates/simple.txt < contexts/simple.yml
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "json-file" {
    run $TEMPLE --context=contexts/simple.json templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE -c contexts/simple.json templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "yaml-file" {
    run $TEMPLE --context=contexts/simple.yml templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]

    run $TEMPLE -c contexts/simple.yml templates/simple.txt
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "json-empty" {
    run $TEMPLE --format=json templates/simple.txt <<< "{}"
    [ $status -eq 0 ]
    [[ "$output" = "" ]]
}

@test "yaml-empty" {
    run $TEMPLE --format=yaml templates/simple.txt <<< "-"
    [ $status -eq 0 ]
    [ "$output" = "" ]
}

@test "invalid-empty" {
    run $TEMPLE --context=contexts/empty.txt templates/simple.txt
    [ $status -eq 64 ]
}

@test "invalid-json-malformed" {
    run $TEMPLE --context=contexts/invalid_malformed.json templates/simple.txt
    [ $status -eq 65 ]
}

@test "invalid-yaml-malformed" {
    run $TEMPLE --context=contexts/invalid_malformed.yml templates/simple.txt
    [ $status -eq 65 ]
}

@test "extends" {
    run $TEMPLE --context=contexts/simple.json --templates=templates templates/extends.txt
    [ $status -eq 0 ]
    [ "$output" = "EXTENDS: bar" ]

    run $TEMPLE -c contexts/simple.json -t templates templates/extends.txt
    [ $status -eq 0 ]
    [ "$output" = "EXTENDS: bar" ]
}

@test "include" {
    run $TEMPLE --context=contexts/simple.json --templates=templates templates/include.txt
    [ $status -eq 0 ]
    [ "$output" = "INCLUDE: bar" ]

    run $TEMPLE -c contexts/simple.json -t templates templates/include.txt
    [ $status -eq 0 ]
    [ "$output" = "INCLUDE: bar" ]
}

@test "auto-escape" {
    run $TEMPLE --context=contexts/auto_escape.json --templates=templates templates/auto_escape.html
    [ $status -eq 0 ]
    [ "$output" = "&lt;script&gt;bar&lt;&#x2f;script&gt;" ]
}

@test "no-auto-escape" {
    run $TEMPLE --no-auto-escape --context=contexts/auto_escape.json --templates=templates templates/auto_escape.html
    [ $status -eq 0 ]
    [ "$output" = "<script>bar</script>" ]

    run $TEMPLE -n --context=contexts/auto_escape.json --templates=templates templates/auto_escape.html
    [ $status -eq 0 ]
    [ "$output" = "<script>bar</script>" ]
}

@test "options-from-env-templates" {
    TEMPLE_TEMPLATES=templates run $TEMPLE --context=contexts/simple.json templates/include.txt
    [ $status -eq 0 ]
    [ "$output" = "INCLUDE: bar" ]
}

@test "options-from-env-context-format" {
    TEMPLE_CONTEXT_FORMAT=json run $TEMPLE templates/simple.txt <<< '{"FOO": "bar"}'
    [ $status -eq 0 ]
    [ "$output" = "bar" ]
}

@test "output-file" {
    run $TEMPLE --output=outputs/output.txt --format=json templates/simple.txt <<< '{"FOO": "bar"}'
    [ $status -eq 0 ]
    [ "$(< outputs/output.txt)" = "bar" ]
}

@test "output-file-exists" {
    touch outputs/output.txt
    run $TEMPLE --output=outputs/output.txt --format=json templates/simple.txt <<< '{"FOO": "bar"}'
    [ $status -eq 73 ]
}
