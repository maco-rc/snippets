#!/usr/bin/env bash
rename() {
    mv Solution/Class1.cs Solution/Solution.cs
    mv Solution.Tests/UnitTest1.cs Solution.Tests/SolutionTest.cs
}

create_proj_dotnet() {
    dotnet new sln -o "$1"

    cd "$1" || return

    dotnet new classlib -o Solution

    dotnet sln add ./Solution/Solution.csproj

    case $2 in
    true)
        dotnet new xunit -o Solution.Tests

        dotnet sln add ./Solution.Tests/Solution.Tests.csproj

        dotnet add ./Solution.Tests/Solution.Tests.csproj reference ./Solution/Solution.csproj

        rename
        ;;
    esac
}

create_proj_rust() {
    case $2 in
    true) cargo new --lib "$1"; cd "$folder/src" || exit; touch 'solution.rs' ;;
    false) cargo new --bin "$1" ;;
    esac

}

main() {
    if [[ $2 -eq 1 ]]; then
        create_proj_rust "$folder" $teste
    fi

    if [[ $2 -eq 2 ]]; then
        create_proj_dotnet "$folder" $teste
    fi
}

teste=false
while getopts rdtf: flag; do
    case "${flag}" in
    r) lang=1 ;;
    d) lang=2 ;;
    t) teste=true ;;
    f) folder=${OPTARG} ;;
    *) echo 'not a flag silly' ;;
    esac
done

if [ -d "$folder" ]; then
    read -r -p "Project folder already exist, wanna remade it? (y/n) " opt

    if [[ "$opt" == "y" ]]; then
        rm -rf "$folder"
        main
    else
        exit 1
    fi
fi

main "$folder" "$lang" "$teste"
