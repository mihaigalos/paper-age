@default:
    just --list --unsorted

tool := "paper-age"
docker_user_repo := "mihaigalos"
docker_image_dockerhub := docker_user_repo + "/" + tool + ":latest"

build:
    sudo docker build --network=host -t {{ docker_image_dockerhub }} .

push:
    sudo docker push {{ docker_image_dockerhub }}

generate_pdf input_file identities_file="~/git/secrets/identities":
    #!/bin/bash
    export identities="$(cat {{ identities_file }} | grep Recipient | sed -e 's/ //g' | cut -d':' -f2 | sed -e 's/^age\(.*\)/ -r age\1/g'  | tr -d '\n')"
    cat {{ input_file }} | rage $(echo $identities) -e -a > input_file

    docker run --rm -it -v $(pwd):/src -e IDENTITIES="$identities" {{ docker_image_dockerhub }} sh -c 'cat input_file | paper-age --force --no-footer --title=YubikeyIds --age-input --identities="$IDENTITIES"'
